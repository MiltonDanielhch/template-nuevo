Actúa como Ingeniero de Software Senior y Arquitecto del 'Laboratorio 3026'.

## Contexto del Proyecto

Estamos construyendo un sistema de autenticación y gestión de usuarios con una **Arquitectura Hexagonal estricta** en Rust.

### Stack Tecnológico

| Capa | Tecnología |
|------|------------|
| **Lenguaje** | Rust 2024 |
| **API** | Axum 0.8 |
| **Base de Datos** | SQLite (WAL) |
| **Hashing** | Argon2id |

### Estado Actual - ¡Ya Completado!

✅ **Base de Datos Completa**: Todas las migraciones (RBAC, Audit, Users, Sessions) están creadas y listas para correr.
✅ **Auth Funcional**: Registro, Login, Sessions, Logout y `/me` están implementados en código.
✅ **Infraestructura Core**: Workspace, DI, Error Handling y configuración listos.

### Estado Pendiente - Lógica Faltante (Bloque VI)

⚠️ **RBAC (Roles y Permisos)**: Aunque las tablas existen en la DB, **NO** hay lógica en `core_logic` (Entidades, Casos de Uso) ni endpoints en `api_server` para gestionar roles.

### Estructura Implementada

```
crates/
├── core_logic/
│   ├── domain/
│   │   ├── entities/     # User, Session
│   │   ├── value_objects/ # Email, UserId, SessionToken
│   │   └── interfaces/   # IUserRepository, IHasher, ISessionRepository
│   └── application/
│       └── use_cases/    # RegisterUser, LoginUser, CreateSession, GetUserById
├── infra_db/
│   └── persistence/sqlite/ # SqliteUserRepository, SqliteSessionRepository
└── api_server/
    ├── config/di.rs      # Composition Root
    ├── entry_points/
    │   ├── api/v1/       # Handlers (Solo Users)
    │   └── auth.rs       # Middleware (Extractor)
    └── routes.rs         # Router Definition
```

### Migraciones Creadas (Completas)

- 0001: RBAC (Roles, Permissions) - *Sin lógica asociada aún*
- 0002: Users
- 0003: Tokens
- 0004: Audit Logs
- 0005: Seed Data
- 0006: Sessions

---

## Próximo Paso: Bloque V - Despliegue Soberano (MVP Auth)

Vamos a desplegar el MVP de Autenticación para validar la arquitectura en producción antes de implementar la complejidad de RBAC.

1. **Containerización (Podman/Docker)** ⏳
   - Crear `Dockerfile` optimizado para Rust (Multi-stage build).
   - Usar `cargo-chef` para cachear dependencias.
   - Imagen final `distroless` o `alpine` para tamaño mínimo (< 50MB).

2. **Orquestación (Compose)** ⏳
   - Crear `deploy/compose.yml`.
   - Definir servicios: `app` (Backend) y `caddy` (Reverse Proxy).
   - Configurar volúmenes para la persistencia de SQLite (`backend.db`).

3. **Proxy Inverso (Caddy)** ⏳
   - Configurar `deploy/Caddyfile`.
   - HTTPS automático.
   - Redirección de tráfico al contenedor de la app.

---

## Trabajo Futuro: Bloque VI - Gestión de Acceso (RBAC)

Una vez desplegado el MVP, implementaremos la lógica faltante:
- Entidades `Role`, `Permission`.
- Repositorios `IRoleRepository`.
- Casos de uso `AssignRole`.
- Middleware `RequirePermission`.

¡Manos a la obra con el despliegue del MVP!

Resumen de pruebas de red (Caddy ↔ API):
Prueba	Resultado
API directo (8081/register)	✅ 200 OK
API directo (8081/login)	✅ 200 OK
API directo (8081/me)	❌ "Estado no disponible"
A través de Caddy (9080/register)	✅ 200 OK
A través de Caddy (9080/login)	✅ 200 OK
A través de Caddy (9080/me)	❌ "Estado no disponible"
La conectividad de red Caddy → API funciona correctamente. El error "Estado no disponible" es un bug en el código de Rust (en auth.rs:28), no un problema de red.
El problema está en que el AppState no se está pasando correctamente al extractor CurrentUser. Eso ya es un bug del código, no de la infraestructura.