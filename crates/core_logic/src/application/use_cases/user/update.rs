// crates/core_logic/src/application/use_cases/user/update.rs
use crate::domain::{
    entities::user::User,
    interfaces::{hasher::IHasher, user_repo::IUserRepository},
    value_objects::{email::Email, user_id::UserId},
};
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct UpdateUserCommand {
    pub id: String,
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
        let user_id = UserId::new_from_string(command.id)?;
        let mut user = self
            .user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow!("Usuario no encontrado"))?;

        if let Some(username) = command.username {
            user.set_username(Some(username));
        }

        if let Some(email_str) = command.email {
            let email = Email::parse(email_str)?;
            // Verificar si el email ya existe en otro usuario
            if self
                .user_repo
                .find_by_email(&email)
                .await?
                .is_some_and(|existing| existing.id() != user.id())
            {
                return Err(anyhow!("Email ya en uso"));
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
