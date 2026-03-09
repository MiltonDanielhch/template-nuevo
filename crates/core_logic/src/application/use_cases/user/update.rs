// crates/core_logic/src/application/use_cases/user/update.rs
//! # Caso de Uso: UpdateUser
//!
//! Permite actualizar los datos de un usuario existente.

use crate::domain::{
    entities::user::User,
    errors::DomainError,
    interfaces::{IHasher, IUserRepository},
    value_objects::{email::Email, user_id::UserId},
};
use anyhow::Result;
use std::sync::Arc;

pub struct UpdateUserCommand {
    pub id: UserId,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

pub struct UpdateUser {
    user_repo: Arc<dyn IUserRepository>,
    hasher: Arc<dyn IHasher>,
}

impl UpdateUser {
    pub fn new(user_repo: Arc<dyn IUserRepository>, hasher: Arc<dyn IHasher>) -> Self {
        Self { user_repo, hasher }
    }

    pub async fn execute(&self, command: UpdateUserCommand) -> Result<User> {
        let mut user = self
            .user_repo
            .find_by_id(&command.id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Usuario con ID {} no encontrado", command.id))
            })?;

        if let Some(username) = command.username {
            user.set_username(Some(username));
        }

        if let Some(email_str) = command.email {
            let email = Email::parse(email_str)?;
            // Verificar si el email ya está en uso por otro usuario
            if let Some(existing) = self.user_repo.find_by_email(&email).await?
                && existing.id() != user.id()
            {
                return Err(DomainError::UserAlreadyExists(email.to_string()).into());
            }
            user.set_email(email);
        }

        if let Some(password) = command.password {
            let hash = self.hasher.hash(&password).await?;
            user.set_password_hash(hash);
        }

        if let Some(avatar_url) = command.avatar_url {
            user.set_avatar_url(Some(avatar_url));
        }

        self.user_repo.save(&user).await?;
        Ok(user)
    }
}
