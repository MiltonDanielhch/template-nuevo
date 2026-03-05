# Justfile - Centro de Mando Código 3026
# Requiere instalar 'just' (cargo install just)

set shell := ["powershell.exe", "-NoProfile", "-Command"]

# Instala las herramientas de desarrollo necesarias para la sintonía
setup:
    @echo "--- 🛠️ Sintonizando el entorno de desarrollo 3026... ---"
    @echo "-> Instalando herramientas de Rust (watch, sqlx-cli)..."
    cargo install cargo-watch sqlx-cli
    @echo "-> Instalando pre-commit para los guardianes de código..."
    pip install pre-commit
    @echo "-> Activando guardianes de pre-commit..."
    pre-commit install
    @echo "✅ ¡Entorno listo! Ahora puedes usar 'just dev' y los commits serán auditados."

# Auditoría de Código (Linting estricto según estándares)
audit:
    cargo clippy --workspace -- -D warnings

# Formateo de Código (Frontend + Backend)
format:
    biome check --write --unsafe .
    cargo fmt --all

# Generación de Código Protobuf (ADN)
proto-gen:
    cd proto; buf generate

# Modo Desarrollo (Watch) - Requiere cargo-watch
dev:
    cargo watch -x 'check --workspace'

# Compilación Release (Sintonía VPS $5)
build-release:
    cargo build --release

# Prepara las consultas SQLx para compilación offline
db-prepare: db-setup
    cargo sqlx prepare --workspace --database-url "sqlite://backend.db"

# Crea el archivo de la base de datos si no existe
db-setup:
    sqlx database create --database-url "sqlite://backend.db"

# Ejecuta las migraciones de la base de datos (depende de db-setup)
db-migrate: db-setup
    sqlx migrate run --source crates/infra_db/migrations --database-url "sqlite://backend.db"