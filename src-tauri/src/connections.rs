use crate::AppState;
use crate::models::ConnectionInfo;
use crate::crypto;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn create_connection(
    state: State<'_, AppState>,
    conn: ConnectionInfo,
) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    let password = conn.password.unwrap_or_default();
    let encrypted_password = crypto::encrypt_password(&password)?;

    let db = state.db.lock().unwrap();
    db.execute(
        "INSERT INTO connections (id, name, engine, host, port, username, password, database_name, backup_path) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &id,
            &conn.name,
            &conn.engine,
            &conn.host,
            conn.port,
            &conn.username,
            &encrypted_password,
            &conn.database_name,
            &conn.backup_path,
        ),
    ).map_err(|e| format!("Database error: {}", e))?;

    Ok(id)
}

#[tauri::command]
pub fn list_connections(state: State<'_, AppState>) -> Result<Vec<ConnectionInfo>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare(
        "SELECT id, name, engine, host, port, username, database_name, backup_path, created_at FROM connections ORDER BY created_at DESC"
    ).map_err(|e| format!("Database error: {}", e))?;

    let iter = stmt.query_map([], |row| {
        Ok(ConnectionInfo {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            engine: row.get(2)?,
            host: row.get(3)?,
            port: row.get(4)?,
            username: row.get(5)?,
            password: None, // Nunca enviar la contraseña de vuelta al frontend
            database_name: row.get(6)?,
            backup_path: row.get(7)?,
            created_at: Some(row.get(8)?),
        })
    }).map_err(|e| format!("Database error: {}", e))?;

    let mut connections = Vec::new();
    for conn in iter {
        if let Ok(c) = conn {
            connections.push(c);
        }
    }

    Ok(connections)
}

#[tauri::command]
pub fn update_connection(
    state: State<'_, AppState>,
    id: String,
    conn: ConnectionInfo,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    
    if let Some(password) = conn.password {
        if !password.is_empty() {
            let encrypted_password = crypto::encrypt_password(&password)?;
            db.execute(
                "UPDATE connections SET name=?1, engine=?2, host=?3, port=?4, username=?5, password=?6, database_name=?7, backup_path=?8 WHERE id=?9",
                (
                    &conn.name,
                    &conn.engine,
                    &conn.host,
                    conn.port,
                    &conn.username,
                    &encrypted_password,
                    &conn.database_name,
                    &conn.backup_path,
                    &id,
                ),
            ).map_err(|e| format!("Database error: {}", e))?;
            return Ok(());
        }
    }
    
    // Si no se envía contraseña, no actualizarla
    db.execute(
        "UPDATE connections SET name=?1, engine=?2, host=?3, port=?4, username=?5, database_name=?6, backup_path=?7 WHERE id=?8",
        (
            &conn.name,
            &conn.engine,
            &conn.host,
            conn.port,
            &conn.username,
            &conn.database_name,
            &conn.backup_path,
            &id,
        ),
    ).map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_connection(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.execute("DELETE FROM connections WHERE id=?1", [&id])
        .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn test_connection(host: String, port: u16) -> Result<bool, String> {
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Duration;

    let addr_str = format!("{}:{}", host, port);
    let mut addrs = addr_str.to_socket_addrs().map_err(|e| format!("Invalid address: {}", e))?;
    
    if let Some(addr) = addrs.next() {
        match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    } else {
        Err("Could not resolve address".to_string())
    }
}
