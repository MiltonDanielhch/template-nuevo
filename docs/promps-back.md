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
| **Telemetría** | Tracing + Rolling Files |
| **Despliegue** | Podman (Rootless) + Caddy |

### Estado Actual - ¡Todo Completado y Verificado!

✅ **Base de Datos Completa**: Todas las migraciones (RBAC, Audit, Users, Sessions) están listas.
✅ **Auth Funcional**: Registro, Login, Sessions, Logout y `/me` funcionan correctamente.
✅ **CRUD Usuarios (Backend Rust)**: Endpoints para Listar, Actualizar y Eliminar usuarios implementados.
✅ **Integración Frontend**: Astro llama al backend real de Rust. Se corrigió el mapeo de `username`.
✅ **Telemetría Avanzada**: Sistema de logs diarios persistentes en `logs/backend.log` con tracing de peticiones HTTP.
✅ **Corrección Crítica**: El extractor `CurrentUser` (Axum 0.8) ya recibe correctamente el `AppState`.
✅ **Despliegue Soberano**: `Dockerfile` optimizado + `podman-compose` + `Caddy`.

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
│       └── use_cases/    # RegisterUser, LoginUser, UpdateUser, ListUsers
├── infra_db/
│   └── persistence/sqlite/ # SqliteUserRepository, SqliteSessionRepository
└── api_server/
    ├── config/di.rs      # Composition Root
    ├── entry_points/
    │   ├── api/v1/       # Handlers (Users CRUD)
    │   └── auth.rs       # Middleware (Extractor)
    └── routes.rs         # Router Definition con TraceLayer
```

---

## Próximo Paso: Bloque VI - Gestión de Acceso (RBAC)

Ahora que tenemos una base sólida, vamos a implementar la lógica de roles.

1. **Entidades del Dominio** ⏳
   - Definir `Role` y `Permission` en `core_logic/domain/entities`.
   - Definir `IRoleRepository` en `core_logic/domain/interfaces`.

2. **Infraestructura** ⏳
   - Implementar `SqliteRoleRepository` en `infra_db`.

3. **Casos de Uso** ⏳
   - `CreateRole`, `AssignRoleToUser`.

4. **API y Middleware** ⏳
   - Crear un nuevo extractor `RequirePermission<P>`.

¡El sistema está estable y monitorizado!
