// crates/core_logic/src/application/use_cases/role/create.rs
//! # Caso de Uso: CreateRole
//!
//! Encapsula la lógica de negocio para crear un nuevo rol en el sistema.
//! Verifica que no exista un rol con el mismo nombre antes de persistir.

use crate::domain::{
    entities::role::Role,
    errors::DomainError,
    interfaces::IRoleRepository,
};
use anyhow::Result;
use std::sync::Arc;

pub struct CreateRole {
    role_repo: Arc<dyn IRoleRepository>,
}

impl CreateRole {
    pub fn new(role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { role_repo }
    }

    /// Crea un nuevo rol. Falla si ya existe un rol con ese nombre.
    pub async fn execute(&self, name: String, description: Option<String>) -> Result<Role> {
        // Verificar duplicado por nombre
        if let Some(_existing) = self.role_repo.find_role_by_name(&name).await? {
            return Err(DomainError::ValidationError(format!(
                "El rol '{}' ya existe en el sistema.",
                name
            ))
            .into());
        }

        let role = Role::new(name, description);
        self.role_repo.create_role(&role).await?;
        Ok(role)
    }
}
