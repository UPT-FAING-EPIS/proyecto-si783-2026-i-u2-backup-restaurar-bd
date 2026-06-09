use axum::{routing::{get, post}, Router};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

mod api;
mod connections;
mod crypto;
mod db;
mod docker;
mod logs;
mod models;
mod sandbox;

/// Estado global compartido de la aplicación.
pub struct AppState {
    pub db: Mutex<Connection>,
}

#[tokio::main]
async fn main() {
    // Inicializar el sistema de logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    tracing::info!("═══════════════════════════════════════════");
    tracing::info!("  SafeBridge API v1.0.0 — Headless Mode");
    tracing::info!("═══════════════════════════════════════════");

    // Inicializar la base de datos SQLite en el directorio actual
    let data_dir = PathBuf::from("./data");
    let conn = db::init_db(&data_dir).expect("Error al inicializar la base de datos");

    let state = Arc::new(AppState {
        db: Mutex::new(conn),
    });

    // Configurar CORS para aceptar peticiones de cualquier cliente
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Definir las rutas de la API
    let app = Router::new()
        // Endpoints principales de validación
        .route("/api/v1/validation/run", post(api::run_validation))
        .route("/api/v1/validation/tasks", get(api::list_tasks))
        .route("/api/v1/validation/{id}/report", get(api::get_validation_report))
        // Health check
        .route("/health", get(api::health_check))
        // Middleware
        .layer(cors)
        // Estado compartido
        .with_state(state);

    // Iniciar el servidor
    let bind_addr = "0.0.0.0:3000";
    tracing::info!("Servidor escuchando en http://{}", bind_addr);
    tracing::info!("Documentación de endpoints:");
    tracing::info!("  POST  /api/v1/validation/run         → Iniciar validación");
    tracing::info!("  GET   /api/v1/validation/{{id}}/report → Consultar reporte");
    tracing::info!("  GET   /api/v1/validation/tasks       → Listar tareas");
    tracing::info!("  GET   /health                        → Estado del servidor");

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .expect("Error al iniciar el servidor");

    axum::serve(listener, app)
        .await
        .expect("Error fatal en el servidor");
}
