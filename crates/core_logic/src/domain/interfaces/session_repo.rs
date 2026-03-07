// crates/core_logic/src/domain/interfaces/session_repo.rs
//! # Puerto ISessionRepository
//!
//! Define el contrato para la persistencia de sesiones.

use crate::domain::entities::session::Session;
use crate::domain::value_objects::SessionToken;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ISessionRepository: Send + Sync {
    async fn save(&self, session: &Session) -> Result<()>;
    async fn find_by_token(&self, token: &SessionToken) -> Result<Option<Session>>;
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Session>>;
    async fn revoke(&self, token: &SessionToken) -> Result<()>;
    async fn delete_expired(&self) -> Result<u64>;
}
