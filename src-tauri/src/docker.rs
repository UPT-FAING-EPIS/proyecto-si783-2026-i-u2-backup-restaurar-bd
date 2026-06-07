use std::process::Command;

#[tauri::command]
pub fn check_docker() -> bool {
    let output = Command::new("docker")
        .arg("info")
        .output();
        
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}
