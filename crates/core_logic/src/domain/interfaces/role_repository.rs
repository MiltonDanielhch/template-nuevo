// crates/core_logic/src/domain/interfaces/role_repository.rs
//! # Puerto IRoleRepository
//!
//! Define el contrato para la persistencia y consulta de Roles y Permisos.
//! Permite que la lógica de negocio gestione RBAC sin acoplarse a SQLite.

use crate::domain::{
    entities::role::{Permission, PermissionId, Role, RoleId},
    value_objects::user_id::UserId,
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait IRoleRepository: Send + Sync {
    /// Guarda un nuevo rol en la persistencia.
    async fn create_role(&self, role: &Role) -> Result<()>;

    /// Busca un rol por su ID. Devuelve `None` si no existe.
    async fn find_role_by_id(&self, id: &RoleId) -> Result<Option<Role>>;

    /// Busca un rol por su nombre. Devuelve `None` si no existe.
    async fn find_role_by_name(&self, name: &str) -> Result<Option<Role>>;

    /// Devuelve todos los roles del sistema.
    async fn list_roles(&self) -> Result<Vec<Role>>;

    /// Asigna un rol a un usuario (inserta en `user_roles` o tabla de join).
    async fn assign_role_to_user(&self, user_id: &UserId, role_id: &RoleId) -> Result<()>;

    /// Elimina la asignación de un rol a un usuario.
    async fn remove_role_from_user(&self, user_id: &UserId, role_id: &RoleId) -> Result<()>;

    /// Retorna todos los permisos asociados a un usuario (via sus roles).
    async fn get_user_permissions(&self, user_id: &UserId) -> Result<Vec<Permission>>;

    /// Verifica si un usuario posee un permiso específico por nombre.
    async fn user_has_permission(&self, user_id: &UserId, permission_name: &str) -> Result<bool>;

    /// Crea un permiso en la persistencia.
    async fn create_permission(&self, permission: &Permission) -> Result<()>;

    /// Devuelve todos los permisos disponibles en el sistema.
    async fn list_permissions(&self) -> Result<Vec<Permission>>;

    /// Asigna un permiso a un rol.
    async fn assign_permission_to_role(
        &self,
        role_id: &RoleId,
        permission_id: &PermissionId,
    ) -> Result<()>;
    /// Actualiza el nombre y descripción de un rol.
    async fn update_role(&self, role: &Role) -> Result<()>;

    /// Elimina un rol físicamente (Hard Delete para roles).
    async fn delete_role(&self, id: &RoleId) -> Result<()>;

    /// Sincroniza los permisos de un rol (reemplaza los actuales por los nuevos).
    async fn sync_role_permissions(
        &self,
        role_id: &RoleId,
        permission_ids: &[PermissionId],
    ) -> Result<()>;
}
