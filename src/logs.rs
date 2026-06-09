use crate::models::BackupLog;
use crate::AppState;
use std::sync::Arc;

/// Listar todos los logs de backup con filtros opcionales.
pub fn list_logs(
    state: &Arc<AppState>,
    engine: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<BackupLog>, String> {
    let db = state.db.lock().unwrap();

    let mut query = "SELECT id, connection_id, connection_name, engine, started_at, finished_at, duration_seconds, file_path, file_size_bytes, status, error_message, restore_verified, full_logs FROM backup_logs WHERE 1=1".to_string();

    let mut params: Vec<String> = Vec::new();

    if let Some(e) = engine {
        if e != "all" && !e.is_empty() {
            query.push_str(&format!(" AND engine = ?{}", params.len() + 1));
            params.push(e.to_string());
        }
    }

    if let Some(s) = status {
        if s != "all" && !s.is_empty() {
            query.push_str(&format!(" AND status = ?{}", params.len() + 1));
            params.push(s.to_string());
        }
    }

    query.push_str(" ORDER BY started_at DESC");

    let mut stmt = db.prepare(&query).map_err(|e| format!("Database error: {}", e))?;

    let iter = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
        Ok(BackupLog {
            id: row.get(0)?,
            connection_id: row.get(1)?,
            connection_name: row.get(2)?,
            engine: row.get(3)?,
            started_at: row.get(4)?,
            finished_at: row.get(5)?,
            duration_seconds: row.get(6)?,
            file_path: row.get(7)?,
            file_size_bytes: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            restore_verified: row.get(11)?,
            full_logs: row.get(12)?,
        })
    }).map_err(|e| format!("Database error: {}", e))?;

    let mut logs = Vec::new();
    for log in iter {
        if let Ok(l) = log {
            logs.push(l);
        }
    }

    Ok(logs)
}

/// Estadísticas del dashboard.
#[derive(serde::Serialize)]
pub struct DashboardStats {
    pub total_connections: i64,
    pub successful_backups: i64,
    pub failed_backups: i64,
    pub total_bytes_backed_up: i64,
    pub recent_activity: Vec<BackupLog>,
}

pub fn get_dashboard_stats(state: &Arc<AppState>) -> Result<DashboardStats, String> {
    let db = state.db.lock().unwrap();

    let total_connections: i64 = db.query_row("SELECT COUNT(*) FROM connections", [], |row| row.get(0)).unwrap_or(0);
    let successful_backups: i64 = db.query_row("SELECT COUNT(*) FROM backup_logs WHERE status = 'OK'", [], |row| row.get(0)).unwrap_or(0);
    let failed_backups: i64 = db.query_row("SELECT COUNT(*) FROM backup_logs WHERE status = 'FAIL'", [], |row| row.get(0)).unwrap_or(0);
    let total_bytes_backed_up: i64 = db.query_row("SELECT COALESCE(SUM(file_size_bytes), 0) FROM backup_logs WHERE status = 'OK'", [], |row| row.get(0)).unwrap_or(0);

    let mut stmt = db.prepare("SELECT id, connection_id, connection_name, engine, started_at, finished_at, duration_seconds, file_path, file_size_bytes, status, error_message, restore_verified, full_logs FROM backup_logs ORDER BY started_at DESC LIMIT 5").map_err(|e| format!("DB Error: {}", e))?;

    let iter = stmt.query_map([], |row| {
        Ok(BackupLog {
            id: row.get(0)?,
            connection_id: row.get(1)?,
            connection_name: row.get(2)?,
            engine: row.get(3)?,
            started_at: row.get(4)?,
            finished_at: row.get(5)?,
            duration_seconds: row.get(6)?,
            file_path: row.get(7)?,
            file_size_bytes: row.get(8)?,
            status: row.get(9)?,
            error_message: row.get(10)?,
            restore_verified: row.get(11)?,
            full_logs: row.get(12)?,
        })
    }).unwrap();

    let mut recent_activity = Vec::new();
    for log in iter {
        if let Ok(l) = log {
            recent_activity.push(l);
        }
    }

    Ok(DashboardStats {
        total_connections,
        successful_backups,
        failed_backups,
        total_bytes_backed_up,
        recent_activity,
    })
}

/// Limpiar todos los logs.
pub fn clear_logs(state: &Arc<AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.execute("DELETE FROM backup_logs", [])
        .map_err(|e| format!("Error al limpiar logs: {}", e))?;
    Ok(())
}
