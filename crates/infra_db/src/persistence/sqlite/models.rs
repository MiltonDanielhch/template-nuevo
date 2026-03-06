// crates/infra_db/src/persistence/sqlite/models.rs
//! # Módulo de Modelos de Base de Datos (SQLite)
//!
//! Este módulo define las estructuras que mapean directamente a las tablas
//! de la base de datos SQLite. A menudo se les llama DTOs (Data Transfer Objects)
//! de persistencia.
//!
//! ## Responsabilidades
//! - Representar la estructura de una fila de una tabla de la base de datos.
//! - Derivar `sqlx::FromRow` para permitir el mapeo automático desde los resultados de una consulta.
//! - Contener la lógica de mapeo para convertir desde una entidad de dominio a este modelo (`from_domain`).
//!
//! ## Dependencias
//! - `chrono`: Para manejar los tipos de fecha/hora `Naive` que vienen de la base de datos.
//! - `sqlx`: Para el macro `FromRow`.

use chrono::NaiveDateTime;
use sqlx::FromRow;

/// DbUser representa el esquema de la tabla 'users' en la base de datos.
///
/// Es una representación directa de los datos tal como se almacenan en SQLite.
/// No contiene lógica de negocio, solo la estructura de datos.
#[derive(Debug, Clone, FromRow)]
pub struct DbUser {
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl DbUser {
    /// Convierte una entidad de dominio `core_logic::User` a un `DbUser` para ser almacenado.
    pub fn from_domain(user: &core_logic::domain::entities::user::User) -> Self {
        Self {
            id: user.id().as_str().to_string(),
            username: user.username().clone(),
            email: user.email().as_str().to_string(),
            password_hash: user.password_hash().to_string(),
            avatar_url: user.avatar_url().clone(),
            email_verified: user.is_email_verified(),
            created_at: user.created_at().naive_utc(),
            updated_at: user.updated_at().naive_utc(),
            deleted_at: user.deleted_at().map(|dt| dt.naive_utc()),
        }
    }
}