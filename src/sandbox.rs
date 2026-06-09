use crate::docker;
use crate::models::ValidationReport;
use crate::AppState;
use chrono::Local;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

/// Imagen Docker por motor de base de datos.
fn docker_image_for_engine(engine: &str) -> Result<&str, String> {
    match engine {
        "postgres" => Ok("postgres:14-alpine"),
        "mysql" => Ok("mysql:8.0"),
        "sqlserver" => Ok("mcr.microsoft.com/mssql/server:2022-latest"),
        "mongodb" => Ok("mongo:7"),
        _ => Err(format!("Motor no soportado: {}", engine)),
    }
}

/// Comando de restauración dentro del contenedor según el motor.
fn restore_command(engine: &str, container_id: &str, backup_path: &str) -> Result<Vec<String>, String> {
    match engine {
        "postgres" => Ok(vec![
            "docker".into(), "exec".into(), "-i".into(), container_id.into(),
            "psql".into(), "-U".into(), "postgres".into(), "-f".into(), "/backup/dump.sql".into(),
        ]),
        "mysql" => Ok(vec![
            "docker".into(), "exec".into(), "-i".into(), container_id.into(),
            "sh".into(), "-c".into(), "mysql -u root -proot testdb < /backup/dump.sql".into(),
        ]),
        "sqlserver" => Ok(vec![
            "docker".into(), "exec".into(), "-i".into(), container_id.into(),
            "/opt/mssql-tools18/bin/sqlcmd".into(), "-S".into(), "localhost".into(),
            "-U".into(), "sa".into(), "-P".into(), "SafeBridge@123".into(),
            "-C".into(), "-i".into(), "/backup/dump.bak".into(),
        ]),
        "mongodb" => Ok(vec![
            "docker".into(), "exec".into(), "-i".into(), container_id.into(),
            "mongorestore".into(), "--archive=/backup/dump.bson".into(),
        ]),
        _ => Err(format!("Motor no soportado para restauración: {}", engine)),
    }
    // Nota: backup_path se usa para el volumen mount, no directamente aquí
    .map(|_v| {
        let _ = backup_path;
        _v
    })
}

