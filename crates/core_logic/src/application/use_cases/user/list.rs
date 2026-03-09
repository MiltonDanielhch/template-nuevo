// crates/core_logic/src/application/use_cases/user/list.rs
use crate::domain::{entities::user::User, interfaces::user_repo::IUserRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ListUsers {
    user_repo: Arc<dyn IUserRepository>,
}

impl ListUsers {
    pub fn new(user_repo: Arc<dyn IUserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>> {
        self.user_repo.find_all().await
    }
}
