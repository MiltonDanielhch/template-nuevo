use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use core_logic::domain::{
    entities::user::User,
    value_objects::email::Email,
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
        let uuid = Uuid::from_slice(&db_user.id)
            .map_err(|e| anyhow::anyhow!("Error al parsear UUID desde BLOB: {}", e))?;
        let user_id = core_logic::domain::value_objects::user_id::UserId::new_from_uuid(uuid)?;
        let email = Email::parse(db_user.email)?;
        let password_hash = core_logic::domain::value_objects::password::PasswordHash::new(db_user.password_hash)?;

        let created_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.created_at, Utc);
        let updated_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.updated_at, Utc);

        Ok(User::new_from_persistence(
            user_id,
            email,
            password_hash,
            created_at,
            updated_at,
        ))
    }
}

#[async_trait]
impl IUserRepository for SqliteUserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        let db_user = DbUser::from_domain(user);

        sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                updated_at = EXCLUDED.updated_at
            "#,
            db_user.id,
            db_user.email,
            db_user.password_hash,
            db_user.created_at,
            db_user.updated_at,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let db_user = sqlx::query_as!(
            DbUser,
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        db_user.map_or(Ok(None), |u| Self::to_domain_user(u).map(Some))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>> {
        let email_str = email.as_str();
        let db_user = sqlx::query_as!(
            DbUser,
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = ?",
            email_str
        )
        .fetch_optional(&self.pool)
        .await?;

        db_user.map_or(Ok(None), |u| Self::to_domain_user(u).map(Some))
    }
}