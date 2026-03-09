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
✅ **RBAC (Roles y Permisos)**: Lógica en `core_logic`, repositorio en `infra_db` y middleware/endpoints en `api_server` completamente funcionales.
✅ **Gestión de Perfil**: Endpoints para que el usuario actualice su propia información (`PUT /me`) implementados y verificados.

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

## Próximos Pasos: Seguridad Avanzada y Notificaciones

1. **Auditoría Estricta**: Middleware para registrar acciones sensibles (cambio de password, login, eliminación) en `audit_logs`.
2. **Seguridad Avanzada**: Implementar Rate Limiting por IP para evitar ataques de fuerza bruta.
3. **Notificaciones**: Sistema base para envío de correos (Verificación de cuenta, Reseteo de contraseña).
4. **Health Extended**: Endpoints de salud más detallados (uso de memoria, conexiones DB).
