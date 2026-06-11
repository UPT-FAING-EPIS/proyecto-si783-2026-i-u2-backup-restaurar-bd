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

Documentación Técnica (DocFX) y Manual de Usuario — FD05

Versión *2.0*

| CONTROL DE VERSIONES | | | | | |
|:---:|:---|:---|:---|:---|:---|
| Versión | Hecha por | Revisada por | Aprobada por | Fecha | Motivo |
| 1.0 | IASR / JSCM | Ing. P. Cuadros | Ing. P. Cuadros | 11/06/2026 | Versión Original: Generación Técnica DocFX simulada y Manual visual |

<div style="page-break-after: always; visibility: hidden"></div>

# ÍNDICE GENERAL

- [1. Documentación Técnica de la API (Estilo DocFX)](#1-documentación-técnica-de-la-api-estilo-docfx)
  - [1.1. Módulo: `crypto.rs`](#11-módulo-cryptors)
  - [1.2. Módulo: `backup.rs`](#12-módulo-backuprs)
  - [1.3. Módulo: `connections.rs`](#13-módulo-connectionsrs)
- [2. Manual de Usuario (Trazas de UI)](#2-manual-de-usuario-trazas-de-ui)
  - [2.1. Panel Principal (Dashboard)](#21-panel-principal-dashboard)
  - [2.2. Gestión de Conexiones](#22-gestión-de-conexiones)
  - [2.3. Generación de Respaldos](#23-generación-de-respaldos)
  - [2.4. Historial y Auditoría](#24-historial-y-auditoría)

<div style="page-break-after: always; visibility: hidden"></div>

---

## 1. Documentación Técnica de la API (Estilo DocFX)

*Esta sección representa la exportación técnica generada automáticamente por herramientas de documentación de código como `cargo doc` (Rust) y `typedoc` / DocFX. Se detalla la especificación de los métodos clave.*

### 1.1. Módulo: `crypto.rs`

Provee las utilidades criptográficas para asegurar las credenciales de la base de datos en almacenamiento local.

#### `encrypt_password(password: &str) -> Result<String, CryptoError>`
Cifra una contraseña en texto plano utilizando el algoritmo AES-256-GCM y una clave derivada de forma segura.

**Parámetros:**
- `password` (`&str`): La contraseña de la base de datos en texto plano.

**Retorna:**
- `Result<String, CryptoError>`: Una cadena hexadecimal que representa el vector de inicialización (IV) concatenado con el texto cifrado, o un error si el cifrado falla.

**Excepciones:**
Lanza `CryptoError::KeyNotFound` si la llave maestra no está disponible en las variables de entorno o el Keyring del sistema operativo.

---

### 1.2. Módulo: `backup.rs`

Orquestador principal de subprocesos y analizador de archivos resultantes.

#### `verify_backup(file_path: &Path, engine: DbEngine) -> Result<bool, IOError>`
Lee los últimos bytes de un archivo generado para asegurar que la herramienta de volcado nativa (ej: `pg_dump`) escribió su firma de conclusión.

**Parámetros:**
- `file_path` (`&Path`): Ruta absoluta hacia el archivo `.sql`, `.bak` o `.archive`.
- `engine` (`DbEngine`): El motor específico para aplicar la heurística correcta.

**Retorna:**
- `Result<bool, IOError>`: `true` si la firma es encontrada. `false` si el archivo parece truncado o corrupto.

#### `calculate_hash_and_size(file_path: &Path) -> Result<(String, u64), IOError>`
Calcula de manera eficiente el hash criptográfico empleando un buffer asíncrono para no saturar la RAM.

**Retorna:**
Una tupla conteniendo el String (Hash Hexadecimal SHA-256) y el peso físico en disco (bytes).

---

### 1.3. Módulo: `connections.rs`

Maneja la persistencia en SQLite para la información de conectividad.

#### `test_connection(host: &str, port: u16) -> Result<bool, NetworkError>`
Intenta abrir un socket TCP hacia el objetivo utilizando `TcpStream`.

**Parámetros:**
- `host` (`&str`): IP o Dominio.
- `port` (`u16`): Puerto de red.

**Comportamiento:**
Implementa un Timeout agresivo de 3000ms. Retorna `false` inmediatamente si la red no alcanza el host, evitando que el usuario bloquee la interfaz de React.

<div style="page-break-after: always; visibility: hidden"></div>

---

## 2. Manual de Usuario (Trazas de UI)

*Este manual ha sido elaborado basándose en las trazas y flujos visuales capturados durante las pruebas automatizadas de Interfaz de Usuario (UI).*

### 2.1. Panel Principal (Dashboard)

El **Dashboard** es la pantalla principal que aparece tras iniciar SafeBridge. Provee un vistazo rápido a la salud de sus respaldos.

- **Indicadores Clave:** En la parte superior observará tres tarjetas que muestran el "Total de Conexiones", "Respaldos Exitosos" y "Respaldos Fallidos".
- **Comportamiento:** Si un respaldo reciente ha fallado, la tarjeta se iluminará en rojo para llamar su atención inmediata.

### 2.2. Gestión de Conexiones

Para respaldar una base de datos, primero debe decirle a SafeBridge cómo llegar a ella.

1. Navegue a la pestaña **Conexiones** en el panel lateral izquierdo.
2. Haga clic en el botón azul **"Nueva Conexión"**.
3. **Formulario:** Seleccione su motor (PostgreSQL, MySQL, etc.). Ingrese la dirección IP (Host), Puerto, Usuario y Contraseña.
4. **Probar:** Antes de guardar, presione **Test Connection**. El sistema verificará de fondo si el servidor está en línea.
5. Presione **Guardar**. *Nota:* Su contraseña se cifra localmente mediante AES-256; ni siquiera los administradores del sistema podrán verla en texto plano.

### 2.3. Generación de Respaldos

El flujo principal para generar una copia de seguridad segura:

1. Diríjase a la sección **Generar Backup**.
2. Utilice el menú desplegable para seleccionar una de las conexiones creadas en el paso anterior.
3. Pulse **Iniciar Volcado**.
4. **Terminal en Vivo:** Debajo del botón, verá una ventana negra simulando una terminal. SafeBridge mostrará el avance del proceso en tiempo real. 
5. Al concluir, el sistema evaluará internamente el archivo resultante y calculará su SHA-256.

### 2.4. Historial y Auditoría

Si desea saber qué ocurrió con un respaldo pasado o necesita extraer la firma Hash para entregarla a un auditor de seguridad:

1. Navegue a **Historial**.
2. Podrá observar una tabla con la fecha, la duración, el tamaño en bytes y un estado visual verde (`OK`) o rojo (`FAIL`).
3. La columna **Verificado EOF** le asegurará que el archivo descargado finalizó correctamente y no fue corrompido durante la escritura en el disco duro.
4. La columna **SHA-256** le proveerá el hash inmutable. Si el archivo `.sql` resultante cambia un solo byte en el futuro, usted podrá saberlo comparando este hash.
