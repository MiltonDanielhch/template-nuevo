# 🛠️ ROADMAP: BACKEND 3026 (Versión Workspace & Hexagonal)

**Stack:** Rust 2024 | Axum 0.8 | SQLx 0.8 (SQLite) | Protobuf (ConnectRPC) | Podman

---

## 🏗️ BLOQUE 0: EL MOTOR (Tooling & Workspace)

**Objetivo:** Configurar el entorno de desarrollo multi-crate para máxima modularidad.

| # | Tarea | Ubicación / Comando | Estado |
|---|-------|---------------------|:------:|
| 1 | Workspace Init | `Cargo.toml` raíz + carpetas `crates/` | ✅ |
| 2 | ADN Setup | `proto/auth.proto` + `buf.yaml` | ✅ |
| 3 | Just Command | `Justfile` (comandos: audit, gen-proto, migrate) | ✅ |
| 4 | Sintonía Release | Perfil `z` (minificación) en `Cargo.toml` raíz | ✅ |
| 5 | Modernización | Update Axum 0.8, Tokio 1.43, SQLx 0.8 | ✅ |

---

## 🏗️ BLOQUE I: FUNDACIÓN (Persistencia y Dominio)

**Objetivo:** Implementar la Capa 1 (Dominio) y Capa 3 (Infraestructura SQL).

### 📅 Fase 1.1: ADN SQL (Migraciones)

**Por qué:** La base de datos es la memoria versionada del sistema. Debe ser tratada como código.

| # | Tarea | Descripción | Estado |
|---|-------|-------------|:------:|
| 1 | Crear RBAC | `0001_create_rbac.sql`: Tablas `roles`, `permissions`, `role_permissions`. | ✅ |
| 2 | Crear Usuarios | `0002_create_users.sql`: Tabla `users` con Soft Delete y trigger `updated_at`. | ✅ |
| 3 | Crear Tokens | `0003_create_tokens.sql`: Tabla `tokens` para verificación y reseteo. | ✅ |
| 4 | Crear Auditoría | `0004_create_audit.sql`: Tabla `audit_logs` para trazabilidad. | ✅ |
| 5 | Seed de Datos | `0005_seed_system_data.sql`: Roles y permisos por defecto. | ✅ |
| 6 | SQLx Prep | `just db-prepare`: Generar `sqlx-data.json` para compilación offline. | ✅ |

**Verificación:** `just db-migrate` se completa sin errores.

### 📅 Fase 1.2: El Corazón Inmortal (core_logic)

**Por qué:** Los `Traits` son contratos. El `core` define "qué" necesita, sin saber "cómo" se implementa. Esto nos da libertad total para cambiar la base de datos en el futuro.

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | Value Objects | `core_logic/domain/value_objects` | `UserId`, `Email`, `PasswordHash` con validación estricta y `Display`. | ✅ |
| 2 | Domain Traits | `core_logic/domain/interfaces` | Puertos: `IUserRepository` (✅), `IHasher` (✅) | ✅ |
| 3 | Use Cases | `core_logic/application/use_cases` | Lógica de `RegisterUser` (✅), `LoginUser` (✅) | ✅ |

**Verificación:** `cargo check -p core_logic` pasa sin errores.

### 📅 Fase 1.3: El Adaptador Concreto (infra_db)

**Por qué:** El adaptador es el traductor. Implementa el `trait` del `core` y sabe hablar el lenguaje específico de SQLite, convirtiendo datos crudos en entidades de dominio ricas.

| # | Tarea | Descripción | Estado |
|---|-------|-------------|:------:|
| 1 | Crear Crate `infra_db` | Añadir el crate al workspace y definir sus dependencias. | ✅ |
| 2 | Implementar Repositorio | Crear `SqliteUserRepository` que implemente el trait `IUserRepository`. | ✅ |
| 3 | Crear DTO de DB | Definir `DbUser` para mapear la tabla `users`, aceptando tipos crudos de la DB. | ✅ |
| 4 | Implementar Mappers | Crear `from_domain` y `to_domain_user` para traducir entre `User` y `DbUser` de forma explícita. | ✅ |
| 5 | **Sintonía de Compilación** | **Punto de control:** Ejecutar `just db-prepare` y `just audit` hasta obtener `Finished` sin errores. Consultar `TROUBLESHOOTING.md` para errores comunes. | ✅ |

**Verificación:** `just audit` se completa sin errores en todo el workspace.

---

## 🔒 BLOQUE II: EL ESCUDO (Seguridad y Protocolo)

**Objetivo:** Implementar hashing Argon2id y un sistema de sesiones seguro.

### 📅 Fase 2.1: Contratos Binarios

| # | Tarea | Descripción | Estado |
|---|-------|-------------|--------|
| 1 | Proto Gen | Generar código Rust desde `.proto` usando buf | ⏳ |
| 2 | Mappers | Traductores entre Proto Structs <-> Domain Entities | ⏳ |

### 📅 Fase 2.2: Identidad 3026

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | Hashing Service | `infra_db` | Implementar `IHasher` con Argon2id. | ✅ |
| 2 | RegisterUser Use Case | `core_logic/application/use_cases/user/register.rs` | Caso de uso para registro con verificación de email duplicado. | ✅ |
| 3 | Register API Endpoint | `api_server/entry_points/api/v1/user_handlers.rs` | Handler `POST /register`. | ✅ |
| 4 | LoginUser Use Case | `core_logic/application/use_cases/user/login.rs` | Caso de uso para autenticación con email/password. | ✅ |
| 5 | Login API Endpoint | `api_server/entry_points/api/v1/user_handlers.rs` | Handler `POST /login` retorna token. | ✅ |
| 6 | Tests de Integración | `api_server/tests/integration_tests.rs` | Tests para register y login. | ✅ |
| 7 | Session Tokens | `infra_db` + `core_logic` | Sistema de tokens de sesión. | ✅ |
| 8 | Migraciones | `infra_db/migrations` | RBAC, Users, Tokens, Audit, Sessions. | ✅ |
| 9 | **Telemetría (Logging)** | `api_server` | Sistema de logs diarios persistentes con tracing y rotación. | ✅ |

