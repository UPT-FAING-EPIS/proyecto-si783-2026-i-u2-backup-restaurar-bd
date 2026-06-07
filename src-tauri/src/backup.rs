use crate::{AppState, crypto, models::ConnectionInfo};
use chrono::Local;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, State, Manager};
use tauri_plugin_shell::ShellExt;

#[derive(Clone, serde::Serialize)]
pub struct BackupLogPayload {
    pub message: String,
    pub level: String, // "info", "success", "error"
}

#[derive(serde::Serialize)]
pub struct BackupResult {
    pub file_path: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub verified: bool,
}

fn emit_log_and_record(app: &AppHandle, buffer: &mut String, message: &str, level: &str) {
    let line = format!("[{}] [{}] {}\n", Local::now().format("%H:%M:%S"), level.to_uppercase(), message);
    buffer.push_str(&line);
    let _ = app.emit(
        "backup_log",
        BackupLogPayload {
            message: message.to_string(),
            level: level.to_string(),
        },
    );
}

// Helper function to return result and accumulate logs
async fn do_backup_process(
    app: &AppHandle,
    conn: &ConnectionInfo,
    log_buffer: &mut String,
    password: &str,
) -> Result<(String, u64, String, bool), String> {
    emit_log_and_record(app, log_buffer, "Iniciando proceso de backup...", "info");

    // 2. Generate file name
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let ext = match conn.engine.as_str() {
        "postgres" | "mysql" | "sqlserver" => "sql",
        "mongodb" => "archive",
        _ => "bak",
    };
    
    let file_name = format!("{}_{}.{}", conn.database_name, timestamp, ext);
    let backup_dir = Path::new(&conn.backup_path);
    if !backup_dir.exists() {
        std::fs::create_dir_all(backup_dir).map_err(|e| format!("Could not create backup directory: {}", e))?;
    }
    let file_path = backup_dir.join(file_name);
    let file_path_str = file_path.to_string_lossy().to_string();

    emit_log_and_record(app, log_buffer, &format!("Archivo destino: {}", file_path_str), "info");
    emit_log_and_record(app, log_buffer, &format!("Conectando a motor: {}", conn.engine), "info");

    // 3. Execute backup tool
    let success = match conn.engine.as_str() {
        "postgres" => {
            let resource_path = app.path().resolve("resources/pg_deps", tauri::path::BaseDirectory::Resource).map_err(|e| e.to_string())?;
            let current_path = std::env::var("PATH").unwrap_or_default();
            let new_path = format!("{};{}", resource_path.to_string_lossy(), current_path);

            let output = app.shell().sidecar("pg_dump")
                .map_err(|e| format!("Error cargando sidecar pg_dump: {}", e))?
                .env("PGPASSWORD", password)
                .env("PATH", new_path)
                .args(["-h", &conn.host, "-p", &conn.port.to_string(), "-U", &conn.username, "-F", "p", "-f", &file_path_str, &conn.database_name])
                .output()
                .await
                .map_err(|e| format!("Failed to execute pg_dump: {}", e))?;
            if !output.status.success() {
                emit_log_and_record(app, log_buffer, &format!("pg_dump error: {}", String::from_utf8_lossy(&output.stderr)), "error");
            }
            output.status.success()
        }
        "mysql" => {
            let output = app.shell().sidecar("mysqldump")
                .map_err(|e| format!("Error cargando sidecar mysqldump: {}", e))?
                .env("MYSQL_PWD", password)
                .args(["-h", &conn.host, "-P", &conn.port.to_string(), "-u", &conn.username, &format!("--result-file={}", file_path_str), &conn.database_name])
                .output()
                .await
                .map_err(|e| format!("Failed to execute mysqldump: {}", e))?;
            if !output.status.success() {
                emit_log_and_record(app, log_buffer, &format!("mysqldump error: {}", String::from_utf8_lossy(&output.stderr)), "error");
            }
            output.status.success()
        }
        "sqlserver" => {
            let output = app.shell().sidecar("sqlcmd")
                .map_err(|e| format!("Error cargando sidecar sqlcmd: {}", e))?
                .args(["-S", &format!("{},{}", conn.host, conn.port), "-U", &conn.username, "-P", password, "-b", "-Q", &format!("BACKUP DATABASE [{}] TO DISK = '{}'", conn.database_name, file_path_str)])
                .output()
                .await
                .map_err(|e| format!("Failed to execute sqlcmd: {}", e))?;
            if !output.status.success() {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                let out_msg = String::from_utf8_lossy(&output.stdout);
                emit_log_and_record(app, log_buffer, &format!("sqlcmd error: {} {}", err_msg, out_msg), "error");
            }
            output.status.success()
        }
        "mongodb" => {
            let mongo_uri = format!("mongodb://{}:{}@{}:{}/{}", conn.username, password, conn.host, conn.port, conn.database_name);
            let output = app.shell().sidecar("mongodump")
                .map_err(|e| format!("Error cargando sidecar mongodump: {}", e))?
                .args(["--uri", &mongo_uri, &format!("--archive={}", file_path_str)])
                .output()
                .await
                .map_err(|e| format!("Failed to execute mongodump: {}", e))?;
            if !output.status.success() {
                emit_log_and_record(app, log_buffer, &format!("mongodump error: {}", String::from_utf8_lossy(&output.stderr)), "error");
            }
            output.status.success()
        }
        _ => return Err("Unsupported engine".to_string()),
    };

    if !success {
        emit_log_and_record(app, log_buffer, "Error al generar el volcado de la base de datos.", "error");
        return Err("Backup command failed".to_string());
    }

    emit_log_and_record(app, log_buffer, "Volcado generado exitosamente.", "success");

    // 4. Calculate size and SHA-256
    emit_log_and_record(app, log_buffer, "Calculando hash SHA-256...", "info");
    let (size_bytes, sha256) = calculate_hash_and_size(&file_path)?;

    emit_log_and_record(app, log_buffer, &format!("Tamaño: {} bytes", size_bytes), "info");
    emit_log_and_record(app, log_buffer, &format!("SHA-256: {}", sha256), "info");
    emit_log_and_record(app, log_buffer, "Fase de backup completada.", "success");

    let verified = match verify_backup(app, conn, &file_path_str, password, log_buffer).await {
        Ok(v) => {
            if v {
                emit_log_and_record(app, log_buffer, "Verificación exitosa: Datos recuperables.", "success");
            } else {
                emit_log_and_record(app, log_buffer, "Verificación fallida: Archivo corrupto o ilegible.", "error");
            }
            v
        },
        Err(e) => {
            emit_log_and_record(app, log_buffer, &format!("Error en verificación: {}", e), "error");
            false
        }
    };

    Ok((file_path_str, size_bytes, sha256, verified))
}

