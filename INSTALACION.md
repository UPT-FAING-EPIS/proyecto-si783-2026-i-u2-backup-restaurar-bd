# Guía de Instalación y Ejecución en el VPS

Esta guía asume que tu servidor (VPS) ya tiene **Docker** instalado y funcionando.

## 1. Descarga de Imágenes para los Motores (Sandboxes)

La API levanta contenedores hijos (sandboxes) de manera dinámica para probar los backups. Para que el proceso sea rápido y la API no tenga que descargar las imágenes en el momento de la primera validación, es muy recomendable descargarlas (pull) previamente en tu VPS.

Ejecuta estos comandos en tu VPS para descargar las imágenes requeridas:

```bash
# PostgreSQL
sudo docker pull postgres:14-alpine

# MySQL
sudo docker pull mysql:8.0

# SQL Server
sudo docker pull mcr.microsoft.com/mssql/server:2022-latest

# MongoDB
sudo docker pull mongo:7
```

## 2. Construir la Imagen de la API

Copia o clona el código de tu proyecto (`safebridge para API`) en tu VPS. Luego ingresa a la carpeta del proyecto y construye la imagen de Docker para la API:

```bash
cd ruta/de/tu/proyecto
sudo docker build -t safebridge-api .
```

## 3. Ejecutar el Contenedor de la API

Para correr la API en Docker, necesitamos hacer dos cosas importantes:
1. Exponer el puerto `3000` (o el que uses) para poder recibir las peticiones.
2. Montar el **socket de Docker** (`/var/run/docker.sock`). Esto es vital porque la API necesita permisos para crear y eliminar otros contenedores Docker en tu VPS.

Ejecuta el siguiente comando para levantar la API en segundo plano (`-d`):

```bash
sudo docker run -d \
  --name safebridge_api \
  --restart always \
  -p 3000:3000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  safebridge-api
```

> **¡IMPORTANTE!** El volumen `-v /tmp:/tmp` es estrictamente necesario porque la API recibe los archivos por HTTP, los guarda temporalmente en su carpeta `/tmp`, y luego le pide a Docker que se los pase al Sandbox. Al compartir la carpeta `/tmp` entre el contenedor principal y el host, garantizamos que el archivo no se pierda en el proceso.

### Comandos Útiles

- **Ver si la API está corriendo:** `sudo docker ps`
- **Ver los registros (logs) de la API:** `sudo docker logs -f safebridge_api`
- **Detener la API:** `sudo docker stop safebridge_api`
- **Reiniciar la API:** `sudo docker restart safebridge_api`
