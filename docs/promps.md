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
| **Despliegue** | Podman (Rootless) + Caddy |

### Estado Actual - ¡Todo Completado y Verificado!

✅ **Base de Datos Completa**: Todas las migraciones (RBAC, Audit, Users, Sessions) están listas.
✅ **Auth Funcional**: Registro, Login, Sessions, Logout y `/me` funcionan correctamente.
✅ **Corrección Crítica**: El extractor `CurrentUser` (Axum 0.8) ya recibe correctamente el `AppState`, solucionando el error "Estado no disponible".
✅ **Despliegue Soberano**:
   - `Dockerfile` optimizado (cargo-chef + distroless).
   - `podman-compose` con persistencia de datos.
   - `Caddy` como proxy inverso con HTTPS automático.

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
    │   └── auth.rs       # Middleware (Extractor) - ¡Corregido!
    └── routes.rs         # Router Definition
```

---

## Próximo Paso: Bloque VI - Gestión de Acceso (RBAC)

Ahora que tenemos una base sólida y desplegada, vamos a implementar la lógica de roles.

1. **Entidades del Dominio** ⏳
   - Definir `Role` y `Permission` en `core_logic/domain/entities`.
   - Definir `IRoleRepository` en `core_logic/domain/interfaces`.

2. **Infraestructura** ⏳
   - Implementar `SqliteRoleRepository` en `infra_db`.
   - Mappers para convertir de SQL a Dominio.

3. **Casos de Uso** ⏳
   - `CreateRole`, `AssignRoleToUser`.
   - `GetUserPermissions`.

4. **API y Middleware** ⏳
   - Crear un nuevo extractor `RequirePermission<P>`.
   - Endpoints para gestión de roles (Solo Admin).

¡El sistema está estable y listo para crecer!
