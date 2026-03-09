// crates/core_logic/src/application/use_cases/role/list.rs
//! # Caso de Uso: ListRoles
//!
//! Retorna todos los roles disponibles en el sistema.

use crate::domain::{entities::role::Role, interfaces::IRoleRepository};
use anyhow::Result;
use std::sync::Arc;

pub struct ListRoles {
    role_repo: Arc<dyn IRoleRepository>,
}

impl ListRoles {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(&self) -> Result<Vec<Role>> {
        self.role_repo.list_roles().await
    }
}