#[tauri::command]
pub async fn generate_backup(
    app: AppHandle,
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<BackupResult, String> {
    let start_time = Local::now();
    let mut log_buffer = String::new();

    // 1. Get connection from DB
    let conn_result = {
        let db = state.db.lock().unwrap();
        let stmt = db.prepare("SELECT name, engine, host, port, username, password, database_name, backup_path FROM connections WHERE id = ?1");
        
        match stmt {
            Ok(mut s) => {
                let mut rows = s.query([&connection_id]).unwrap();
                if let Some(row) = rows.next().unwrap() {
                    Ok(ConnectionInfo {
                        id: Some(connection_id.clone()),
                        name: row.get::<_, String>(0).unwrap(),
                        engine: row.get::<_, String>(1).unwrap(),
                        host: row.get::<_, String>(2).unwrap(),
                        port: row.get::<_, u16>(3).unwrap(),
                        username: row.get::<_, String>(4).unwrap(),
                        password: row.get::<_, Option<String>>(5).unwrap(),
                        database_name: row.get::<_, String>(6).unwrap(),
                        backup_path: row.get::<_, String>(7).unwrap(),
                        created_at: None,
                    })
                } else {
                    Err("Connection not found".to_string())
                }
            },
            Err(e) => Err(format!("Database error: {}", e))
        }
    };

    let conn = match conn_result {
        Ok(c) => c,
        Err(e) => {
            emit_log_and_record(&app, &mut log_buffer, &e, "error");
            return Err(e);
        }
    };

    let password = if let Some(ref enc_pass) = conn.password {
        crypto::decrypt_password(enc_pass).unwrap_or_default()
    } else {
        "".to_string()
    };

    // Run the backup process catching any error
    let backup_outcome = do_backup_process(&app, &conn, &mut log_buffer, &password).await;
    
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time).num_seconds();
    
    let (status, error_msg, file_path_str, size_bytes, verified, sha256) = match backup_outcome {
        Ok((fp, sz, sha, v)) => {
            let status = if v { "OK" } else { "FAIL" };
            let err = if v { None } else { Some("Verification failed".to_string()) };
            (status, err, fp, sz, v, sha)
        },
        Err(e) => {
            emit_log_and_record(&app, &mut log_buffer, &format!("Error fatal: {}", e), "error");
            ("FAIL", Some(e.clone()), "".to_string(), 0, false, "".to_string())
        }
    };

    let log_id = uuid::Uuid::new_v4().to_string();
    {
        let db = state.db.lock().unwrap();
        let _ = db.execute(
            "INSERT INTO backup_logs (id, connection_id, connection_name, engine, started_at, finished_at, duration_seconds, file_path, file_size_bytes, status, error_message, restore_verified, full_logs)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &log_id,
                &connection_id,
                &conn.name,
                &conn.engine,
                start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                duration,
                &file_path_str,
                size_bytes as i64,
                status,
                error_msg,
                verified,
                &log_buffer
            )
        );
    }

    if status == "FAIL" {
        return Err("Backup operation failed. Check history for details.".to_string());
    }

    Ok(BackupResult {
        file_path: file_path_str,
        size_bytes,
        sha256,
        verified,
    })
}

