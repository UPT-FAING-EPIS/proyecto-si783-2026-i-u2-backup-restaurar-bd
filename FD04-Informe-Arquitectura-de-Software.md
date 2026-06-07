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

Diagramas de Arquitectura (Ingeniería Inversa) — FD04

Versión *2.0*

| CONTROL DE VERSIONES | | | | | |
|:---:|:---|:---|:---|:---|:---|
| Versión | Hecha por | Revisada por | Aprobada por | Fecha | Motivo |
| 1.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 20/04/2026 | Versión Original |
| 2.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 31/05/2026 | Actualización para Tauri/Rust Architecture |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Diagrama de Clases / Estructuras](#1-diagrama-de-clases--estructuras)
- [2. Diagrama de Base de Datos](#2-diagrama-de-base-de-datos)
- [3. Diagrama de Componentes](#3-diagrama-de-componentes)
- [4. Diagrama de Despliegue](#4-diagrama-de-despliegue)
- [5. Diagrama de Arquitectura](#5-diagrama-de-arquitectura)
- [6. Diagrama de Infraestructura (Terraform Cloud)](#6-diagrama-de-infraestructura-terraform-cloud)

<div style="page-break-after: always; visibility: hidden"></div>

> **Nota metodológica**: Todos los diagramas de este documento han sido generados mediante **ingeniería inversa** del código fuente del repositorio `safebridge`. Los elementos representados corresponden exclusivamente a las estructuras en Rust (`src-tauri/src`) y TypeScript (`src/`) presentes en el código real.

---

## 1. Diagrama de Clases / Estructuras

El siguiente diagrama ilustra cómo las estructuras de datos (Structs) de Rust interactúan y se exponen al frontend como interfaces en TypeScript. Al usar Rust, las "clases" no existen per se, por lo que se representan los `Structs` y sus funciones implícitas (impl blocks) o comandos Tauri asociados.

```mermaid
classDiagram
    %% ── RUST STRUCTS (src-tauri/src/models.rs y db.rs) ────────────────
    class AppState {
        <<Rust Struct>>
        +Mutex~Connection~ db
    }

    class ConnectionInfo {
        <<Rust Struct / Serde>>
        +Option~String~ id
        +String name
        +String engine
        +String host
        +u16 port
        +String username
        +Option~String~ password
        +String database_name
        +String backup_path
        +Option~String~ created_at
    }

    class BackupResult {
        <<Rust Struct / Serde>>
        +String file_path
        +u64 size_bytes
        +String sha256
        +bool verified
    }

    class BackupLogPayload {
        <<Rust Struct / Serde>>
        +String message
        +String level
    }

    %% ── RUST MODULES (Como Clases Estáticas) ─────────────────────────
    class BackupModule {
        <<Rust Module: backup.rs>>
        +generate_backup(app, state, connection_id) Result~BackupResult, String~
        -do_backup_process(app, conn, log_buffer, pass) Result
        -verify_backup(app, conn, file_path, pass, log_buffer) Result~bool~
        -calculate_hash_and_size(path) Result~u64, String~
    }

    class CryptoModule {
        <<Rust Module: crypto.rs>>
        +encrypt_password(password) Result~String~
        +decrypt_password(encrypted) Result~String~
    }

    class ConnectionsModule {
        <<Rust Module: connections.rs>>
        +create_connection(state, conn) Result~String~
        +list_connections(state) Result~Vec~ConnectionInfo~~
        +update_connection(state, id, conn) Result~()~
        +delete_connection(state, id) Result~()~
        +test_connection(host, port) Result~bool~
    }

    %% ── TYPESCRIPT INTERFACES (src/types) ───────────────────────────
    class TS_ConnectionInfo {
        <<TypeScript Interface>>
        +id?: string
        +name: string
        +engine: string
        +host: string
        +port: number
        +username: string
        +password?: string
        +database_name: string
        +backup_path: string
        +created_at?: string
    }

    %% ── RELACIONES ──────────────────────────────────────────────────
    AppState --> ConnectionInfo : Gestiona en BD
    BackupModule --> AppState : Accede al state Mutex
    BackupModule --> ConnectionInfo : Lee datos para conectarse
    BackupModule --> CryptoModule : Usa decrypt_password()
    ConnectionsModule --> CryptoModule : Usa encrypt_password()
    BackupModule --> BackupResult : Retorna
    BackupModule --> BackupLogPayload : Emite como evento
    TS_ConnectionInfo ..|> ConnectionInfo : Representación Serde JSON
```

---

## 2. Diagrama de Base de Datos

SafeBridge utiliza persistencia local mediante **SQLite** (`rusqlite`) contenida en el archivo `safebridge.db`. El código SQL de creación se encuentra en `src-tauri/src/db.rs`.

```mermaid
erDiagram
    CONNECTIONS {
        TEXT id PK "UUID autogenerado en Rust"
        TEXT name "Nombre amigable (ej: Mi Servidor Prod)"
        TEXT engine "postgres | mysql | sqlserver | mongodb"
        TEXT host "Dirección IP o DNS"
        INTEGER port "Puerto"
        TEXT username "Usuario de base de datos"
        TEXT password "Contraseña (Cifrada con AES-256-GCM)"
        TEXT database_name "Nombre de BD a respaldar"
        TEXT backup_path "Carpeta destino elegida por el usuario"
        DATETIME created_at "Timestamp automático"
    }

    BACKUP_LOGS {
        TEXT id PK "UUID autogenerado del log"
        TEXT connection_id FK "Ref: connections.id (ON DELETE SET NULL)"
        TEXT connection_name "Copia del nombre al momento de ejecución"
        TEXT engine "Motor usado"
        DATETIME started_at "Inicio del backup"
        DATETIME finished_at "Fin de validación"
        INTEGER duration_seconds "Tiempo de ejecución"
        TEXT file_path "Ruta absoluta resultante"
        INTEGER file_size_bytes "Tamaño en disco"
        TEXT status "OK | FAIL"
        TEXT error_message "Stderr capturado si falló"
        BOOLEAN restore_verified "1 (True) si la validación EOF fue exitosa"
        TEXT full_logs "Consola de salida completa generada por Rust"
    }

    CONNECTIONS ||--o{ BACKUP_LOGS : "genera (1:N)"
```

---

## 3. Diagrama de Componentes

Este diagrama refleja la arquitectura híbrida Tauri, con comunicación IPC entre React y Rust, y Rust interactuando con los binarios del sistema operativo (Sidecars).

```mermaid
graph TB
    subgraph "Frontend (Webview)"
        subgraph "React UI"
            APP["App.tsx\n(Router principal)"]
            SB["Sidebar.tsx"]
            P_DASH["Dashboard.tsx"]
            P_CONN["Connections.tsx"]
            P_BACK["Backup.tsx"]
            P_HIST["History.tsx"]
        end
        TAURI_API["@tauri-apps/api\n(core, event)"]

        APP --> SB
        APP --> P_DASH
        APP --> P_CONN
        APP --> P_BACK
        APP --> P_HIST
        P_CONN --> TAURI_API : invoke()
        P_BACK --> TAURI_API : invoke(), listen()
        P_HIST --> TAURI_API : invoke()
    end

    subgraph "Backend (Rust Core)"
        TAURI_IPC["Tauri IPC Router\n(generate_handler!)"]
        
        MOD_CONN["connections.rs\n(CRUD, TcpStream test)"]
        MOD_BACK["backup.rs\n(Orquestador, EOF check, SHA2)"]
        MOD_LOG["logs.rs\n(Read logs, Stats)"]
        MOD_CRYPTO["crypto.rs\n(aes-gcm encryption)"]
        MOD_DB["db.rs\n(rusqlite engine)"]

        TAURI_API <-->|"JSON over IPC"| TAURI_IPC
        TAURI_IPC --> MOD_CONN
        TAURI_IPC --> MOD_BACK
        TAURI_IPC --> MOD_LOG

        MOD_CONN --> MOD_CRYPTO
        MOD_BACK --> MOD_CRYPTO
        
        MOD_CONN --> MOD_DB
        MOD_BACK --> MOD_DB
        MOD_LOG --> MOD_DB
    end

    subgraph "Operating System Layer"
        SQLITE[("safebridge.db\n(SQLite)")]
        FS[("File System\n(Rutas destino)")]
        
        subgraph "Tauri Sidecars (Binarios nativos)"
            PGDUMP["pg_dump.exe"]
            MYSQLDUMP["mysqldump.exe"]
            SQLCMD["sqlcmd.exe"]
        end
        
        EXT_DB[("Servidor BD Remoto\n(PostgreSQL, MySQL, etc.)")]
    end

    MOD_DB --> SQLITE
    MOD_BACK -->|"tauri_plugin_shell"| PGDUMP
    MOD_BACK -->|"tauri_plugin_shell"| MYSQLDUMP
    MOD_BACK -->|"tauri_plugin_shell"| SQLCMD
    
    MOD_BACK -->|"Calcula Hash y lee EOF"| FS
    PGDUMP -->|"Escribe .sql"| FS
    
    PGDUMP -->|"Descarga datos vía TCP/IP"| EXT_DB
    MOD_CONN -->|"Testea puerto (TcpStream)"| EXT_DB
```

---

## 4. Diagrama de Despliegue

```mermaid
graph LR
    subgraph "Equipo del Desarrollador (Windows / Linux)"
        subgraph "SafeBridge.exe / AppImage (Tauri Bundle)"
            WEBVIEW["Webview2 / WebKit\n(React UI compilada en dist/)"]
            RUSTBIN["Binario Nativo Rust\n(Gestión de hilos y memoria)"]
            
            WEBVIEW <-->|"IPC"| RUSTBIN
        end

        subgraph "Archivos en AppData (~/.config o %APPDATA%)"
            DB[("safebridge.db")]
        end

        subgraph "Directorio de Backups (Elegido por usuario)"
            BAK[("backups/*.sql\nbackups/*.bak\nbackups/*.archive")]
        end
    end

    RUSTBIN <-->|"Lectura/Escritura"| DB
    RUSTBIN -->|"Ejecuta volcado hacia"| BAK

    subgraph "Red Interna / VPN / Nube"
        DB1[("PostgreSQL\n(Puerto 5432)")]
        DB2[("MySQL\n(Puerto 3306)")]
        DB3[("SQL Server\n(Puerto 1433)")]
    end

    RUSTBIN -->|"TDS / TCP"| DB1
    RUSTBIN -->|"TCP"| DB2
    RUSTBIN -->|"TCP"| DB3

    style RUSTBIN fill:#d4531c,color:#fff
    style WEBVIEW fill:#3776ab,color:#fff
    style DB fill:#003B57,color:#fff
```

---

## 5. Diagrama de Arquitectura

El proyecto adopta los principios de **Clean Architecture**, aunque al ser una aplicación Tauri pequeña la organización está dictada por módulos. La "interfaz" que divide las capas está impuesta físicamente entre Node.js (Vite) y Rust (Tauri).

```mermaid
graph TD
    subgraph "🖥️ Presentation Layer (TypeScript / React)"
        UI["React Router\nComponentes Tailwind\nGestión de Estado (useState)\n• Llama a invoke() de Tauri\n• Reacciona a eventos (app.listen)"]
    end

    subgraph "⚙️ Application Layer (Rust Commands)"
        TAURI["Tauri Commands (main.rs)\n• generate_backup()\n• test_connection()\n• create_connection()\n• Encapsulan casos de uso"]
    end

    subgraph "📐 Domain Layer (Rust Structs)"
        DOM["Models (models.rs)\n• ConnectionInfo\n• BackupResult\n• BackupLogPayload\n• (Estructuras puras sin dependencias externas)"]
    end

    subgraph "🔧 Infrastructure Layer (Rust Modules)"
        DB["Database (db.rs)\n• Consultas SQLite\n• Conexión rusqlite"]
        CRYP["Cryptography (crypto.rs)\n• aes-gcm\n• sha2 (Hashing)"]
        SHELL["Shell & FileSystem (backup.rs)\n• Invoca Sidecars\n• Lee chunks de archivos del SO"]
    end

    UI -->|"Aísla al usuario de la lógica"| TAURI
    TAURI -->|"Usa Modelos"| DOM
    TAURI -->|"Delega persistencia"| DB
    TAURI -->|"Delega seguridad"| CRYP
    TAURI -->|"Ejecuta procesos nativos"| SHELL

    style UI fill:#2d5a8e,color:#fff
    style TAURI fill:#5a8e2d,color:#fff
    style DOM fill:#8e6d2d,color:#fff
    style DB fill:#8e2d2d,color:#fff
    style CRYP fill:#8e2d2d,color:#fff
    style SHELL fill:#8e2d2d,color:#fff
```

---

## 6. Diagrama de Infraestructura (Terraform Cloud)

> **Contexto:** Aunque el MVP de SafeBridge funciona en local, este diagrama responde al Análisis Económico de Cloud (`FD01-Informe-Factibilidad.md`), ilustrando cómo se puede extender la validación de backups hacia AWS usando Terraform. 

La idea es que una versión futura de SafeBridge suba automáticamente el backup a un S3 Bucket. Una función Lambda detectaría el archivo y levantaría temporalmente una base de datos Amazon RDS para restaurar el archivo, validarlo profundamente, emitir el resultado y autodestruirse.

```mermaid
graph TD
    subgraph "Entorno Local del Desarrollador"
        SB["SafeBridge v2.0\n(Tauri + AWS SDK Rust)"]
        BAK["Archivo Local .sql"]
        SB --> BAK
    end

    subgraph "Infraestructura AWS (Gestionada por Terraform)"
        S3["Amazon S3\nsafebridge-secure-backups\n(Bucket Privado)"]
        
        LAMBDA["AWS Lambda\n(Orquestador de Validación)"]
        
        RDS["Amazon RDS\n(Instancia db.t4g.micro efímera)"]
        
        DYNAMO["Amazon DynamoDB\n(Registro de Validación Remota)"]
    end

    SB -->|"1. PutObject (Upload del .sql)"| S3
    S3 -->|"2. Evento s3:ObjectCreated"| LAMBDA
    LAMBDA -->|"3. Levanta y restaura"| RDS
    LAMBDA -->|"4. Verifica datos SQL y destruye RDS"| RDS
    LAMBDA -->|"5. Guarda resultado"| DYNAMO
    SB -.->|"6. Consulta Estado Remoto"| DYNAMO

    style S3 fill:#7aa116,color:#fff
    style LAMBDA fill:#ff9900,color:#000
    style RDS fill:#336699,color:#fff
    style DYNAMO fill:#336699,color:#fff
```

---

*Documento generado por el equipo BitCraft Solutions — Universidad Privada de Tacna, FAING-EPIS, Ciclo 2026-I.*
