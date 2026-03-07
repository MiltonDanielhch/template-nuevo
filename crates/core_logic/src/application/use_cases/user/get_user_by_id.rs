// crates/core_logic/src/application/use_cases/user/get_user_by_id.rs
//! # Caso de Uso: GetUserById
//!
//! Recupera un usuario por su ID.

use crate::domain::{
    entities::user::User, errors::DomainError, interfaces::IUserRepository, value_objects::UserId,
};
use anyhow::{Context, Result};
use std::sync::Arc;

pub struct GetUserById {
    user_repo: Arc<dyn IUserRepository>,
}

impl GetUserById {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: String) -> Result<User> {
        let user_id = UserId::new_from_string(user_id).context("Error al parsear el UserId")?;

        let user = self
            .user_repo
            .find_by_id(&user_id)
            .await
            .context("Error al buscar el usuario")?
            .ok_or_else(|| DomainError::NotFound("Usuario no encontrado".to_string()))?;

        Ok(user)
    }
}
