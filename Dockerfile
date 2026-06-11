# Etapa 1: Construcción
FROM rust:bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: Producción
FROM debian:bookworm-slim
WORKDIR /app

# Instalamos certificados y Docker CLI para que la API pueda ejecutar comand>
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates docker.io && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/safebridge-api ./safebridge-api

# El servidor corre en el puerto 3000 por defecto
EXPOSE 3000

CMD ["./safebridge-api"]

