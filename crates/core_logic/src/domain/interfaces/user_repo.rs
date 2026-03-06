// crates/core_logic/src/domain/interfaces/user_repo.rs
//! # Puerto IUserRepository
//!
//! Define el contrato (la interfaz) para la persistencia de usuarios.
//! Es un "Puerto" en la Arquitectura Hexagonal, permitiendo que la lógica de negocio
//! interactúe con los datos del usuario sin acoplarse a una base de datos específica
//! (SQL, NoSQL, etc.).
//!
//! ## Responsabilidades
//! - Definir las operaciones CRUD abstractas para la entidad `User`.
//! - Asegurar que cualquier implementación pueda ser compartida de forma segura entre hilos (`Send + Sync`),
//!   un requisito para el estado compartido en `api_server` (ej. Axum).
//!
//! ## Dependencias
//! - `anyhow`: Para el manejo de errores.
//! - `async_trait`: Para permitir métodos `async` en el trait.
//! - Tipos del dominio: `User`, `UserId`, `Email`.

use crate::domain::{
    entities::user::User,
    value_objects::{email::Email, user_id::UserId},
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    /// Guarda (inserta o actualiza) un usuario en la capa de persistencia.
    async fn save(&self, user: &User) -> Result<()>;
    /// Busca un usuario por su `UserId`. Devuelve `None` si no se encuentra.
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>>;
    /// Busca un usuario por su `Email`. Devuelve `None` si no se encuentra.
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>>;
}
