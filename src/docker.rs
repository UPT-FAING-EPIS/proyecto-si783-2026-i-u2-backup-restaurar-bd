use std::process::Command;

/// Verifica si Docker está instalado y en ejecución en el sistema.
pub fn check_docker() -> bool {
    let output = Command::new("docker")
        .arg("info")
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

/// Ejecuta un comando Docker y retorna (success, stdout, stderr).
pub fn run_docker_command(args: &[&str]) -> Result<String, String> {
    let output = Command::new("docker")
        .args(args)
        .output()
        .map_err(|e| format!("Error ejecutando docker: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Err(format!("{} {}", stderr, stdout))
    }
}
