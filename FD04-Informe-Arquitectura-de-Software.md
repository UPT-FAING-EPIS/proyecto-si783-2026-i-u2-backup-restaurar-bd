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
| 2.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 11/06/2026 | Actualización para añadir Casos de Uso y Diagramas de Secuencia |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Diagrama de Casos de Uso](#1-diagrama-de-casos-de-uso)
- [2. Diagramas de Secuencia](#2-diagramas-de-secuencia)
- [3. Diagrama de Clases / Estructuras](#3-diagrama-de-clases--estructuras)
- [4. Diagrama de Base de Datos](#4-diagrama-de-base-de-datos)
- [5. Diagrama de Componentes](#5-diagrama-de-componentes)
- [6. Diagrama de Despliegue](#6-diagrama-de-despliegue)
- [7. Diagrama de Arquitectura](#7-diagrama-de-arquitectura)
- [8. Diagrama de Infraestructura (Terraform Cloud)](#8-diagrama-de-infraestructura-terraform-cloud)

<div style="page-break-after: always; visibility: hidden"></div>

> **Nota metodológica**: Todos los diagramas de este documento han sido elaborados en base a las Historias de Usuario (Casos de Uso y Secuencia) y generados mediante **ingeniería inversa** del código fuente (Arquitectura, Clases, Componentes).

---

## 1. Diagrama de Casos de Uso

A continuación se presentan los Casos de Uso derivados directamente de las Historias de Usuario (HU-01 a HU-06).

```mermaid
usecaseDiagram
actor "Desarrollador (Usuario)" as User
actor "Sistema SafeBridge (Rust Core)" as System

rectangle "Gestión de Backups y Conexiones" {
  User -- (Gestionar Conexiones DB)
  (Gestionar Conexiones DB) ..> (Cifrar Credenciales AES) : <<include>>
  
  User -- (Probar Conectividad DB)
  User -- (Generar Volcado Multi-Motor)
  
  (Generar Volcado Multi-Motor) ..> (Inyectar Credencial Temporal) : <<include>>
  (Generar Volcado Multi-Motor) ..> (Emitir Logs en Tiempo Real) : <<include>>
  
  System -- (Validar Integridad EOF)
  System -- (Calcular Hash SHA-256)
  
  (Generar Volcado Multi-Motor) <.. (Validar Integridad EOF) : <<extend>>
  (Generar Volcado Multi-Motor) <.. (Calcular Hash SHA-256) : <<extend>>
  
  User -- (Ver Historial y Auditoría)
}
```

---

## 2. Diagramas de Secuencia

Estos diagramas representan las interacciones detalladas en los escenarios de prueba del FD03.

### 2.1. Creación Segura de Conexión y Prueba de Red

Este diagrama muestra cómo se transmite una contraseña desde la vista de React y se asegura utilizando AES antes de impactar en SQLite.

```mermaid
sequenceDiagram
    actor Usuario
    participant React as Componente UI<br/>(Connections.tsx)
    participant Tauri as Rust Command<br/>(connections.rs)
    participant Crypto as Módulo Crypto<br/>(aes-gcm)
    participant TCP as TcpStream
    participant DB as SQLite<br/>(safebridge.db)

    Usuario->>React: Completa formulario y clic "Test Connection"
    React->>Tauri: test_connection("10.0.0.5", 3306)
    Tauri->>TCP: TcpStream::connect_timeout()
    alt Host responde
        TCP-->>Tauri: Ok(Stream)
        Tauri-->>React: true
        React->>Usuario: Muestra "Test Exitoso" en UI
    else Timeout
        TCP-->>Tauri: Err(Connection Refused)
        Tauri-->>React: false
        React->>Usuario: Muestra "Test Fallido" en UI
    end

    Usuario->>React: Clic en "Guardar"
    React->>Tauri: create_connection(ConnectionInfo)
    Tauri->>Crypto: encrypt_password(conn.password)
    Crypto-->>Tauri: encrypted_password (Hex)
    Tauri->>DB: INSERT INTO connections (...) VALUES (...)
    DB-->>Tauri: Éxito
    Tauri-->>React: Ok(ID_generado)
    React->>Usuario: Cierra modal, actualiza lista
```

### 2.2. Flujo de Generación de Backup y Validación

El proceso más crítico del orquestador, mostrando el ciclo asíncrono y la verificación nativa EOF.

```mermaid
sequenceDiagram
    actor Usuario
    participant React as Panel de Backup<br/>(Backup.tsx)
    participant Tauri as Rust Command<br/>(generate_backup)
    participant Crypto as Módulo Crypto
    participant Shell as tauri_plugin_shell<br/>(Sidecar)
    participant FS as File System
    participant DB as SQLite<br/>(backup_logs)

    Usuario->>React: Clic "Generar Backup" (ID: 123)
    React->>Tauri: invoke("generate_backup", { connection_id })
    Note over Tauri: Se crea un hilo asíncrono
    Tauri->>DB: SELECT * FROM connections WHERE id = 123
    DB-->>Tauri: ConnectionInfo (incluye pass cifrado)
    Tauri->>Crypto: decrypt_password(encrypted)
    Crypto-->>Tauri: plain_password
    Tauri->>React: emit("backup_log", "Iniciando proceso...")

    Tauri->>Shell: sidecar("pg_dump").env("PGPASSWORD", plain_password)
    Note over Shell: Se genera archivo en disco local
    Shell-->>Tauri: output (stdout/stderr)
    Tauri->>React: emit("backup_log", "Volcado generado exitosamente.")
    
    Tauri->>FS: calculate_hash_and_size(file_path)
    loop Lectura en Chunks de 8KB
        FS-->>Tauri: bytes
        Tauri->>Tauri: hasher.update(bytes)
    end
    Tauri-->>Tauri: SHA-256 resultante
    Tauri->>React: emit("backup_log", "SHA-256 calculado")

    Tauri->>FS: verify_backup() → SeekFrom::End(-256)
    FS-->>Tauri: Últimos 256 bytes
    Tauri->>Tauri: Buscar cadena "PostgreSQL database dump complete"
    alt Firma presente
        Tauri->>React: emit("backup_log", "Verificación exitosa")
        Tauri->>Tauri: verified = true
    else Firma ausente
        Tauri->>React: emit("backup_log", "Fallo de validación")
        Tauri->>Tauri: verified = false
    end

    Tauri->>DB: INSERT INTO backup_logs (status, hash, verified...)
    DB-->>Tauri: OK
    Tauri-->>React: Result(BackupResult)
    React->>Usuario: Muestra confirmación y detalles visuales (OK/FAIL)
```

---

## 3. Diagrama de Clases / Estructuras

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

## 4. Diagrama de Base de Datos

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

## 5. Diagrama de Componentes

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

## 6. Diagrama de Despliegue

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

## 7. Diagrama de Arquitectura

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

## 8. Diagrama de Infraestructura (Terraform Cloud)

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

*Documento actualizado por el equipo BitCraft Solutions — Universidad Privada de Tacna, FAING-EPIS, Ciclo 2026-I.*
