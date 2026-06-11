---
marp: true
theme: uncover
class: invert
paginate: true
header: "**SafeBridge** | Sistema de Respaldo"
footer: "Presentación de Proyecto - 2026"
style: |
  @import url('https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;800&family=Inter:wght@300;400;500;600&display=swap');
  
  :root {
    --bg-color: #0f172a;
    --text-main: #f8fafc;
    --accent-1: #3b82f6; /* Blue */
    --accent-2: #06b6d4; /* Cyan */
    --card-bg: rgba(30, 41, 59, 0.7);
  }
  
  section {
    background: radial-gradient(circle at 100% 0%, #1e293b 0%, #0f172a 60%);
    color: var(--text-main);
    font-family: 'Inter', sans-serif;
    font-size: 26px; 
    padding: 60px 70px 85px 70px;
    justify-content: flex-start;
  }
  
  h1, h2, h3, h4 {
    font-family: 'Outfit', sans-serif;
    margin-top: 0;
  }
  
  h1 {
    font-size: 3.2em;
    font-weight: 800;
    background: -webkit-linear-gradient(45deg, var(--accent-1), var(--accent-2));
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin-bottom: 0.1em;
  }
  
  h2 {
    font-size: 1.8em;
    color: #e2e8f0;
    border-bottom: 2px solid rgba(59, 130, 246, 0.3);
    padding-bottom: 10px;
    margin-bottom: 20px;
  }
  
  h3 {
    color: var(--accent-2);
    font-size: 1.25em;
    margin-bottom: 10px;
  }
  
  p, li {
    line-height: 1.5;
    color: #cbd5e1;
    margin-bottom: 10px;
  }
  
  ul, ol {
    margin-top: 10px;
  }
  
  li {
    margin-bottom: 8px;
  }
  
  li strong {
    color: #93c5fd;
  }
  
  .grid-2 {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px; 
    width: 100%;
  }
  
  .card {
    background: var(--card-bg);
    border: 1px solid rgba(59, 130, 246, 0.2);
    border-radius: 16px;
    padding: 20px; 
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(12px);
  }
  
  .card h3 {
    margin-top: 0;
    color: #fff;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  
  .card p {
    margin: 0;
    font-size: 0.9em;
  }
  
  code {
    background: rgba(0, 0, 0, 0.6);
    color: #34d399;
    padding: 3px 8px;
    border-radius: 6px;
    font-family: 'Consolas', monospace;
    font-size: 0.85em;
    border: 1px solid rgba(52, 211, 153, 0.2);
  }
  
  mark {
    background: transparent;
    color: #fbbf24;
    font-weight: 600;
  }
---

<!-- _class: lead -->
<!-- _backgroundColor: #0f172a -->
<!-- _backgroundImage: "radial-gradient(circle at 50% 50%, #1e293b 0%, #020617 100%)" -->
<!-- _header: "" -->
<!-- _footer: "" -->

# SafeBridge
### Tranquilidad para tu Información

Generación y **Verificación** de Respaldos de Bases de Datos.

---

## 🎯 ¿Para qué sirve SafeBridge?

Tener un archivo de respaldo no sirve de nada si a la hora de una emergencia el archivo está corrupto o incompleto.

SafeBridge **no solo genera los backups**, sino que **verifica automáticamente** que estén correctos y funcionales. Todo esto desde una aplicación de escritorio sencilla y segura.

---

## 🛠️ ¿Qué estamos implementando?

Estamos construyendo una solución de escritorio que ofrece:

<div class="grid-2" style="margin-top: 15px;">
  <div class="card">
    <h3>🌐 Múltiples Motores</h3>
    <p>Soporte unificado para extraer datos de <strong>PostgreSQL, MySQL, SQL Server y MongoDB</strong> desde un solo lugar.</p>
  </div>
  <div class="card">
    <h3>🚫 Independencia</h3>
    <p>Funciona por sí sola. No requiere instalaciones complejas en la red de tu empresa (como Docker o contenedores).</p>
  </div>
  <div class="card">
    <h3>🛡️ Seguridad Local</h3>
    <p>Tus contraseñas jamás viajan por internet. Todo se encripta y se protege dentro de tu propia computadora.</p>
  </div>
  <div class="card">
    <h3>📊 Historial Claro</h3>
    <p>Un registro visual que te dice de inmediato si tus respaldos de esta semana funcionaron o fallaron.</p>
  </div>
</div>

---

## 💻 Tecnologías que Utilizamos

Mantenemos las cosas modernas pero estables combinando estas tecnologías:

1. **React:** Crea la interfaz gráfica moderna, amigable y fluida que el usuario ve y controla.
2. **Rust:** El motor invisible detrás de todo. Garantiza que la extracción de datos sea rápida y sin agujeros de seguridad.
3. **Tauri:** La herramienta que une la interfaz gráfica con el motor interno para empaquetarlo todo como un programa tradicional (un ejecutable `.exe`).

---

## ✅ ¿Cómo funciona la Verificación?

¿Cómo sabe SafeBridge que un archivo está "sano"?

<div class="card" style="margin-top: 20px;">
  <ul style="margin: 0; padding-left: 20px; font-size: 0.95em;">
    <li><strong>1. Lectura de Firmas de Éxito:</strong> SafeBridge analiza el interior del archivo de respaldo buscando el mensaje final que deja la base de datos <em>(ej: "Dump completed successfully")</em>. Si no encuentra ese mensaje, sabe que el proceso falló a la mitad.</li>
    <br>
    <li><strong>2. Huella Digital:</strong> Al terminar, se le asigna un código único al archivo. Si alguien modifica el archivo después, o este se corrompe en el disco duro, la huella cambiará y la aplicación te avisará del problema.</li>
  </ul>
</div>

---

<!-- _backgroundImage: "radial-gradient(circle at top right, #172554 0%, #0f172a 100%)" -->

## 📋 Logs Transparentes

En SafeBridge, **no hay cajas negras**. Si un respaldo falla, necesitas saber exactamente el porqué.

- **Historial Completo:** Puedes consultar cualquier respaldo que hayas intentado en el pasado.
- **Visor de Consola (Logs):** Con un clic, puedes leer el texto exacto del error que arrojó la base de datos (por ejemplo: *"Contraseña incorrecta"* o *"Sin espacio en el disco"*).
- **Resolución Directa:** Al no ocultar los errores reales, solucionar los problemas de infraestructura se vuelve un proceso mucho más rápido.

---

<!-- _class: lead -->
<!-- _backgroundImage: "radial-gradient(circle at 50% 50%, #1e3a8a 0%, #020617 100%)" -->
<!-- _header: "" -->
<!-- _footer: "" -->

# 🚀 SafeBridge

### Tus datos, asegurados y verificados.

<p style="font-size: 0.6em; color: #94a3b8; margin-top: 40px;">Listos para proteger tu información.</p>
