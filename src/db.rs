use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub fn init_db(data_dir: &PathBuf) -> Result<Connection> {
    if !data_dir.exists() {
        fs::create_dir_all(data_dir).expect("Failed to create data directory");
    }
    let db_path = data_dir.join("safebridge.db");
    let conn = Connection::open(db_path)?;

    // Habilitar Foreign Keys
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    // Crear tabla connections
    conn.execute(
        "CREATE TABLE IF NOT EXISTS connections (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            engine TEXT NOT NULL,
            host TEXT NOT NULL,
            port INTEGER NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            database_name TEXT NOT NULL,
            backup_path TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Crear tabla backup_logs
    conn.execute(
        "CREATE TABLE IF NOT EXISTS backup_logs (
            id TEXT PRIMARY KEY,
            connection_id TEXT,
            connection_name TEXT NOT NULL,
            engine TEXT NOT NULL,
            started_at DATETIME NOT NULL,
            finished_at DATETIME NOT NULL,
            duration_seconds INTEGER NOT NULL,
            file_path TEXT NOT NULL,
            file_size_bytes INTEGER NOT NULL,
            status TEXT NOT NULL,
            error_message TEXT,
            restore_verified BOOLEAN NOT NULL,
            full_logs TEXT,
            FOREIGN KEY(connection_id) REFERENCES connections(id) ON DELETE SET NULL
        )",
        [],
    )?;

    // Crear tabla validation_tasks (nueva para la API)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS validation_tasks (
            task_id TEXT PRIMARY KEY,
            status TEXT NOT NULL DEFAULT 'queued',
            progress TEXT,
            backup_path TEXT NOT NULL,
            engine TEXT NOT NULL,
            database_name TEXT,
            created_at DATETIME NOT NULL,
            finished_at DATETIME,
            report_json TEXT
        )",
        [],
    )?;

    // Migración simple: intentar agregar la columna si la tabla ya existía sin ella
    let _ = conn.execute("ALTER TABLE backup_logs ADD COLUMN full_logs TEXT", []);

    Ok(conn)
}
