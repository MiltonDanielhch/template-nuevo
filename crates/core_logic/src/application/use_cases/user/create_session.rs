// crates/core_logic/src/application/use_cases/user/create_session.rs
//! # Caso de Uso: CreateSession
//!
//! Crea una nueva sesión para un usuario autenticado.

use crate::domain::entities::session::Session;
use crate::domain::entities::user::User;
use crate::domain::interfaces::session_repo::ISessionRepository;
use crate::domain::value_objects::SessionToken;
use anyhow::Result;
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateSession {
    session_repo: Arc<dyn ISessionRepository>,
}

impl CreateSession {
    pub fn new(session_repo: Arc<dyn ISessionRepository>) -> Self {
        Self { session_repo }
    }

    pub async fn execute(
        &self,
        user: User,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<SessionToken> {
        let token = Self::generate_token();
        let now = Utc::now();
        let expires = now + Duration::days(7);

        let session = Session {
            id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string(),
            user_id: user.id().as_str().to_string(),
            session_token: token.as_str().to_string(),
            ip_address,
            user_agent,
            expires_at: expires,
            created_at: now,
            last_activity_at: now,
            is_revoked: false,
        };

        self.session_repo.save(&session).await?;

        Ok(token)
    }

    fn generate_token() -> SessionToken {
        let token = Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        SessionToken::new(token).unwrap()
    }
}
