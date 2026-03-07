Actúa como Ingeniero de Software Senior y Arquitecto del 'Laboratorio 3026'.

## Contexto del Proyecto

Estamos construyendo un sistema de autenticación y gestión de usuarios con una **Arquitectura Hexagonal estricta** en Rust.

### Stack Tecnológico

| Capa | Tecnología |
|------|------------|
| **Lenguaje** | Rust 2024 |
| **API** | Axum |
| **Base de Datos** | SQLite (WAL) |
| **Hashing** | Argon2id |

### Estado Actual - ¡Ya Completado!

✅ **Registro de Usuario (POST /register)**
✅ **Login de Usuario (POST /login)** - Retorna token de sesión
✅ **Sistema de Sesiones** - Sessions, RBAC, Tokens, Audit

### Estructura Implementada

```
crates/
├── core_logic/
│   ├── domain/
│   │   ├── entities/     # User, Session
│   │   ├── value_objects/ # Email, UserId, SessionToken
│   │   └── interfaces/   # IUserRepository, IHasher, ISessionRepository
│   └── application/
│       └── use_cases/    # RegisterUser, LoginUser, CreateSession
├── infra_db/
│   └── persistence/sqlite/ # SqliteUserRepository, SqliteSessionRepository
└── api_server/
    ├── config/di.rs      # Composition Root
    └── entry_points/    # Handlers
```

### Migraciones Creadas

- 0001: RBAC (roles, permissions, role_permissions)
- 0002: Users (con Soft Delete)
- 0003: Tokens
- 0004: Audit Logs
- 0005: Seed Data
- 0006: Sessions

### Value Objects Implementados

- **Email**: Validación de formato con regex, Display
- **UserId**: UUIDv7, Display
- **PasswordHash**: Wrapper seguro
- **SessionToken**: Token de sesión

---

## Próximo Paso: Proteger Rutas con Auth

Ahora que tenemos el sistema de sesiones:

1. **Middleware de Autenticación** ✅
   - Crear extractor `CurrentUser`
   - Extraer token del header `Authorization: Bearer <token>`
   - Validar contra `ISessionRepository`

2. **Rutas Protegidas** ✅
   - `GET /me` - Datos del usuario logueado

3. **Logout** ✅
   - `POST /logout` - Invalidar sesión

---

## Estándares del Laboratorio 3026

- **Arquitectura Hexagonal**: Dominio no conoce infraestructura
- **Inyección de Dependencias**: Via `Arc<dyn Trait>` en `di.rs`
- **Testing**: Tests pasando
- **Documentación**: Mantener actualizado

¡Manos a la obra!
