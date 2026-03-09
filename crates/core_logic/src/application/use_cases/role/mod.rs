// crates/core_logic/src/application/use_cases/role/mod.rs
//! Módulo de casos de uso para Roles y Permisos (RBAC).

pub mod assign;
pub mod create;
pub mod delete;
pub mod list;
pub mod list_permissions;
pub mod update;

pub use assign::AssignRoleToUser;
pub use create::CreateRole;
pub use delete::DeleteRole;
pub use list::ListRoles;
pub use list_permissions::ListPermissions;
pub use update::UpdateRole;
