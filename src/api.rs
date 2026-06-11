use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use chrono::Local;
use uuid::Uuid;

use crate::models::{
    TaskStatusResponse, ValidationReport,
};
use crate::sandbox;
use crate::AppState;

// ─────────────────────────────────────────────────────────────────────────────
// POST /api/v1/validation/run
// ─────────────────────────────────────────────────────────────────────────────

/// Recibe un backup o script, crea una tarea de validación y lanza el sandbox
/// de forma asíncrona. Responde inmediatamente con un task_id.
#[axum::debug_handler]
pub async fn run_validation(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let task_id = Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut engine = String::new();
    let mut database_name: Option<String> = None;
    let mut temp_file_path = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "engine" {
            if let Ok(text) = field.text().await {
                engine = text;
            }
        } else if name == "database_name" {
            if let Ok(text) = field.text().await {
                if !text.is_empty() {
                    database_name = Some(text);
                }
            }
        } else if name == "file" {
            let file_name = field.file_name().unwrap_or("backup.sql").to_string();
            let safe_file_name = file_name.replace(|c: char| !c.is_ascii_alphanumeric() && c != '.', "_");
            
            let temp_dir = std::env::temp_dir();
            let file_path = temp_dir.join(format!("{}_{}", task_id, safe_file_name));
            
            if let Ok(data) = field.bytes().await {
                if let Err(e) = tokio::fs::write(&file_path, &data).await {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("Error al guardar archivo temporal: {}", e)
                        })),
                    );
                }
                temp_file_path = file_path.to_string_lossy().to_string();
            }
        }
    }

    if engine.is_empty() || temp_file_path.is_empty() {
        if !temp_file_path.is_empty() {
            let _ = tokio::fs::remove_file(&temp_file_path).await;
        }
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Los campos 'engine' y 'file' son requeridos."
            })),
        );
    }

    // Registrar la tarea en la base de datos con estado "queued"
    let db_result = {
        let db = state.db.lock().unwrap();
        db.execute(
            "INSERT INTO validation_tasks (task_id, status, backup_path, engine, database_name, created_at)
             VALUES (?1, 'queued', ?2, ?3, ?4, ?5)",
            (
                &task_id,
                &temp_file_path,
                &engine,
                &database_name,
                &now,
            ),
        )
    };

    if let Err(e) = db_result {
        let _ = tokio::fs::remove_file(&temp_file_path).await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Error al registrar la tarea: {}", e)
            })),
        );
    }

    // Lanzar el proceso de validación de forma asíncrona (no bloqueante)
    let state_clone = Arc::clone(&state);
    let tid = task_id.clone();

    tokio::spawn(async move {
        sandbox::run_validation(state_clone, tid, temp_file_path, engine, database_name, true).await;
    });

    // Responder inmediatamente al cliente
    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({
            "task_id": task_id,
            "status": "queued",
            "message": "Validación iniciada en segundo plano. Consulte el estado con GET /api/v1/validation/{task_id}/report"
        })),
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /api/v1/validation/:id/report
// ─────────────────────────────────────────────────────────────────────────────

/// Devuelve el estado actual de la tarea de validación.
/// Si la tarea ya finalizó, incluye el reporte completo con tablas validadas,
/// advertencias y errores críticos.
pub async fn get_validation_report(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();

    let result = db.query_row(
        "SELECT task_id, status, progress, report_json FROM validation_tasks WHERE task_id = ?1",
        [&task_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        },
    );

    match result {
        Ok((id, status, progress, report_json)) => {
            let report: Option<ValidationReport> = report_json
                .and_then(|json| serde_json::from_str(&json).ok());

            let response = TaskStatusResponse {
                task_id: id,
                status,
                progress,
                report,
            };

            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": format!("No se encontró la tarea con ID: {}", task_id)
            })),
        ),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /api/v1/validation/tasks (listado de todas las tareas)
// ─────────────────────────────────────────────────────────────────────────────

/// Lista todas las tareas de validación registradas.
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();

    let mut stmt = db.prepare(
        "SELECT task_id, status, progress, backup_path, engine, database_name, created_at, finished_at 
         FROM validation_tasks ORDER BY created_at DESC"
    ).unwrap();

    let iter = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "task_id": row.get::<_, String>(0)?,
            "status": row.get::<_, String>(1)?,
            "progress": row.get::<_, Option<String>>(2)?,
            "backup_path": row.get::<_, String>(3)?,
            "engine": row.get::<_, String>(4)?,
            "database_name": row.get::<_, Option<String>>(5)?,
            "created_at": row.get::<_, String>(6)?,
            "finished_at": row.get::<_, Option<String>>(7)?,
        }))
    }).unwrap();

    let tasks: Vec<serde_json::Value> = iter.filter_map(|t| t.ok()).collect();

    (StatusCode::OK, Json(serde_json::json!({ "tasks": tasks })))
}

// ─────────────────────────────────────────────────────────────────────────────
// GET /health (verificación de salud del servidor)
// ─────────────────────────────────────────────────────────────────────────────

pub async fn health_check() -> impl IntoResponse {
    let docker_available = crate::docker::check_docker();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "ok",
            "service": "SafeBridge API",
            "version": "1.0.0",
            "docker_available": docker_available,
            "timestamp": Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        })),
    )
}
