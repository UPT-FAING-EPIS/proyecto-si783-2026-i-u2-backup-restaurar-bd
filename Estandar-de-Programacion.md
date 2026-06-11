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

Estándar de Programación

Versión *1.0*

| CONTROL DE VERSIONES | | | | | |
|:---:|:---|:---|:---|:---|:---|
| Versión | Hecha por | Revisada por | Aprobada por | Fecha | Motivo |
| 1.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 11/06/2026 | Versión Original |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Introducción](#1-introducción)
- [2. Estándares para Backend (Rust)](#2-estándares-para-backend-rust)
- [3. Estándares para Frontend (TypeScript / React)](#3-estándares-para-frontend-typescript--react)
- [4. Control de Versiones (Git y Commits)](#4-control-de-versiones-git-y-commits)

<div style="page-break-after: always; visibility: hidden"></div>

---

## 1. Introducción

El presente documento define los estándares de programación, convenciones de nomenclatura y mejores prácticas para el desarrollo del proyecto **SafeBridge**. Al tratarse de una aplicación híbrida usando **Tauri**, se exigen estándares separados pero consistentes para el backend en Rust y el frontend en TypeScript (React).

---

## 2. Estándares para Backend (Rust)

El backend de SafeBridge sigue estrictamente las directrices idiomáticas de la comunidad de Rust. Se requiere el uso del linter oficial `clippy` y el formateador `rustfmt`.

### 2.1. Nomenclatura (Naming Conventions)
- **Variables y Funciones:** `snake_case`. (ej. `let connection_id = ...`, `fn verify_backup()`).
- **Structs y Enums:** `UpperCamelCase`. (ej. `struct ConnectionInfo`, `enum BackupStatus`).
- **Constantes y Estáticas:** `SCREAMING_SNAKE_CASE`. (ej. `const MAX_TIMEOUT: u32 = 3000;`).
- **Macros:** Siempre seguidas de `!`. (ej. `println!()`, `format!()`).

### 2.2. Manejo de Errores
- **No hacer `unwrap()` en producción:** El uso de `.unwrap()` o `.expect()` está prohibido para lógicas críticas que manejen inputs del usuario.
- **Uso de Result:** Las funciones que puedan fallar deben retornar un `Result<T, E>`.
- Se permite el uso del operador `?` para propagar errores tempranamente.

### 2.3. Formateo y Estructura
- Mantener los bloques `impl` separados de las definiciones de `struct`.
- Indentación estándar de Rust: 4 espacios (no usar Tabs).
- Ancho máximo de línea: 100 caracteres.

---

## 3. Estándares para Frontend (TypeScript / React)

### 3.1. Nomenclatura y Tipado
- **Variables y Funciones TS:** `camelCase`. (ej. `const handleGenerateBackup = () => ...`).
- **Componentes React:** `PascalCase`. (ej. `function ConnectionModal() { ... }`).
- **Archivos:** `PascalCase.tsx` para componentes React (ej. `Dashboard.tsx`). `camelCase.ts` para funciones de utilidad.
- **Tipos e Interfaces:** `PascalCase` usando siempre `interface` o `type`. 
- **Estrictez:** No se permite el uso del tipo `any`. TypeScript debe estar en modo estricto (`"strict": true` en el `tsconfig.json`).

### 3.2. Estructura de Componentes
- Emplear Componentes Funcionales y Hooks (`useState`, `useEffect`). Las clases de React (Class Components) están depreciadas en este proyecto.
- Evitar lógica de redugio pesada directamente dentro de los componentes visuales; separar en Hooks personalizados (ej. `useConnections()`).

### 3.3. Estilos (Tailwind CSS)
- No usar archivos `.css` puros a menos que sea estrictamente necesario para variables globales.
- Evitar concatenaciones impuras de clases.

---

## 4. Control de Versiones (Git y Commits)

El proyecto utiliza **Conventional Commits** para generar logs y releases automáticos a través de GitHub Actions.

**Formato exigido:**
`<tipo>(<alcance opcional>): <descripción corta>`

**Tipos válidos:**
- `feat:` Una nueva característica.
- `fix:` Corrección de un bug.
- `docs:` Cambios que solo afectan la documentación.
- `style:` Cambios que no afectan el significado del código (espacios, formato).
- `refactor:` Un cambio de código que no arregla un bug ni añade una característica.
- `test:` Adición o corrección de pruebas (Playwright o BDD).
- `chore:` Actualización de tareas de compilación, gestor de paquetes o configuraciones.

**Ejemplos correctos:**
- `feat(rust): agregar soporte para mysql a tauri shell`
- `fix(ui): resolver desbordamiento del botón en historial`
- `test(bdd): implementar escenarios dado/cuando/entonces en FD03`
