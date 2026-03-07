// crates/core_logic/src/domain/entities/session.rs
//! # Entidad: Session
//!
//! Representa una sesión de usuario activa.

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub session_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub is_revoked: bool,
}
