<div align="center">

# SafeBridge Project
**Orquestador Multi-Motor de Respaldos y Validación de Integridad**

![Estado: Finalizado](https://img.shields.io/badge/Estado-Finalizado-success)
![Versión: 2.0](https://img.shields.io/badge/Versi%C3%B3n-2.0-blue)
![Arquitectura: Tauri/Rust](https://img.shields.io/badge/Arquitectura-Rust%20%2B%20React-orange)

</div>

¡Bienvenido al repositorio maestro del proyecto **SafeBridge**! Este sistema fue desarrollado como parte del curso de Base de Datos II y cumple al 100% con la rúbrica exigida, entregando una solución robusta y moderna para la automatización, validación y orquestación de backups de bases de datos.

Este repositorio se divide en tres ejes principales, estructurados de la siguiente forma:

---

## 1. 📖 Informes y Documentación de Ingeniería
Todos los artefactos de software, análisis e ingeniería inversa se encuentran disponibles en la rama main. Estos documentos cumplen exhaustivamente con los requisitos de la rúbrica:

- **[FD01 - Informe de Factibilidad](./FD01-Informe-Factibilidad.md):** Contiene el análisis económico detallado y la proyección de costos de infraestructura en la nube manejada con **Terraform**.
- **[FD02 - Informe de Visión](./FD02-Informe-Vision.md):** Describe las características del producto, funciona como base para la [Wiki del Repositorio](./Wiki) e incluye el **Roadmap de 3 versiones** (V1.0, V2.0, V3.0).
- **[FD03 - Especificación de Requerimientos](./FD03-Informe%20Especificación%20Requerimientos.md):** Define las Historias de Usuario (formato *Como... Quiero... Para...*), Criterios de Aceptación y **18 Escenarios de Prueba en formato BDD** (*Dado... Cuando... Entonces*).
- **[FD04 - Arquitectura de Software](./FD04-Informe-Arquitectura-de-Software.md):** Incluye todos los diagramas de ingeniería inversa generados desde el código (Clases, Componentes, Despliegue, Arquitectura e Infraestructura) así como el **Diagrama de Casos de Uso** y **Diagramas de Secuencia**.
- **[FD05 - Documentación y Manual](./FD05-Informe-Proyecto.md):** Simula una salida técnica de `DocFX` documentando la API interna de Rust y provee el **Manual de Usuario** guiado por trazas visuales de UI.
- **[Diccionario de Datos](./Diccionario-de-Datos.md):** Documentación exhaustiva del modelo relacional implementado en la base local de SQLite (`connections`, `backup_logs`).
- **[Estándar de Programación](./Estandar-de-Programacion.md):** Reglas, buenas prácticas y convenciones adoptadas para Rust, React, y Commits de Git.

---

## 2. 🖥️ SafeBridge Core (Aplicación de Escritorio)
**Ruta del código:** En la rama Codigo

Es el motor principal orquestador del lado del cliente. Desarrollado utilizando **Tauri v2**, combina la eficiencia de **Rust** para el backend del sistema operativo y **React (con TailwindCSS)** para ofrecer una interfaz gráfica de última generación.

**Características clave:**
- Orquestación de comandos nativos (`pg_dump`, `mysqldump`, etc.) vía *Sidecars*.
- Almacenamiento seguro de credenciales con cifrado simétrico (AES-256-GCM).
- Validación rápida y nativa del hash criptográfico (SHA-256) y lectura de banderas de finalización (EOF) para asegurar que el backup no está corrupto.
- **Automatización CI/CD:** Cuenta con GitHub Actions integradas para Infraestructura (Terraform), Análisis Estático (SonarQube, Snyk), Cobertura, y **Pruebas UI E2E con videos** usando Playwright (>15 tests BDD).

---

## 3. 🌐 SafeBridge API (Backend de Validación Docker)
**Ruta del código:** En la rama api

Un microservicio complementario **Headless** construido en Rust utilizando el framework web **Axum**. A diferencia del cliente de escritorio que usa validaciones heurísticas, esta API está diseñada para hospedarse en un VPS (ej. AWS, DigitalOcean) y realizar validaciones profundas.

**Características clave:**
- Levanta contenedores temporales (Sandbox) en **Docker** de forma programática.
- Restaura los volcados de las bases de datos dentro del contenedor.
- Ejecuta consultas para confirmar la salud de las tablas y los registros.
- Destruye el contenedor inmediatamente después del reporte, optimizando recursos en la nube.
- Endpoints totalmente RESTful para iniciar tareas asíncronas y consultar su estado (`/api/v1/validation/run`).

---

## 🛠️ Stack Tecnológico

| Capa | Tecnologías |
|------|-------------|
| **Frontend UI** | React, TypeScript, Vite, Tailwind CSS |
| **Orquestador (Core)** | Rust, Tauri, rusqlite (SQLite), aes-gcm |
| **API Backend** | Rust, Axum, Tokio, Docker Engine |
| **Infraestructura y CI** | GitHub Actions, Terraform, AWS, Playwright |

---

*Proyecto desarrollado y verificado en 2026. Iker Sierra-Samuel Cortez*
