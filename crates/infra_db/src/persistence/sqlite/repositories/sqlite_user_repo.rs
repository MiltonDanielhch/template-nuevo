// crates/infra_db/src/persistence/sqlite/repositories/sqlite_user_repo.rs
//! # Adaptador SqliteUserRepository
//!
//! Implementación concreta (Adaptador) del puerto `IUserRepository`.
//! Es el puente que traduce las necesidades de la lógica de negocio
//! ("guarda este usuario") a comandos específicos de la base de datos SQLite.
//!
//! ## Responsabilidades
//! - Implementar todos los métodos del trait `IUserRepository`.
//! - Manejar la conexión con la base de datos a través de un `SqlitePool`.
//! - Mapear entre las entidades de dominio (`User`) y los modelos de base de datos (`DbUser`).
//!
//! ## Dependencias
//! - `core_logic`: Para acceder a los traits y entidades del dominio.
//! - `sqlx`: Para interactuar con la base de datos SQLite.
//! - `anyhow`: Para el manejo de errores.
use async_trait::async_trait;
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use anyhow::Result;

use core_logic::domain::{
    entities::user::{User, UserPersistenceData},
    value_objects::{email::Email, password_hash::PasswordHash, user_id::UserId},
    interfaces::user_repo::IUserRepository,
};

use crate::persistence::sqlite::models::DbUser;

/// SqliteUserRepository es el adaptador concreto para IUserRepository.
///
/// Implementa la lógica de persistencia para usuarios utilizando SQLx y SQLite.
/// Mantiene una referencia al pool de conexiones de la base de datos.
#[derive(Clone)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    /// Crea una nueva instancia de SqliteUserRepository.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Convierte un `DbUser` a una entidad de dominio `core_logic::User`.
    /// Este es nuestro "Mapper" de la capa de infraestructura al dominio.
    fn to_domain_user(db_user: DbUser) -> Result<User> {
        let user_id = UserId::new_from_string(db_user.id)?;
        let email = Email::parse(db_user.email)?;
        let password_hash = PasswordHash::new(db_user.password_hash)?;

        let created_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.created_at, Utc);
        let updated_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.updated_at, Utc);
        let deleted_at = db_user.deleted_at.map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));

        Ok(User::new_from_persistence(UserPersistenceData {
            id: user_id,
            email,
            password_hash,
            username: db_user.username,
            avatar_url: db_user.avatar_url,
            email_verified: db_user.email_verified,
            created_at,
            updated_at,
            deleted_at,
        }))
    }
}

#[async_trait]
impl IUserRepository for SqliteUserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        let db_user = DbUser::from_domain(user);

        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, avatar_url, email_verified, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                username = EXCLUDED.username,
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                avatar_url = EXCLUDED.avatar_url,
                email_verified = EXCLUDED.email_verified,
                updated_at = EXCLUDED.updated_at,
                deleted_at = EXCLUDED.deleted_at
            "#,
            db_user.id,
            db_user.username,
            db_user.email,
            db_user.password_hash,
            db_user.avatar_url,
            db_user.email_verified,
            db_user.created_at,
            db_user.updated_at,
            db_user.deleted_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>> {
        let user_id_str = id.as_str();
        let db_user = sqlx::query_as!(
            DbUser,
            "SELECT * FROM users WHERE id = ? AND deleted_at IS NULL",
            user_id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        db_user.map_or(Ok(None), |u| Self::to_domain_user(u).map(Some))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>> {
        let email_str = email.as_str();
        let db_user = sqlx::query_as!(
            DbUser,
            "SELECT * FROM users WHERE email = ? AND deleted_at IS NULL",
            email_str
        )
        .fetch_optional(&self.pool)
        .await?;

        db_user.map_or(Ok(None), |u| Self::to_domain_user(u).map(Some))
    }
}