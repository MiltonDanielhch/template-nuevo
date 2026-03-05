use crate::domain::value_objects::{email::Email, password::PasswordHash, user_id::UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Constructor para nuevos usuarios.
    /// El nombre se omite por ahora para ser consistente con la migración de la BD.
    pub fn new(email: Email, password_hash: PasswordHash) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }

    /// Constructor para reconstruir una entidad desde la capa de persistencia.
    pub fn new_from_persistence(
        id: UserId,
        email: Email,
        password_hash: PasswordHash,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            password_hash,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &UserId { &self.id }
    pub fn email(&self) -> &Email { &self.email }
    pub fn password_hash(&self) -> &PasswordHash { &self.password_hash }
    pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    pub fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
}