// crates/core_logic/src/application/use_cases/user/delete.rs
use crate::domain::{interfaces::user_repo::IUserRepository, value_objects::user_id::UserId};
use anyhow::Result;
use std::sync::Arc;

pub struct DeleteUser {
    user_repo: Arc<dyn IUserRepository>,
}

impl DeleteUser {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, id: String) -> Result<()> {
        let user_id = UserId::new_from_string(id)?;
        self.user_repo.delete(&user_id).await
    }
}