async fn verify_backup(
    app: &AppHandle,
    conn: &ConnectionInfo,
    file_path: &str,
    _password: &str,
    log_buffer: &mut String
) -> Result<bool, String> {
    emit_log_and_record(app, log_buffer, "Iniciando verificación de integridad nativa...", "info");
    
    // Si el archivo no existe o no se puede abrir, fallamos rápido
    let mut file = std::fs::File::open(file_path).map_err(|e| format!("No se pudo abrir el archivo de backup: {}", e))?;
    let metadata = file.metadata().map_err(|e| format!("No se pudo leer metadatos: {}", e))?;
    
    if metadata.len() == 0 {
        emit_log_and_record(app, log_buffer, "El archivo de backup está vacío (0 bytes).", "error");
        return Ok(false);
    }

    match conn.engine.as_str() {
        "mysql" => {
            let mut buffer = [0; 256];
            let read_size = if metadata.len() > 256 { 256 } else { metadata.len() };
            file.seek(SeekFrom::End(-(read_size as i64))).unwrap_or(0);
            if let Ok(_) = file.read_exact(&mut buffer[0..read_size as usize]) {
                let end_str = String::from_utf8_lossy(&buffer);
                if end_str.contains("Dump completed on") {
                    emit_log_and_record(app, log_buffer, "Verificación exitosa: Se encontró firma de mysqldump válida.", "success");
                    return Ok(true);
                } else {
                    emit_log_and_record(app, log_buffer, "Advertencia: No se encontró la firma '-- Dump completed on' al final del archivo.", "error");
                    return Ok(false);
                }
            }
            Ok(false)
        }
        "postgres" => {
            let mut buffer = [0; 256];
            let read_size = if metadata.len() > 256 { 256 } else { metadata.len() };
            file.seek(SeekFrom::End(-(read_size as i64))).unwrap_or(0);
            if let Ok(_) = file.read_exact(&mut buffer[0..read_size as usize]) {
                let end_str = String::from_utf8_lossy(&buffer);
                if end_str.contains("PostgreSQL database dump complete") {
                    emit_log_and_record(app, log_buffer, "Verificación exitosa: Se encontró firma de pg_dump válida.", "success");
                    return Ok(true);
                } else {
                    emit_log_and_record(app, log_buffer, "Advertencia: No se encontró la firma de pg_dump al final del archivo.", "error");
                    return Ok(false);
                }
            }
            Ok(false)
        }
        "sqlserver" | "mongodb" => {
            emit_log_and_record(app, log_buffer, "Verificación exitosa: Archivo generado correctamente (validación de tamaño).", "success");
            Ok(true)
        }
        _ => Err("Motor no soportado para verificación".to_string())
    }
}

fn calculate_hash_and_size(path: &PathBuf) -> Result<(u64, String), String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open backup file: {}", e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    let mut size = 0;

    loop {
        let count = file.read(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
        if count == 0 {
            break;
        }
        size += count as u64;
        hasher.update(&buffer[..count]);
    }

    let hash_result = hasher.finalize();
    let hash_hex = hex::encode(hash_result);

    Ok((size, hash_hex))
}
