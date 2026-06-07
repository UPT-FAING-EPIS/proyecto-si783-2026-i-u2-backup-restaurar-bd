# SafeBridge

SafeBridge es una aplicación de escritorio para Windows construida con **Tauri v2 + React + Rust** que permite generar backups de bases de datos y verificar automáticamente su integridad, manteniendo un historial completo de operaciones.

## Motores Soportados

| Motor      | Herramienta | Extensión |
|------------|-------------|-----------|
| PostgreSQL | `pg_dump`   | `.sql`    |
| MySQL      | `mysqldump` | `.sql`    |
| SQL Server | `sqlcmd`    | `.sql`    |
| MongoDB    | `mongodump` | `.archive`|

## Prerrequisitos

- **Node.js** v18+
- **Rust** (con `cargo`)
- Las herramientas de volcado nativas (`pg_dump`, `mysqldump`, `sqlcmd`, `mongodump`) deben estar disponibles como sidecars en `src-tauri/binaries/` con el sufijo de triple target (ej: `pg_dump-x86_64-pc-windows-msvc.exe`).

## Instalación desde Código Fuente

```bash
# 1. Instalar dependencias de Node
npm install

# 2. Ejecutar en modo desarrollo
npm run tauri dev
```

## Compilación para Producción

```bash
npm run tauri build
```

El instalador NSIS se generará en `src-tauri/target/release/bundle/nsis/`.

## Características

1. **Gestión de conexiones** — CRUD completo con credenciales cifradas (AES-256-GCM).
2. **Backups nativos** — Volcados directos usando herramientas sidecar empaquetadas.
3. **Verificación de integridad** — Validación SHA-256 y comprobación de firmas del archivo generado.
4. **Historial persistente** — Registro completo en SQLite local con logs detallados de ejecución.
5. **Dashboard** — Vista general con KPIs y actividad reciente.

## Estructura del Proyecto

```
safebridge/
├── src/                    # Frontend (React + TypeScript)
│   ├── components/         # Componentes reutilizables
│   ├── pages/              # Vistas principales
│   └── App.tsx             # Enrutador de vistas
├── src-tauri/              # Backend (Rust + Tauri)
│   ├── src/
│   │   ├── lib.rs          # Entry point y registro de comandos
│   │   ├── db.rs           # Inicialización SQLite
│   │   ├── connections.rs  # CRUD de conexiones
│   │   ├── backup.rs       # Motor de backups
│   │   ├── logs.rs         # Historial y estadísticas
│   │   ├── crypto.rs       # Cifrado AES-256-GCM
│   │   ├── models.rs       # Structs de datos
│   │   └── docker.rs       # Detección de Docker
│   ├── binaries/           # Sidecars (no versionados)
│   └── resources/          # DLLs de dependencia (no versionadas)
├── index.html
├── package.json
└── vite.config.ts
```