---

## ⚙️ BLOQUE III: LA ANTENA (API & Entry Points)

**Objetivo:** Exponer el sistema al mundo exterior mediante `api_server`.
**Estado:** ✅ Completado

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | AppState | `api_server` | Struct con `Arc<dyn IUserRepository>`. | ✅ |
| 2 | DI Container | `api_server/config/di.rs` | Unir `SqliteUserRepository` con `IUserRepository`. | ✅ |
| 3 | Axum Router | `api_server/routes.rs` | Configuración de rutas, `State` y `TraceLayer`. | ✅ |
| 4 | Handlers CRUD | `api_server/v1/user_handlers.rs` | Registro, Login, Listar, Editar y Eliminar usuarios. | ✅ |
| 5 | Value Object Display | `core_logic/domain/value_objects` | Implementar `Display` para `UserId`, `Email`. | ✅ |
| 6 | DomainError | `core_logic/domain/errors` | Variante `UserAlreadyExists`. | ✅ |
| 7 | Email Duplicate Check | `core_logic/application/use_cases/user/register.rs` | Verificar email antes de guardar. | ✅ |
| 8 | Tests de Integración | `api_server/tests/integration_tests.rs` | Tests `register_user_success` y `register_user_duplicate_email`. | ✅ |

---

## 🔐 BLOQUE IV: SESIONES Y AUTH

**Objetivo:** Implementar sistema de tokens para mantener autenticación entre requests.
**Estado:** ✅ Completado

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | Migraciones | `infra_db/migrations` | Tabla `sessions` con tokens. | ✅ |
| 2 | Value Object SessionToken | `core_logic/domain/value_objects` | Token con validación. | ✅ |
| 3 | ISessionRepository | `core_logic/domain/interfaces` | Trait para persistencia. | ✅ |
| 4 | SqliteSessionRepository | `infra_db/persistence` | Implementación SQLite. | ✅ |
| 5 | CreateSession Use Case | `core_logic/application/use_cases` | Generar token al hacer login. | ✅ |
| 6 | Login retorna Token | `api_server/entry_points` | `POST /login` retorna token. | ✅ |
| 7 | Auth Middleware | `api_server/entry_points/auth.rs` | Extractor CurrentUser sin macro async_trait. | ✅ |
| 8 | GetUserById Use Case | `core_logic/application/use_cases` | Caso de uso para buscar usuario por ID. | ✅ |
| 9 | GET /me | `api_server/routes.rs` | Ruta protegida que retorna datos del usuario. | ✅ |
| 10 | Logout | `api_server/routes.rs` | `POST /logout`. | ✅ |
| 11 | Tests de Integración | `api_server/tests/integration_tests.rs` | Tests para /me y /logout. | ✅ |

---

## 📡 BLOQUE V: DESPLIEGUE SOBERANO (MVP)

**Objetivo:** Poner el laboratorio en órbita en el VPS de $5 (Solo Auth).
**Estado:** ✅ Completado

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | Podman Pod | `deploy/podman-compose.yml` | Orquestación Rootless (App + Caddy). | ✅ |
| 2 | Dockerfile | `deploy/Dockerfile` | Multi-stage build con cargo-chef. | ✅ |
| 3 | Caddy SSL | `deploy/Caddyfile` | HTTPS automático con HTTP/3. | ✅ |
| 4 | Kamal Config | `config/deploy.yml` | Automatización del despliegue. | ⏳ |

---

## 🛡️ BLOQUE VI: GESTIÓN DE ACCESO (RBAC)

**Objetivo:** Implementar la lógica de Roles y Permisos (ya existen en DB).
**Estado:** ✅ Completado

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | Entidades Dominio | `core_logic/domain/entities` | `Role`, `Permission` | ✅ |
| 2 | Repositorio Roles | `core_logic` + `infra_db` | `IRoleRepository` y `SqliteRoleRepository` | ✅ |
| 3 | Casos de Uso | `core_logic/application` | `CreateRole`, `AssignRoleToUser` | ✅ |
| 4 | Middleware RBAC | `api_server/entry_points` | Extractor `RequirePermission<P>` | ✅ |
| 5 | API Endpoints | `api_server/api/v1` | CRUD de Roles y asignación | ✅ |

---

## 👥 BLOQUE VII: SOBERANÍA DE DATOS (CRUD de Usuarios)

**Objetivo:** Implementar la gestión completa de usuarios en el backend real.
**Estado:** ✅ Completado

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|:------:|
| 1 | IUserRepository Ext | `core_logic` | Añadir `find_all` y `delete` al contrato. | ✅ |
| 2 | SQLite Repo Ext | `infra_db` | Implementar `find_all` y `delete` con Soft Delete. | ✅ |
| 3 | Use Cases CRUD | `core_logic` | `ListUsers`, `UpdateUser`, `DeleteUser`. | ✅ |
| 4 | API Handlers | `api_server` | Endpoints `GET /users`, `PUT /users/:id`, `DELETE /users/:id`. | ✅ |
| 5 | Auth Guards | `api_server` | Protección de rutas CRUD mediante `CurrentUser`. | ✅ |
