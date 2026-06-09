use serde::{Deserialize, Serialize};

// ─── Conexiones de Base de Datos ───

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionInfo {
    pub id: Option<String>,
    pub name: String,
    pub engine: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database_name: String,
    pub backup_path: String,
    pub created_at: Option<String>,
}

// ─── Logs de Backup ───

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupLog {
    pub id: String,
    pub connection_id: Option<String>,
    pub connection_name: String,
    pub engine: String,
    pub started_at: String,
    pub finished_at: String,
    pub duration_seconds: i64,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub status: String,
    pub error_message: Option<String>,
    pub restore_verified: bool,
    pub full_logs: Option<String>,
}

// ─── Tareas de Validación (API) ───

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationTask {
    pub task_id: String,
    pub status: String,         // "queued", "processing", "completed", "failed"
    pub progress: Option<String>,
    pub backup_path: String,
    pub engine: String,
    pub database_name: Option<String>,
    pub created_at: String,
    pub finished_at: Option<String>,
    pub report: Option<ValidationReport>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationReport {
    pub integrity_valid: bool,
    pub execution_time_seconds: i64,
    pub tables_validated: i64,
    pub warnings: Vec<String>,
    pub critical_errors: Vec<String>,
    pub logs: Vec<String>,
}

// ─── Request / Response para la API ───

#[derive(Debug, Deserialize)]
pub struct ValidationRequest {
    pub backup_path: String,
    pub engine: String,
    pub database_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidationResponse {
    pub task_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TaskStatusResponse {
    pub task_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<ValidationReport>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}
