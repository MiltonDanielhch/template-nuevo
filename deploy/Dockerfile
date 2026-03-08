# Etapa 1: Planificación (Caching de Dependencias)
# Usamos cargo-chef para pre-calcular las dependencias y cachearlas.
# Esto acelera dramáticamente los builds subsiguientes.
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Etapa 2: Construcción (Compilación Real)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Construimos solo las dependencias (se cachea esta capa)
RUN cargo chef cook --release --recipe-path recipe.json

# Copiamos el código fuente
COPY . .
# Compilamos el binario final
# Usamos --release para máxima optimización
RUN cargo build --release --bin api_server

# Preparamos la estructura de carpetas para el runtime
# Esto es vital porque Distroless no tiene 'mkdir'
RUN mkdir -p /app/data && mkdir -p /app/migrations

# Etapa 3: Runtime (Imagen Final Minúscula)
# Usamos distroless cc-debian12 para tener solo lo mínimo necesario (glibc, ssl).
# Es más seguro (sin shell, sin gestor de paquetes) y ligero.
FROM gcr.io/distroless/cc-debian12

# Copiamos el binario desde la etapa de construcción
COPY --from=builder /app/target/release/api_server /usr/local/bin/api_server

# Copiamos la estructura de carpetas vacía para asegurar que existe
# Esto permite que SQLite escriba en /app/data sin problemas
COPY --from=builder /app/data /app/data

# Copiamos las migraciones y datos necesarios para SQLx/Runtime
COPY --from=builder /app/crates/infra_db/migrations /app/migrations
#COPY --from=builder /app/config /app/config
# (Opcional) Si usas un .env en tiempo de ejecución, pero idealmente se pasan como variables de entorno

WORKDIR /app
ENV DATABASE_URL="sqlite:///app/data/backend.db?mode=rwc"

# Exponemos el puerto
EXPOSE 8080

# Comando de inicio
CMD ["api_server"]
