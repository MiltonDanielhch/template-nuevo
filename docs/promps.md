Actúa como Ingeniero de Software Senior y Arquitecto del 'Laboratorio 3026'.

## Contexto del Proyecto

Estamos construyendo un sistema de autenticación y gestión de usuarios con una **Arquitectura Hexagonal estricta** en Rust. El objetivo es crear un backend de alto rendimiento, seguro y económico (VPS de $5) usando las mejores prácticas de desarrollo.

### Stack Tecnológico

| Capa | Tecnología | Propósito |
|------|------------|-----------|
| **Lenguaje** | Rust 2024 | Seguridad y rendimiento extremo |
| **API** | Axum | Framework web asíncrono |
| **Base de Datos** | SQLite (WAL) | Persistencia con SQLx |
| **Hashing** | Argon2id | Seguridad de contraseñas |
| **Workspace** | Cargo Multi-crate | Organización modular |

### Estructura del Proyecto

```
crates/
├── core_logic/        # Lógica de negocio pura (DOMINIO)
│   ├── domain/
│   │   ├── entities/  # User, Role, Permission
│   │   ├── value_objects/  # Email, UserId, PasswordHash
│   │   ├── interfaces/    # Traits (IUserRepository, IHasher)
│   │   └── errors.rs      # DomainError
│   └── application/
│       └── use_cases/     # RegisterUser, LoginUser
├── infra_db/          # Adaptadores de infraestructura
│   └── persistence/
│       └── sqlite/    # SqliteUserRepository, Argon2idHasher
└── api_server/        # Adaptador de entrada (Axum)
    ├── config/di.rs   # Composition Root (Inyección de Dependencias)
    ├── entry_points/  # Handlers HTTP
    └── routes.rs      # Router
```

### Estado Actual - ¡Ya Completado!

✅ **Registro de Usuario (POST /register)**
- Valida email con regex
- Hashea contraseña con Argon2id
- Verifica email duplicado antes de guardar
- Devuelve 409 Conflict si el email ya existe

✅ **Login de Usuario (POST /login)**
- Busca usuario por email
- Verifica contraseña contra hash almacenado
- Devuelve 401 Unauthorized si credenciales inválidas

✅ **Tests de Integración**
- `register_user_success`
- `register_user_duplicate_email`

### Value Objects Implementados

- **Email**: Validación de formato con regex, implementación de Display
- **UserId**: UUIDv7, implementación de Display
- **PasswordHash**: Wrapper seguro para hashes

### Errores de Dominio

```rust
pub enum DomainError {
    UserAlreadyExists(String),  // 409 Conflict
    InvalidCredentials,         // 401 Unauthorized
    ValidationError(String),   // 400 Bad Request
}
```

---

## Objetivo: Sistema de Sesiones (Próximo Paso)

Ahora que los usuarios pueden registrarse y hacer login, necesitamos implementar un **sistema de sesiones** para mantener la autenticación entre requests.

### Tareas a Implementar

1. **Crear Modelo de Sesión**:
   - Crear migración `0003_create_tokens.sql` con tabla `tokens`
   - Implementar Value Object `SessionToken`
   - Crear entidad `Session` en `core_logic/domain/entities/`

2. **Crear Repositorio de Sesiones**:
   - Definir trait `ISessionRepository` en `core_logic/domain/interfaces/`
   - Implementar `SqliteSessionRepository` en `infra_db/`

3. **Generar y Validar Tokens**:
   - Crear caso de uso `CreateSession` que genere un token único al hacer login
   - Implementar `ValidateSession` para verificar tokens en requests protegidos

4. **Middleware de Autenticación**:
   - Crear extractor `CurrentUser` en `api_server/entry_points/`
   - Proteger rutas que requieren autenticación

5. **Logout**:
   - Implementar caso de uso `LogoutUser` que invalide la sesión

---

## Estándares del Laboratorio 3026

- **Arquitectura Hexagonal**: El dominio no conoce la infraestructura
- **Inyección de Dependencias**: Via `Arc<dyn Trait>` en `di.rs`
- **Errores**: Usar `DomainError` para errores de negocio, `anyhow` para errores técnicos
- **Testing**: Tests de integración con base de datos en memoria
- **Documentación**: Mantener actualizado `docs/TROUBLESHOOTING.md` y `roadmaps/backend.md`

---

## Próximo Paso Sugerido

Implementar el sistema de sesiones/tokens comenzando por:
1. Crear la migración para la tabla de tokens
2. Definir el Value Object `SessionToken`
3. Crear el caso de uso `CreateSession` (se ejecuta después de LoginUser)

¡Manos a la obra!
