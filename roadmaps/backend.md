# 🛠️ ROADMAP: BACKEND 3026 (Versión Workspace & Hexagonal)

**Stack:** Rust 2024 | Axum | SQLx (SQLite) | Protobuf (ConnectRPC) | Podman

---

## 🏗️ BLOQUE 0: EL MOTOR (Tooling & Workspace)

**Objetivo:** Configurar el entorno de desarrollo multi-crate para máxima modularidad.

| # | Tarea | Ubicación / Comando | Estado |
|---|-------|---------------------|--------|
| 1 | Workspace Init | `Cargo.toml` raíz + carpetas `crates/` | ⏳ |
| 2 | ADN Setup | `proto/auth.proto` + `buf.yaml` | ⏳ |
| 3 | Just Command | `Justfile` (comandos: audit, gen-proto, migrate) | ⏳ |
| 4 | Sintonía Release | Perfil `z` (minificación) en `Cargo.toml` raíz | ⏳ |

---

## 🏗️ BLOQUE I: FUNDACIÓN (Persistencia y Dominio)

**Objetivo:** Implementar la Capa 1 (Dominio) y Capa 3 (Infraestructura SQL).

### 📅 Fase 1.1: ADN SQL (Migraciones)

Mantenemos SQLite WAL para el VPS de $5.

| # | Tarea | Descripción | Estado |
|---|-------|-------------|--------|
| 1 | RBAC Base | Tablas `roles`, `permissions` y `users` (UUIDv7) | ⏳ |
| 2 | Audit Log | Tabla para trazabilidad total de cambios | ⏳ |
| 3 | SQLx Prep | Configurar `.env` y preparar `sqlx-data.json` para compilación offline | ⏳ |

### 📅 Fase 1.2: El Corazón Inmortal (core_logic)

Aquí reside la inteligencia pura, sin dependencias de base de datos.

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|--------|
| 1 | Value Objects | `core_logic/domain/value_objects` | `Email`, `Password` (validación estricta) | ⏳ |
| 2 | Domain Traits | `core_logic/domain/interfaces` | Puertos: `IUserRepository`, `IHasher` | ⏳ |
| 3 | Use Cases | `core_logic/application/use_cases` | Lógica de `RegisterUser`, `LoginUser` | ⏳ |

---

## 🔒 BLOQUE II: EL ESCUDO (Seguridad y Protocolo)

**Objetivo:** Implementar ConnectRPC (Protobuf) y Seguridad Rootless.

### 📅 Fase 2.1: Contratos Binarios

| # | Tarea | Descripción | Estado |
|---|-------|-------------|--------|
| 1 | Proto Gen | Generar código Rust desde `.proto` usando buf | ⏳ |
| 2 | Mappers | Traductores entre Proto Structs &lt;-&gt; Domain Entities | ⏳ |

### 📅 Fase 2.2: Identidad 3026

| # | Tarea | Descripción | Estado |
|---|-------|-------------|--------|
| 1 | Argon2id | Implementación de hashing en `infra_db` | ⏳ |
| 2 | Session Layer | JWT o Sesiones en SQLite con rotación de llaves | ⏳ |

---

## ⚙️ BLOQUE III: LA ANTENA (API & Entry Points)

**Objetivo:** Exponer el sistema al mundo exterior mediante `api_server`.

| # | Tarea | Descripción | Estado |
|---|-------|-------------|--------|
| 1 | Axum Router | Configuración de rutas en `api_server/routes.rs` | ⏳ |
| 2 | DI Container | Inyección de dependencias en `config/di.rs` (unir Core con Infra) | ⏳ |
| 3 | Scalar Doc | Documentación automática desde el código | ⏳ |

---

## 📡 BLOQUE IV: DESPLIEGUE SOBERANO

**Objetivo:** Poner el laboratorio en órbita en el VPS de $5.

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|--------|
| 1 | Podman Pod | `deploy/podman-compose.yml` | Orquestación Rootless (App + Caddy) | ⏳ |
| 2 | Kamal Config | `config/deploy.yml` | Automatización del despliegue | ⏳ |
| 3 | Caddy SSL | `deploy/Caddyfile` | HTTPS automático con HTTP/3 | ⏳ |

---

## 🔧 Comandos Maestro (Justfile sugerido)

```just
# Código 3026 - Centro de Mando

audit:
    python3 ver-proyecto.py

proto-gen:
    buf generate

dev:
    cargo watch -x 'run -p api_server'

db-migrate:
    sqlx migrate run --source crates/infra_db/migrations