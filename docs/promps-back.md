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

✅ **RBAC (Roles y Permisos)**: Lógica en `core_logic`, repositorio en `infra_db` y middleware/endpoints en `api_server` completamente funcionales.

### Estructura Implementada

```
crates/
├── core_logic/
│   ├── domain/
│   │   ├── entities/     # User, Session
│   │   ├── value_objects/ # Email, UserId, SessionToken
│   │   └── interfaces/   # IUserRepository, IHasher, ISessionRepository
│   └── application/
│       └── use_cases/    # Register, Login, Update, ListUsers, CreateRole, AssignRole
├── infra_db/
│   └── persistence/sqlite/ # SqliteUserRepository, SqliteSessionRepository, SqliteRoleRepository
└── api_server/
    ├── config/di.rs      # Composition Root
    ├── entry_points/
    │   ├── api/v1/       # Handlers (Users CRUD, Roles CRUD)
    │   └── auth.rs       # Middleware (Extractor)
    │   └── middleware/rbac.rs # RBAC Guard (RequirePermission)
    └── routes.rs         # Router Definition con TraceLayer
```

---

## Próximos Pasos: ¡Sistema Base Completo!

El Bloque VI (RBAC) y Bloque VII (CRUD) están listos. El sistema es ahora robusto y extensible.

¡El sistema está estable y monitorizado!
