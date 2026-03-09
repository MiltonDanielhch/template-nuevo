// crates/core_logic/src/application/use_cases/role/assign.rs
//! # Caso de Uso: AssignRoleToUser
//!
//! Encapsula la lógica para asignar un rol a un usuario.
//! Verifica que el rol exista antes de aceptar la asignación.

use crate::domain::{
    entities::role::RoleId,
    errors::DomainError,
    interfaces::IRoleRepository,
    value_objects::user_id::UserId,
};
use anyhow::Result;
use std::sync::Arc;

pub struct AssignRoleToUser {
    role_repo: Arc<dyn IRoleRepository>,
}

impl AssignRoleToUser {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    /// Asigna el `role_id` al `user_id`. Falla si el rol no existe.
    pub async fn execute(&self, user_id: UserId, role_id: RoleId) -> Result<()> {
        // Verificar que el rol exista
        self.role_repo
            .find_role_by_id(&role_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Rol '{}' no encontrado.", role_id))
            })?;

        self.role_repo.assign_role_to_user(&user_id, &role_id).await
    }
}
