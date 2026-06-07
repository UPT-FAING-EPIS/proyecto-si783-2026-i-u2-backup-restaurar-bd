use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
