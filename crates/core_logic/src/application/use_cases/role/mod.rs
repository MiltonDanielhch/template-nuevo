// crates/core_logic/src/application/use_cases/role/mod.rs
//! Módulo de casos de uso para Roles y Permisos (RBAC).

pub mod assign;
pub mod create;
pub mod list;

pub use assign::AssignRoleToUser;
pub use create::CreateRole;
pub use list::ListRoles;
