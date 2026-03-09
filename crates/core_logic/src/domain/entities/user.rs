// crates/core_logic/src/domain/entities/user.rs
//! # Entidad: User
//!
//! Representa un usuario en el núcleo del dominio. Es el agregado principal
//! para la lógica relacionada con usuarios.
//!
//! ## Responsabilidades
//! - Mantener la consistencia de sus datos a través de sus métodos.
//! - Encapsular las reglas de negocio (ej. cómo se crea un nuevo usuario).
//! - Exponer sus datos de forma controlada a través de getters.
//!
//! ## Dependencias
//! - `Value Objects`: `UserId`, `Email`, `PasswordHash` para garantizar la validez de sus atributos.
//! - `chrono`: Para manejar las marcas de tiempo `created_at` y `updated_at`.

use crate::domain::value_objects::{Email, PasswordHash, UserId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: Email,
    username: Option<String>,
    password_hash: PasswordHash,
    avatar_url: Option<String>,
    email_verified: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

/// UserPersistenceData es un DTO para reconstruir la entidad User desde la persistencia.
/// Se utiliza para evitar constructores con un número excesivo de argumentos.
pub struct UserPersistenceData {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    /// Constructor para nuevos usuarios.
    pub fn new(email: Email, password_hash: PasswordHash, username: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            username,
            password_hash,
            avatar_url: None,
            email_verified: false,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    /// Constructor para reconstruir una entidad desde la capa de persistencia.
    pub fn new_from_persistence(data: UserPersistenceData) -> Self {
        Self {
            id: data.id,
            email: data.email,
            password_hash: data.password_hash,
            username: data.username,
            avatar_url: data.avatar_url,
            email_verified: data.email_verified,
            created_at: data.created_at,
            updated_at: data.updated_at,
            deleted_at: data.deleted_at,
        }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }
    pub fn email(&self) -> &Email {
        &self.email
    }
    pub fn username(&self) -> &Option<String> {
        &self.username
    }
    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }
    pub fn avatar_url(&self) -> &Option<String> {
        &self.avatar_url
    }
    pub fn is_email_verified(&self) -> bool {
        self.email_verified
    }
    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    pub fn deleted_at(&self) -> &Option<DateTime<Utc>> {
        &self.deleted_at
    }

    // Setters (Domain logic for updates)
    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
        self.updated_at = Utc::now();
    }
    pub fn set_email(&mut self, email: Email) {
        self.email = email;
        self.updated_at = Utc::now();
    }
    pub fn set_password_hash(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash;
        self.updated_at = Utc::now();
    }
    pub fn set_avatar_url(&mut self, avatar_url: Option<String>) {
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }
}
