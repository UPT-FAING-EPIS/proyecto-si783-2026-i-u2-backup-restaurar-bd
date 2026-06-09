# SafeBridge API

SafeBridge API es un servicio backend **headless** (sin interfaz gráfica) construido en **Rust** con **Axum**, diseñado para validar backups de bases de datos en entornos aislados (Sandbox) mediante contenedores Docker.

## Endpoints

### `POST /api/v1/validation/run`
Recibe un backup o script, levanta un sandbox Docker de forma asíncrona, restaura los datos y ejecuta pruebas de integridad.

**Request:**
```json
{
  "backup_path": "C:/backups/mi_backup.sql",
  "engine": "postgres",
  "database_name": "mi_empresa_db"
}
```

**Response (202 Accepted):**
```json
{
  "task_id": "uuid-generado",
  "status": "queued",
  "message": "Validación iniciada en segundo plano."
}
```

### `GET /api/v1/validation/{id}/report`
Devuelve el estado y reporte detallado de una tarea de validación.

**Response (200 OK):**
```json
{
  "task_id": "uuid-generado",
  "status": "completed",
  "report": {
    "integrity_valid": true,
    "execution_time_seconds": 45,
    "tables_validated": 24,
    "warnings": [],
    "critical_errors": [],
    "logs": ["..."]
  }
}
```

### `GET /api/v1/validation/tasks`
Lista todas las tareas de validación registradas.

### `GET /health`
Estado del servidor y disponibilidad de Docker.

## Motores Soportados

| Motor      | Imagen Docker                              |
|------------|---------------------------------------------|
| PostgreSQL | `postgres:14-alpine`                       |
| MySQL      | `mysql:8.0`                                |
| SQL Server | `mcr.microsoft.com/mssql/server:2022-latest` |
| MongoDB    | `mongo:7`                                  |

## Prerrequisitos

- **Rust** (con `cargo`)
- **Docker** instalado y en ejecución

## Ejecución

```bash
# Compilar y ejecutar
cargo run

# El servidor inicia en http://localhost:3000
```

## Estructura del Proyecto

```text
safebridge-api/
├── src/
│   ├── main.rs          # Servidor Axum y enrutamiento
│   ├── api.rs           # Handlers de los endpoints (POST/GET)
│   ├── sandbox.rs       # Lógica de validación en Docker Sandbox
│   ├── docker.rs        # Utilidades para ejecutar comandos Docker
│   ├── db.rs            # Inicialización de SQLite
│   ├── models.rs        # Estructuras de datos y DTOs
│   ├── crypto.rs        # Cifrado AES-256-GCM
│   ├── connections.rs   # CRUD de conexiones (disponible)
│   └── logs.rs          # Consultas de historial (disponible)
├── Cargo.toml
└── PLAN_API.md
```

## Tecnologías

- **Axum** — Framework web de alto rendimiento
- **Tokio** — Runtime asíncrono
- **rusqlite** — Base de datos SQLite embebida
- **tower-http** — CORS middleware
- **Docker** — Sandbox de validación
