use rusqlite::Connection;
use std::sync::Mutex;
use tauri::Manager;

mod db;
mod docker;
mod models;
mod crypto;
mod connections;
mod backup;
mod logs;

pub struct AppState {
    pub db: Mutex<Connection>,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let conn = db::init_db(&app_dir).expect("failed to init db");
            
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            docker::check_docker,
            connections::create_connection,
            connections::list_connections,
            connections::update_connection,
            connections::delete_connection,
            connections::test_connection,
            backup::generate_backup,
            logs::list_logs,
            logs::get_dashboard_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