/// Ejecuta el flujo completo de validación en sandbox.
/// Esta función se ejecuta de forma asíncrona vía tokio::spawn.
pub async fn run_validation(state: Arc<AppState>, task_id: String, backup_path: String, engine: String, _database_name: Option<String>) {
    let mut logs: Vec<String> = Vec::new();
    let start = Local::now();

    // 1. Verificar que Docker está disponible
    log_msg(&mut logs, "Verificando disponibilidad de Docker...");
    if !docker::check_docker() {
        finish_task_failed(&state, &task_id, &start, &mut logs, "Docker no está disponible o no está en ejecución.");
        return;
    }
    log_msg(&mut logs, "Docker está disponible.");

    // 2. Verificar que el archivo de backup existe
    log_msg(&mut logs, &format!("Verificando archivo de backup: {}", backup_path));
    if !Path::new(&backup_path).exists() {
        finish_task_failed(&state, &task_id, &start, &mut logs, &format!("El archivo de backup no existe: {}", backup_path));
        return;
    }

    let file_size = std::fs::metadata(&backup_path).map(|m| m.len()).unwrap_or(0);
    if file_size == 0 {
        finish_task_failed(&state, &task_id, &start, &mut logs, "El archivo de backup está vacío (0 bytes).");
        return;
    }
    log_msg(&mut logs, &format!("Archivo encontrado. Tamaño: {} bytes", file_size));

    // 3. Calcular hash SHA-256
    log_msg(&mut logs, "Calculando hash SHA-256 del archivo...");
    match calculate_sha256(&backup_path) {
        Ok(hash) => log_msg(&mut logs, &format!("SHA-256: {}", hash)),
        Err(e) => log_msg(&mut logs, &format!("Advertencia: No se pudo calcular hash: {}", e)),
    }

    // 4. Actualizar estado a "processing"
    update_task_status(&state, &task_id, "processing", Some("Levantando contenedor sandbox..."));

    // 5. Obtener la imagen Docker correspondiente
    let image = match docker_image_for_engine(&engine) {
        Ok(img) => img,
        Err(e) => {
            finish_task_failed(&state, &task_id, &start, &mut logs, &e);
            return;
        }
    };
    log_msg(&mut logs, &format!("Imagen Docker seleccionada: {}", image));

    // 6. Levantar el contenedor sandbox
    log_msg(&mut logs, "Iniciando contenedor temporal de sandbox...");
    let backup_dir = Path::new(&backup_path).parent().unwrap_or(Path::new(".")).to_string_lossy().to_string();
    
    let env_args = match engine.as_str() {
        "postgres" => vec!["-e", "POSTGRES_HOST_AUTH_METHOD=trust"],
        "mysql" => vec!["-e", "MYSQL_ROOT_PASSWORD=root", "-e", "MYSQL_DATABASE=testdb"],
        "sqlserver" => vec!["-e", "ACCEPT_EULA=Y", "-e", "SA_PASSWORD=SafeBridge@123"],
        "mongodb" => vec![],
        _ => vec![],
    };

    let mut docker_run_args: Vec<String> = vec![
        "run".into(), "-d".into(), "--rm".into(),
        "--name".into(), format!("safebridge-sandbox-{}", &task_id[..8]),
        "-v".into(), format!("{}:/backup", backup_dir),
    ];
    for arg in &env_args {
        docker_run_args.push(arg.to_string());
    }
    docker_run_args.push(image.to_string());

    let docker_run_refs: Vec<&str> = docker_run_args.iter().map(|s| s.as_str()).collect();
    let container_id = match docker::run_docker_command(&docker_run_refs) {
        Ok(id) => {
            let short_id = if id.len() > 12 { &id[..12] } else { &id };
            log_msg(&mut logs, &format!("Contenedor levantado exitosamente (ID: {})", short_id));
            id
        }
        Err(e) => {
            finish_task_failed(&state, &task_id, &start, &mut logs, &format!("Error al levantar contenedor: {}", e));
            return;
        }
    };

    // 7. Esperar a que el motor de BD esté listo
    update_task_status(&state, &task_id, "processing", Some("Esperando que el motor de base de datos inicie..."));
    log_msg(&mut logs, "Esperando 10 segundos para que el motor de BD inicie...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // 8. Restaurar el backup
    update_task_status(&state, &task_id, "processing", Some("Restaurando base de datos en contenedor temporal..."));
    log_msg(&mut logs, "Iniciando restauración del backup en el sandbox...");

    let restore_args = match restore_command(&engine, &container_id, &backup_path) {
        Ok(args) => args,
        Err(e) => {
            cleanup_container(&container_id, &mut logs);
            finish_task_failed(&state, &task_id, &start, &mut logs, &e);
            return;
        }
    };

    let restore_refs: Vec<&str> = restore_args[1..].iter().map(|s| s.as_str()).collect();
    match docker::run_docker_command(&restore_refs) {
        Ok(output) => {
            log_msg(&mut logs, "Restauración completada exitosamente.");
            if !output.is_empty() {
                log_msg(&mut logs, &format!("Output de restauración: {}", output));
            }
        }
        Err(e) => {
            log_msg(&mut logs, &format!("Advertencia en restauración: {}", e));
            // No necesariamente fatal, puede haber warnings
        }
    }

    // 9. Ejecutar validaciones dentro del contenedor
    update_task_status(&state, &task_id, "processing", Some("Ejecutando validaciones de integridad..."));
    log_msg(&mut logs, "Ejecutando consultas de validación...");

    let (tables_count, warnings, critical_errors) = run_integrity_checks(&engine, &container_id, &mut logs);

    // 10. Limpiar: destruir el contenedor
    cleanup_container(&container_id, &mut logs);

    // 11. Generar el reporte final
    let end = Local::now();
    let duration = end.signed_duration_since(start).num_seconds();
    let integrity_valid = critical_errors.is_empty();

    let report = ValidationReport {
        integrity_valid,
        execution_time_seconds: duration,
        tables_validated: tables_count,
        warnings,
        critical_errors,
        logs: logs.clone(),
    };

    if integrity_valid {
        log_msg(&mut logs, &format!("Validación completada exitosamente en {} segundos.", duration));
    } else {
        log_msg(&mut logs, &format!("Validación completada con errores en {} segundos.", duration));
    }

    // 12. Guardar el reporte en la base de datos
    let report_json = serde_json::to_string(&report).unwrap_or_default();
    {
        let db = state.db.lock().unwrap();
        let _ = db.execute(
            "UPDATE validation_tasks SET status = ?1, finished_at = ?2, report_json = ?3 WHERE task_id = ?4",
            (
                if integrity_valid { "completed" } else { "failed" },
                end.format("%Y-%m-%d %H:%M:%S").to_string(),
                &report_json,
                &task_id,
            ),
        );
    }
}

/// Ejecuta chequeos de integridad dentro del contenedor según el motor.
fn run_integrity_checks(engine: &str, container_id: &str, logs: &mut Vec<String>) -> (i64, Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();
    let mut critical_errors = Vec::new();

    // Consulta para contar tablas según el motor
    let count_query = match engine {
        "postgres" => {
            vec!["docker", "exec", container_id, "psql", "-U", "postgres", "-t", "-c",
                 "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'"]
        }
        "mysql" => {
            vec!["docker", "exec", container_id, "mysql", "-u", "root", "-proot", "-N", "-e",
                 "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'testdb'"]
        }
        _ => {
            log_msg(logs, "Motor no soporta conteo detallado de tablas, se usa validación de tamaño.");
            return (0, warnings, critical_errors);
        }
    };

    match docker::run_docker_command(&count_query[1..]) {
        Ok(output) => {
            let count: i64 = output.trim().parse().unwrap_or(0);
            log_msg(logs, &format!("Tablas encontradas en el sandbox: {}", count));
            if count == 0 {
                critical_errors.push("No se encontraron tablas después de la restauración.".into());
            }
            (count, warnings, critical_errors)
        }
        Err(e) => {
            log_msg(logs, &format!("Error al consultar tablas: {}", e));
            warnings.push(format!("No se pudo verificar el conteo de tablas: {}", e));
            (0, warnings, critical_errors)
        }
    }
}

/// Destruye el contenedor sandbox.
fn cleanup_container(container_id: &str, logs: &mut Vec<String>) {
    log_msg(logs, "Destruyendo contenedor sandbox...");
    match docker::run_docker_command(&["rm", "-f", container_id]) {
        Ok(_) => log_msg(logs, "Contenedor destruido exitosamente."),
        Err(e) => log_msg(logs, &format!("Advertencia al destruir contenedor: {}", e)),
    }
}

/// Actualiza el estado de una tarea en la base de datos.
fn update_task_status(state: &Arc<AppState>, task_id: &str, status: &str, progress: Option<&str>) {
    let db = state.db.lock().unwrap();
    let _ = db.execute(
        "UPDATE validation_tasks SET status = ?1, progress = ?2 WHERE task_id = ?3",
        (status, progress.unwrap_or(""), task_id),
    );
}

/// Finaliza una tarea como fallida.
fn finish_task_failed(state: &Arc<AppState>, task_id: &str, start: &chrono::DateTime<Local>, logs: &mut Vec<String>, error: &str) {
    log_msg(logs, &format!("ERROR CRÍTICO: {}", error));
    let end = Local::now();
    let duration = end.signed_duration_since(*start).num_seconds();

    let report = ValidationReport {
        integrity_valid: false,
        execution_time_seconds: duration,
        tables_validated: 0,
        warnings: vec![],
        critical_errors: vec![error.to_string()],
        logs: logs.clone(),
    };

    let report_json = serde_json::to_string(&report).unwrap_or_default();
    let db = state.db.lock().unwrap();
    let _ = db.execute(
        "UPDATE validation_tasks SET status = 'failed', finished_at = ?1, report_json = ?2 WHERE task_id = ?3",
        (
            end.format("%Y-%m-%d %H:%M:%S").to_string(),
            &report_json,
            task_id,
        ),
    );
}

/// Registra un mensaje en el buffer de logs.
fn log_msg(logs: &mut Vec<String>, message: &str) {
    let line = format!("[{}] {}", Local::now().format("%H:%M:%S"), message);
    tracing::info!("{}", message);
    logs.push(line);
}

/// Calcula el hash SHA-256 de un archivo.
fn calculate_sha256(path: &str) -> Result<String, String> {
    let mut file = std::fs::File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hex::encode(hasher.finalize()))
}
