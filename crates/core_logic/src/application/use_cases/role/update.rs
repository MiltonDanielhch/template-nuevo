// crates/core_logic/src/application/use_cases/role/update.rs
use crate::domain::{
    entities::role::{PermissionId, Role, RoleId},
    interfaces::IRoleRepository,
};
use anyhow::{Result, anyhow};
use std::sync::Arc;

pub struct UpdateRole {
    role_repo: Arc<dyn IRoleRepository>,
}

impl UpdateRole {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    pub async fn execute(
        &self,
        id: RoleId,
        name: String,
        description: Option<String>,
        permission_ids: Vec<PermissionId>,
    ) -> Result<Role> {
        let role = self
            .role_repo
            .find_role_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Rol no encontrado"))?;

        // Validar nombre único si cambió
        if name != role.name() && self.role_repo.find_role_by_name(&name).await?.is_some() {
            return Err(anyhow!("Ya existe un rol con ese nombre"));
        }

        // Creamos una nueva entidad con los datos actualizados (o mutamos si el dominio lo permitiera)
        // Por simplicidad en este dominio, reconstruimos.
        let updated_role = Role::from_persistence(
            id.clone(),
            name,
            description,
            *role.created_at(),
            vec![], // Los permisos se sincronizan aparte
        );

        self.role_repo.update_role(&updated_role).await?;
        self.role_repo
            .sync_role_permissions(&id, &permission_ids)
            .await?;

        // Retornar el rol fresco
        self.role_repo
            .find_role_by_id(&id)
            .await?
            .ok_or_else(|| anyhow!("Error al recuperar rol actualizado"))
    }
}
