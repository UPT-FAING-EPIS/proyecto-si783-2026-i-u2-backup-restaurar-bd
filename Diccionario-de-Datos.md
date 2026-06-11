<center>

![./media/logo-upt.png](./media/logo-upt.png)

**UNIVERSIDAD PRIVADA DE TACNA**

**FACULTAD DE INGENIERÍA**

**Escuela Profesional de Ingeniería de Sistemas**

**Proyecto: *SafeBridge: Orquestador Multi-Motor de Respaldos y Validación de Integridad***

Curso: *Base de Datos II*

Docente: *Ing. Patrick José Cuadros Quiroga*

Integrantes:

***Sierra Ruiz, Iker Alberto (2023077090)***

***Cortez Mamani, Julio Samuel (2023077283)***

**Tacna – Perú**

***2026***

</center>

<div style="page-break-after: always; visibility: hidden"></div>

Sistema *SafeBridge*

Diccionario de Datos — Base de Datos Local (SQLite)

Versión *1.0*

| CONTROL DE VERSIONES | | | | | |
|:---:|:---|:---|:---|:---|:---|
| Versión | Hecha por | Revisada por | Aprobada por | Fecha | Motivo |
| 1.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 11/06/2026 | Versión Original |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Descripción General](#1-descripción-general)
- [2. Tabla: `connections`](#2-tabla-connections)
- [3. Tabla: `backup_logs`](#3-tabla-backup_logs)
- [4. Relaciones (Constraints)](#4-relaciones-constraints)

<div style="page-break-after: always; visibility: hidden"></div>

---

## 1. Descripción General

La aplicación SafeBridge opera bajo una arquitectura *Local First*, almacenando toda la información de manera persistente en una base de datos **SQLite** ubicada en la carpeta local de la aplicación del usuario (`%APPDATA%/safebridge/safebridge.db` o `~/.config/safebridge/safebridge.db`). 

El esquema consta de dos tablas fundamentales: `connections` (para guardar credenciales cifradas) y `backup_logs` (auditoría de cada volcado).

---

## 2. Tabla: `connections`

**Propósito:** Almacena la información necesaria para establecer la conectividad con diferentes motores de bases de datos.
**Clave Primaria:** `id`

| Nombre del Campo | Tipo de Dato (SQLite) | Nulo | Descripción / Reglas |
|:-----------------|:----------------------|:-----|:---------------------|
| `id` | `TEXT` | No | UUID v4 único generado en Rust. Actúa como PK. |
| `name` | `TEXT` | No | Nombre amigable ingresado por el usuario (ej: "Producción AWS"). |
| `engine` | `TEXT` | No | Motor de base de datos. Valores aceptados: `postgres`, `mysql`, `sqlserver`, `mongodb`. |
| `host` | `TEXT` | No | Dirección IP local, remota o dominio. |
| `port` | `INTEGER` | No | Puerto numérico (ej: 5432, 3306). |
| `username` | `TEXT` | No | Nombre del usuario administrativo o con privilegios de volcado. |
| `password` | `TEXT` | Sí | Contraseña fuertemente cifrada (AES-256-GCM). Nunca en texto plano. |
| `database_name` | `TEXT` | No | Nombre lógico de la base de datos a respaldar. |
| `backup_path` | `TEXT` | No | Ruta absoluta del sistema de archivos local hacia donde se enviará el archivo. |
| `created_at` | `DATETIME` | No | Marca de tiempo por defecto (`CURRENT_TIMESTAMP`). |

---

## 3. Tabla: `backup_logs`

**Propósito:** Actúa como un registro de auditoría inmutable de todas las operaciones de orquestación, sus estados, y los resultados de las comprobaciones de integridad (EOF y Hash).
**Clave Primaria:** `id`

| Nombre del Campo | Tipo de Dato (SQLite) | Nulo | Descripción / Reglas |
|:-----------------|:----------------------|:-----|:---------------------|
| `id` | `TEXT` | No | UUID v4 único generado en Rust por cada intento de respaldo. |
| `connection_id` | `TEXT` | Sí | Llave Foránea hacia `connections(id)`. Puede ser `NULL` si la conexión fue eliminada (historial persistente). |
| `connection_name` | `TEXT` | No | Copia en duro del nombre de la conexión en el momento de la ejecución. |
| `engine` | `TEXT` | No | Copia en duro del motor usado. |
| `started_at` | `DATETIME` | No | Momento en que el proceso de volcado inicia. |
| `finished_at` | `DATETIME` | Sí | Momento en que el volcado y la verificación terminan. |
| `duration_seconds`| `INTEGER` | Sí | Tiempo total transcurrido. |
| `file_path` | `TEXT` | Sí | Ruta completa del archivo generado (`.sql`, `.bak`, etc.). |
| `file_size_bytes` | `INTEGER` | Sí | Peso exacto en disco en bytes. |
| `status` | `TEXT` | No | Estado de finalización. Valores: `OK`, `FAIL`. |
| `error_message` | `TEXT` | Sí | Extracto del `stderr` del motor de base de datos si falló. |
| `restore_verified`| `BOOLEAN` | Sí | `1` si la firma final de EOF fue exitosa; `0` si el archivo falló validación. |
| `sha256_hash` | `TEXT` | Sí | Firma criptográfica SHA-256 calculada del archivo final. |
| `full_logs` | `TEXT` | Sí | Consola cruda generada por el sidecar durante todo el proceso. |

---

## 4. Relaciones (Constraints)

- **FK_Backup_Connection**: El campo `backup_logs.connection_id` posee una restricción de clave foránea `REFERENCES connections(id) ON DELETE SET NULL`. Esto asegura que si el usuario elimina un servidor de base de datos de su lista, el registro de auditoría de los backups pasados se mantenga intacto.
