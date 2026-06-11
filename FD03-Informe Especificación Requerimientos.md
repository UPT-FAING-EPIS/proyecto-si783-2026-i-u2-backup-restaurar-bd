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

Historias de Usuario y Escenarios de Prueba — FD03

Versión *2.0*

| CONTROL DE VERSIONES | | | | | |
|:---:|:---|:---|:---|:---|:---|
| Versión | Hecha por | Revisada por | Aprobada por | Fecha | Motivo |
| 1.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 12/04/2026 | Versión Original |
| 2.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 11/06/2026 | Actualización de BDD (Dado/Cuando/Entonces) y migración de diagramas a FD04 |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Historias de Usuario](#1-historias-de-usuario)
- [2. Criterios de Aceptación](#2-criterios-de-aceptación)
- [3. Escenarios de Prueba (BDD)](#3-escenarios-de-prueba-bdd)

<div style="page-break-after: always; visibility: hidden"></div>

---

## 1. Historias de Usuario

Las siguientes historias de usuario han sido derivadas directamente de las funcionalidades implementadas en el código fuente de SafeBridge MVP (frontend en React y backend en Tauri/Rust).

---

### HU-01 — Gestión Centralizada de Conexiones de Base de Datos

**Identificador**: HU-01  
**Módulo**: Conexiones  
**Prioridad**: Alta  
**Estimación**: 5 puntos de historia

> **Como** desarrollador de software que trabaja con diferentes motores,  
> **quiero** registrar, editar y listar las credenciales de mis servidores de base de datos en una interfaz unificada,  
> **para** no tener que ingresar contraseñas y parámetros en la terminal repetidamente al momento de requerir un backup.

---

### HU-02 — Prueba Rápida de Conectividad a la Base de Datos

**Identificador**: HU-02  
**Módulo**: Conexiones  
**Prioridad**: Media  
**Estimación**: 2 puntos de historia

> **Como** desarrollador que ha configurado un nuevo servidor de base de datos,  
> **quiero** poder probar la conectividad de red con el host y el puerto directamente desde la aplicación,  
> **para** asegurarme de que el servidor está alcanzable antes de intentar ejecutar operaciones críticas como un volcado.

---

### HU-03 — Generación de Volcados Multi-Motor

**Identificador**: HU-03  
**Módulo**: Orquestador de Backups  
**Prioridad**: Crítica  
**Estimación**: 8 puntos de historia

> **Como** usuario preocupado por la integridad de sus datos,  
> **quiero** generar archivos de copia de seguridad seleccionando simplemente el motor y la conexión,  
> **para** que el sistema orqueste automáticamente la ejecución del cliente nativo (`pg_dump`, `mysqldump`, etc.) y me envíe el registro del progreso en tiempo real.

---

### HU-04 — Validación Nativa de Integridad del Archivo

**Identificador**: HU-04  
**Módulo**: Validación de Integridad  
**Prioridad**: Alta  
**Estimación**: 5 puntos de historia

> **Como** sistema orquestador de copias de seguridad,  
> **quiero** calcular el hash SHA-256 del archivo recién generado y leer sus últimos bytes para verificar la firma de conclusión del motor (EOF),  
> **para** certificar que el archivo resultante no está corrupto ni truncado por una interrupción en el proceso.

---

### HU-05 — Cifrado Local de Credenciales (AES-256)

**Identificador**: HU-05  
**Módulo**: Seguridad  
**Prioridad**: Alta  
**Estimación**: 5 puntos de historia

> **Como** usuario que registra credenciales sensibles de producción,  
> **quiero** que mis contraseñas sean encriptadas de forma transparente antes de ser guardadas en disco y nunca expuestas en texto plano en la interfaz,  
> **para** proteger mi acceso en caso de compromiso físico del archivo de la base de datos local SQLite.

---

### HU-06 — Auditoría e Historial de Backups

**Identificador**: HU-06  
**Módulo**: Historial y Dashboard  
**Prioridad**: Media  
**Estimación**: 3 puntos de historia

> **Como** usuario que busca tener control sobre la política de respaldos,  
> **quiero** visualizar un historial inmutable de todos los backups generados, indicando tiempos de ejecución, estado y validación,  
> **para** mantener una auditoría técnica completa sin depender de investigar carpetas del sistema operativo.

---

## 2. Criterios de Aceptación

### CA-01 — Gestión y Cifrado de Conexiones

| ID | Criterio |
|:--:|:---------|
| CA-01-1 | Las contraseñas insertadas se cifran mediante AES-GCM antes del `INSERT` en SQLite. |
| CA-01-2 | El comando `list_connections` no retorna la contraseña en el struct enviado a React. |
| CA-01-3 | El UUID generado es único para cada conexión. |

### CA-02 — Ejecución del Sidecar (Backup)

| ID | Criterio |
|:--:|:---------|
| CA-02-1 | El nombre de archivo autogenerado respeta el patrón configurado. |
| CA-02-2 | La contraseña se inyecta en variables de entorno o STDIN de forma temporal segura. |
| CA-02-3 | Los logs del proceso se envían al frontend en tiempo real. |

### CA-03 — Validación de Integridad (EOF)

| ID | Criterio |
|:--:|:---------|
| CA-03-1 | Archivo de PostgreSQL debe contener firma final correcta en los últimos bytes. |
| CA-03-2 | Archivo de MySQL debe contener firma final correcta en los últimos bytes. |
| CA-03-3 | El SHA-256 generado coincide de forma inmutable con el archivo físico y es guardado. |

---

## 3. Escenarios de Prueba (BDD)

Se han definido 18 escenarios de prueba (2 por cada Criterio de Aceptación) utilizando el formato formal **Dado... Cuando... Entonces**.

### Criterio: CA-01-1 (Cifrado de Contraseñas)

**Escenario 1: Cifrado exitoso de contraseña válida**
```gherkin
Dado que el usuario ha ingresado la contraseña "Secreta123" en el formulario de nueva conexión
Cuando el sistema Rust recibe la petición de guardado
Entonces la función crypto::encrypt_password convierte la contraseña en un hash AES-GCM
Y el valor almacenado en la tabla SQLite luce ilegible (hexadecimal).
```

**Escenario 2: Fallo intencionado de cifrado por clave de entorno faltante**
```gherkin
Dado que la llave maestra de cifrado del sistema ha sido corrompida o eliminada
Cuando el sistema Rust intenta cifrar la contraseña recibida
Entonces el módulo crypto retorna un Err
Y la aplicación muestra una alerta crítica indicando "Error interno de cifrado" sin guardar la conexión.
```

### Criterio: CA-01-2 (Ocultamiento de Contraseñas hacia Frontend)

**Escenario 3: Obtención de lista de conexiones**
```gherkin
Dado que el usuario navega a la pantalla "Conexiones"
Cuando React invoca el comando Tauri list_connections()
Entonces Rust ejecuta un SELECT en SQLite
Y el JSON devuelto al frontend tiene el campo "password" establecido en null para todos los registros.
```

**Escenario 4: Edición de una conexión existente**
```gherkin
Dado que el usuario hace clic en el botón "Editar" de la conexión "DB_Ventas"
Cuando el modal de edición se abre
Entonces el campo de contraseña aparece vacío
Y un texto de ayuda indica "Deje en blanco para mantener la contraseña actual".
```

### Criterio: CA-01-3 (Unicidad de UUID)

**Escenario 5: Generación estándar de UUID**
```gherkin
Dado que el usuario guarda una nueva conexión
Cuando el backend en Rust prepara el objeto ConnectionInfo
Entonces se genera un identificador UUIDv4 de 36 caracteres
Y la base de datos lo acepta como PRIMARY KEY sin conflicto.
```

**Escenario 6: Prevención de duplicados**
```gherkin
Dado que una conexión con el UUID "123e4567-e89b-12d3-a456-426614174000" ya existe
Cuando una operación de inserción forzada intenta usar el mismo UUID
Entonces SQLite arroja un error UNIQUE CONSTRAINT
Y Rust captura el error registrándolo en los logs del sistema operativo.
```

### Criterio: CA-02-1 (Patrón de nombre de archivo)

**Escenario 7: Patrón correcto para PostgreSQL**
```gherkin
Dado que el usuario inicia un respaldo para la base de datos "inventario_db" usando PostgreSQL
Cuando el orquestador prepara la ruta de destino
Entonces el nombre de archivo generado sigue el formato "inventario_db_YYYYMMDD_HHMMSS.sql"
Y el archivo se guarda en la ruta por defecto del usuario.
```

**Escenario 8: Patrón correcto para MongoDB**
```gherkin
Dado que el usuario inicia un respaldo para la base de datos "nosql_data" usando MongoDB
Cuando el orquestador prepara la ruta de destino
Entonces el nombre de archivo generado sigue el formato "nosql_data_YYYYMMDD_HHMMSS.archive"
Y la extensión corresponde correctamente a mongodump.
```

### Criterio: CA-02-2 (Inyección Segura de Contraseña)

**Escenario 9: Inyección en PostgreSQL vía variable de entorno**
```gherkin
Dado que el proceso de backup para PostgreSQL está a punto de iniciar
Cuando Rust instancia el sidecar "pg_dump"
Entonces inyecta la contraseña descifrada exclusivamente en la variable de entorno temporal "PGPASSWORD"
Y el proceso nativo se ejecuta sin exponer la clave en los argumentos de PowerShell.
```

**Escenario 10: Limpieza automática del proceso**
```gherkin
Dado que el volcado de la base de datos ha concluido exitosamente
Cuando el proceso hijo (sidecar) es destruido por el orquestador Tauri
Entonces la variable de entorno temporal "PGPASSWORD" desaparece de la memoria del sistema
Y no quedan rastros legibles de la contraseña en la memoria RAM compartida.
```

### Criterio: CA-02-3 (Emisión de Logs en Tiempo Real)

**Escenario 11: Emisión de logs estándar (Stdout)**
```gherkin
Dado que el comando de respaldo está corriendo
Cuando el motor de base de datos envía un progreso de "Respaldando tabla clientes (10%)"
Entonces Rust lee el stdout del sidecar
Y emite un evento "backup_log" a la ventana principal de React para que el usuario lo visualice de inmediato.
```

**Escenario 12: Emisión de errores (Stderr)**
```gherkin
Dado que ocurre una interrupción de red durante el respaldo
Cuando el motor de base de datos emite un mensaje de error fatal por Stderr
Entonces Rust captura esta cadena
Y la emite hacia React con una etiqueta roja indicando nivel de "ERROR".
```

### Criterio: CA-03-1 (Firma EOF en PostgreSQL)

**Escenario 13: Validación exitosa de pg_dump**
```gherkin
Dado que el archivo de respaldo de PostgreSQL ha sido generado en disco
Cuando la función verify_backup() inspecciona los últimos 256 bytes usando SeekFrom::End
Entonces encuentra la cadena "PostgreSQL database dump complete"
Y marca la bandera "verified" del backup_log como True.
```

**Escenario 14: Detección de archivo corrupto de PostgreSQL**
```gherkin
Dado que el usuario detuvo abruptamente la computadora a mitad del respaldo
Cuando la función verify_backup() inspecciona el archivo incompleto en el siguiente arranque
Entonces no logra encontrar la cadena "PostgreSQL database dump complete"
Y marca la bandera "verified" como False.
```

### Criterio: CA-03-2 (Firma EOF en MySQL)

**Escenario 15: Validación exitosa de mysqldump**
```gherkin
Dado que el archivo de respaldo de MySQL ha sido generado en disco
Cuando la función verify_backup() inspecciona los últimos 256 bytes
Entonces encuentra la cadena "Dump completed on" seguida de la fecha
Y marca la bandera "verified" del backup_log como True en la auditoría.
```

**Escenario 16: Archivo vacío por falta de permisos en MySQL**
```gherkin
Dado que el usuario no tiene permisos LOCK TABLES en MySQL
Cuando el archivo generado pesa 0 bytes
Entonces verify_backup() detecta el archivo anómalo
Y retorna un error de validación inmediato indicando "Archivo vacío o corrupto".
```

### Criterio: CA-03-3 (Generación inmutable de SHA-256)

**Escenario 17: Cálculo correcto en archivos grandes**
```gherkin
Dado que se ha completado un respaldo exitoso de 5 GB
Cuando el orquestador invoca la función calculate_hash_and_size
Entonces lee el archivo por bloques (chunks) de 8KB de manera eficiente
Y devuelve una cadena hexadecimal de 64 caracteres correspondiente al SHA-256.
```

**Escenario 18: Persistencia del Hash para auditoría**
```gherkin
Dado que el SHA-256 ha sido calculado tras la verificación
Cuando el ciclo de orquestación termina
Entonces se realiza un INSERT final en la tabla backup_logs
Y el valor del SHA-256 se guarda permanentemente para futuras comprobaciones de integridad.
```
