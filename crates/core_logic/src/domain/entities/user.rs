use crate::domain::value_objects::email::Email;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: Email,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    // Factory method para crear un nuevo usuario
    pub fn new(name: String, email: Email) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            name,
            email,
            created_at: now,
            updated_at: now,
        }
    }
}