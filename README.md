# SafeBridge

SafeBridge es una aplicación de escritorio multiplataforma (enfocada en Windows) construida con **Tauri v2, React y Rust** que permite generar backups de bases de datos y verificar automáticamente su integridad, manteniendo un historial completo y auditable de operaciones.

## Motores Soportados

| Motor      | Herramienta Nativa | Extensión Generada |
|------------|--------------------|--------------------|
| PostgreSQL | `pg_dump`          | `.sql`             |
| MySQL      | `mysqldump`        | `.sql`             |
| SQL Server | `sqlcmd`           | `.bak`             |
| MongoDB    | `mongodump`        | `.bson`            |

## Características Principales

1. **Gestión de Conexiones Segura** — CRUD completo de credenciales de bases de datos. Las contraseñas se almacenan de manera local y están fuertemente encriptadas utilizando **AES-256-GCM**.
2. **Backups Nativos (Sin Docker)** — La aplicación utiliza herramientas nativas directamente empaquetadas (sidecars) para generar los volcados de bases de datos. Al no depender de Docker, se evitan errores de red, garantizando velocidad y resiliencia en redes corporativas restringidas.
3. **Verificación de Integridad Profunda** — 
   - Validación nativa leyendo firmas de finalización al final de los archivos de volcado (ej. `-- Dump completed on` para MySQL y `PostgreSQL database dump complete` para PostgreSQL).
   - Generación de hashes **SHA-256** para auditoría.
4. **Historial y Auditoría Persistente** — Registro completo almacenado en una base de datos SQLite local. Captura métricas como duración del volcado, tamaño del archivo, estado de verificación y proporciona una interfaz para visualizar los **logs completos** de cada ejecución.
5. **Dashboard Analítico** — Vista general intuitiva con estadísticas de éxito/error y actividad reciente.

## Prerrequisitos para el Desarrollo

- **Node.js** v18+
- **Rust** (con `cargo`)
- Las herramientas de volcado nativas (`pg_dump`, `mysqldump`, `sqlcmd`, `mongodump`) deben estar ubicadas como sidecars en la carpeta `src-tauri/binaries/` con el sufijo de triple target correspondiente de Tauri (ej: `pg_dump-x86_64-pc-windows-msvc.exe`).
- DLLs dependientes requeridas (como dependencias de `pg_dump`) deben estar ubicadas en `src-tauri/resources/pg_deps/`.

## Instalación desde Código Fuente

```bash
# 1. Instalar dependencias del frontend (React, TailwindCSS)
npm install

# 2. Ejecutar la aplicación en modo desarrollo
npm run tauri dev
```

## Compilación para Producción

Para compilar el binario optimizado y el instalador instalable:

```bash
npm run tauri build
```

El instalador final en formato NSIS se generará en la ruta `src-tauri/target/release/bundle/nsis/`.

## Tecnologías Utilizadas

- **Frontend**: React 19, TypeScript, Vite, Tailwind CSS v4, Lucide Icons, React Hot Toast.
- **Backend**: Rust, Tauri v2, SQLite (rusqlite), Crypto (AES-256-GCM).

## Estructura del Proyecto

```text
safebridge/
├── src/                    # Frontend (React + TypeScript + Tailwind)
│   ├── components/         # Componentes reutilizables (Sidebar, Layout)
│   ├── pages/              # Vistas: Dashboard, Connections, Backup, History
│   └── App.tsx             # Enrutador principal de vistas
├── src-tauri/              # Backend de Sistema (Rust + Tauri)
│   ├── src/
│   │   ├── lib.rs          # Entry point y registro de comandos Tauri
│   │   ├── db.rs           # Inicialización y esquema SQLite
│   │   ├── connections.rs  # CRUD e Invocación de base de datos
│   │   ├── backup.rs       # Core del motor de backups y verificación nativa
│   │   ├── logs.rs         # Consultas de historial y estadísticas
│   │   ├── crypto.rs       # Funciones de cifrado AES-256-GCM
│   │   └── models.rs       # Estructuras de datos
│   ├── binaries/           # Binarios Sidecar (pg_dump, etc. - no versionados)
│   └── resources/          # DLLs de dependencias (no versionadas)
├── index.html
├── package.json
└── vite.config.ts
```
