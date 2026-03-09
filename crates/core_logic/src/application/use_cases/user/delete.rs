// crates/core_logic/src/application/use_cases/user/delete.rs
//! # Caso de Uso: DeleteUser
//!
//! Realiza un borrado lógico de un usuario.

use crate::domain::{interfaces::IUserRepository, value_objects::user_id::UserId};
use anyhow::Result;
use std::sync::Arc;

pub struct DeleteUser {
    user_repo: Arc<dyn IUserRepository>,
}

impl DeleteUser {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, id: UserId) -> Result<()> {
        self.user_repo.delete(&id).await
    }
}
