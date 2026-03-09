// crates/core_logic/src/application/use_cases/role/list_permissions.rs
use crate::domain::{entities::role::Permission, interfaces::IRoleRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ListPermissions {
    role_repo: Arc<dyn IRoleRepository>,
}

impl ListPermissions {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Permission>> {
        self.role_repo.list_permissions().await
    }
}
