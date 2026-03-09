// crates/core_logic/src/application/use_cases/role/delete.rs
use crate::domain::{entities::role::RoleId, interfaces::IRoleRepository};
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct DeleteRole {
    role_repo: Arc<dyn IRoleRepository>,
}

impl DeleteRole {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(&self, id: RoleId) -> Result<()> {
        let role = self
            .role_repo
            .find_role_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Rol no encontrado"))?;

        // No permitir borrar roles vitales (opcional pero recomendado)
        if role.name() == "Admin" || role.name() == "User" {
            return Err(anyhow!(
                "No se pueden eliminar los roles del sistema (Admin/User)"
            ));
        }

        self.role_repo.delete_role(&id).await
    }
}
