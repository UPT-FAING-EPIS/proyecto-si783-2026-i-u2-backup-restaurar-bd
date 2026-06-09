# Plan de Migración Altamente Eficiente: SafeBridge API Headless

Este plan detalla la manera **más eficiente y directa** de transformar el actual SafeBridge (Rust/Tauri) en una API Headless. Al reutilizar el núcleo de Rust y eliminar la carga de la interfaz gráfica, garantizamos un alto rendimiento, bajo consumo de recursos y una transición rápida.

## 1. La Estrategia Más Eficiente

En lugar de reescribir la lógica en otro lenguaje, la solución óptima es:
1. **Desacoplar la UI:** Eliminar por completo el frontend (React) y la dependencia de Tauri.
2. **Implementar Axum:** Utilizar `Axum` (el framework web más rápido y eficiente de Rust, mantenido por el equipo de Tokio) para exponer las funciones como API.
3. **Ejecución Asíncrona (Tokio):** Aprovechar el ecosistema asíncrono de Rust para levantar el sandbox sin bloquear el servidor.

## 2. Endpoints Requeridos

### 2.1. `POST /api/v1/validation/run`
**Rol:** Recibe un backup o un script. Levanta el sandbox de forma asíncrona, restaura los datos y realiza las pruebas.

- **Request (Ejemplo JSON):**
  ```json
  {
    "backup_path": "C:/backups/archivo.sql",
    "engine": "postgres"
  }
  ```
- **Acción Interna (Máxima Eficiencia):**
  1. Genera un `task_id` único y lo guarda en la base de datos local SQLite con estado `procesando`.
  2. Lanza un hilo asíncrono (`tokio::spawn`) que ejecutará los comandos de Docker (Sandbox) en segundo plano.
  3. Responde **inmediatamente** al cliente.
- **Response (202 Accepted):**
  ```json
  {
    "task_id": "uuid-1234",
    "status": "processing",
    "message": "Sandbox levantándose asíncronamente."
  }
  ```

### 2.2. `GET /api/v1/validation/{id}/report`
**Rol:** Devuelve un JSON con el resultado detallado de la integridad (tablas validadas, advertencias, errores críticos).

- **Acción Interna:** Consulta rápida a la base de datos SQLite por el `task_id`.
- **Response (200 OK):**
  ```json
  {
    "task_id": "uuid-1234",
    "status": "completed",
    "report": {
      "integrity_valid": true,
      "tables_validated": 45,
      "warnings": [],
      "critical_errors": [],
      "logs": [
         "Contenedor levantado.",
         "Datos restaurados con éxito."
      ]
    }
  }
  ```

## 3. Pasos de Ejecución Inmediata

1. **Limpieza Quirúrgica:** Borrar la carpeta `src` (frontend), `index.html` y configuraciones de Node/Vite.
2. **Actualización de `Cargo.toml`:** Eliminar `tauri` y agregar `axum`, `tokio`, `serde`, `serde_json`.
3. **Punto de Entrada (`main.rs`):** Crear el servidor web Axum en el puerto 3000.
4. **Adaptación de Rutas (`api.rs`):** Programar el POST y el GET inyectando el pool de conexiones de SQLite.
5. **Lógica de Sandbox (`sandbox.rs`):** Usar `std::process::Command` para invocar al CLI de Docker (`docker run`, `docker exec`) de forma nativa desde Rust.

---
*Con este enfoque, el proyecto queda 100% en Rust, minimizando dependencias, asegurando un rendimiento nativo y cumpliendo exactamente con la estructura de endpoints solicitada.*
