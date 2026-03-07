// crates/infra_db/src/persistence/sqlite/repositories/sqlite_session_repo.rs
//! # Adaptador SqliteSessionRepository

use crate::persistence::sqlite::models::DbSession;
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use core_logic::domain::entities::session::Session;
use core_logic::domain::interfaces::session_repo::ISessionRepository;
use core_logic::domain::value_objects::SessionToken;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct SqliteSessionRepository {
    pool: SqlitePool,
}

impl SqliteSessionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn to_domain_session(db: DbSession) -> Result<Session> {
        let expires_at = db.expires_at
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .unwrap_or_else(|| Utc::now());
        let created_at = db.created_at
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .unwrap_or_else(Utc::now);
        let last_activity_at = db.last_activity_at
            .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
            .unwrap_or_else(Utc::now);
        let is_revoked = db.is_revoked.unwrap_or(false);

        Ok(Session {
            id: db.id,
            user_id: db.user_id,
            session_token: db.session_token,
            ip_address: db.ip_address,
            user_agent: db.user_agent,
            expires_at,
            created_at,
            last_activity_at,
            is_revoked,
        })
    }
}

#[async_trait]
impl ISessionRepository for SqliteSessionRepository {
    async fn save(&self, session: &Session) -> Result<()> {
        let expires_at = session.expires_at.naive_utc();
        let created_at = session.created_at.naive_utc();
        let last_activity_at = session.last_activity_at.naive_utc();
        let is_revoked = session.is_revoked;

        sqlx::query!(
            r#"
            INSERT INTO sessions (id, user_id, session_token, ip_address, user_agent, expires_at, created_at, last_activity_at, is_revoked)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            session.id,
            session.user_id,
            session.session_token,
            session.ip_address,
            session.user_agent,
            expires_at,
            created_at,
            last_activity_at,
            is_revoked,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_token(&self, token: &SessionToken) -> Result<Option<Session>> {
        let token_str = token.as_str().to_string();
        let db_session = sqlx::query_as!(
            DbSession,
            "SELECT * FROM sessions WHERE session_token = ? AND is_revoked = FALSE AND expires_at > datetime('now')",
            token_str
        )
        .fetch_optional(&self.pool)
        .await?;

        db_session.map_or(Ok(None), |s| Self::to_domain_session(s).map(Some))
    }

    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Session>> {
        let user_id_str = user_id.to_string();
        let sessions = sqlx::query_as!(
            DbSession,
            "SELECT * FROM sessions WHERE user_id = ? AND is_revoked = FALSE AND expires_at > datetime('now') ORDER BY created_at DESC",
            user_id_str
        )
        .fetch_all(&self.pool)
        .await?;

        sessions
            .into_iter()
            .map(Self::to_domain_session)
            .collect()
    }

    async fn revoke(&self, token: &SessionToken) -> Result<()> {
        let token_str = token.as_str();
        sqlx::query!(
            "UPDATE sessions SET is_revoked = TRUE WHERE session_token = ?",
            token_str
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_expired(&self) -> Result<u64> {
        let result = sqlx::query!(
            "DELETE FROM sessions WHERE expires_at <= datetime('now') OR is_revoked = TRUE"
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}
