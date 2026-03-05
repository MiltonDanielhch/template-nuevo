# Justfile - Centro de Mando Código 3026
# Requiere instalar 'just' (cargo install just)

set shell := ["powershell.exe", "-c"]

# Auditoría de Código (Linting estricto según estándares)
audit:
    cargo clippy --workspace -- -D warnings

# Generación de Código Protobuf (ADN)
proto-gen:
    cd proto; buf generate

# Modo Desarrollo (Watch) - Requiere cargo-watch
dev:
    cargo watch -x 'run -p api_server'

# Compilación Release (Sintonía VPS $5)
build-release:
    cargo build --release