// crates/api_server/src/config/di.rs
//! # Inyección de Dependencias (Composition Root)
//!
//! Único lugar donde las implementaciones concretas se instancian y se ensamblan.
//! Actúa como el "Composition Root" de la Arquitectura Hexagonal.

use core_logic::{
    application::use_cases::{
        lead::CreateLead,
        role::{AssignRoleToUser, CreateRole, DeleteRole, ListPermissions, ListRoles, UpdateRole},
        user::{
            CreateSession, DeleteUser, GetUserById, ListUsers, LoginUser, RegisterUser, UpdateUser,
        },
    },
    domain::interfaces::{IAuditRepository, ILeadRepository, IRoleRepository, ISessionRepository},
};
use infra_db::{
    Argon2idHasher, SqliteAuditRepository, SqliteLeadRepository, SqliteRoleRepository,
    SqliteSessionRepository, SqliteUserRepository,
};
use std::sync::Arc;

/// Estado de la aplicación compartido a través de los handlers de Axum.
/// Contiene todos los casos de uso y repositorios necesarios.
#[derive(Clone)]
pub struct AppState {
    // User use cases
    pub register_user: Arc<RegisterUser>,
    pub login_user: Arc<LoginUser>,
    pub list_users: Arc<ListUsers>,
    pub update_user: Arc<UpdateUser>,
    pub delete_user: Arc<DeleteUser>,
    pub create_session: Arc<CreateSession>,
    pub session_repo: Arc<dyn ISessionRepository>,
    pub get_user_by_id: Arc<GetUserById>,
    // Role/RBAC use cases
    pub create_role: Arc<CreateRole>,
    pub assign_role: Arc<AssignRoleToUser>,
    pub list_roles: Arc<ListRoles>,
    pub update_role: Arc<UpdateRole>,
    pub delete_role: Arc<DeleteRole>,
    pub list_permissions: Arc<ListPermissions>,
    pub role_repo: Arc<dyn IRoleRepository>,
    // Leads
    pub create_lead: Arc<CreateLead>,
    pub lead_repo: Arc<dyn ILeadRepository>,
    // Audit
    pub audit_repo: Arc<dyn IAuditRepository>,
}

pub fn create_app_state(pool: sqlx::SqlitePool) -> AppState {
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));
    let session_repo = Arc::new(SqliteSessionRepository::new(pool.clone()));
    let role_repo: Arc<dyn IRoleRepository> = Arc::new(SqliteRoleRepository::new(pool.clone()));
    let lead_repo: Arc<dyn ILeadRepository> = Arc::new(SqliteLeadRepository::new(pool.clone()));
    let hasher = Arc::new(Argon2idHasher {});

    // User use cases
    let register_user = Arc::new(RegisterUser::new(
        user_repo.clone(),
        hasher.clone(),
        role_repo.clone(),
    ));
    let login_user = Arc::new(LoginUser::new(user_repo.clone(), hasher.clone()));
    let list_users = Arc::new(ListUsers::new(user_repo.clone()));
    let update_user = Arc::new(UpdateUser::new(user_repo.clone(), hasher.clone()));
    let delete_user = Arc::new(DeleteUser::new(user_repo.clone()));
    let create_session = Arc::new(CreateSession::new(session_repo.clone()));
    let get_user_by_id = Arc::new(GetUserById::new(user_repo.clone()));
    let create_lead = Arc::new(CreateLead::new(lead_repo.clone()));

    // Role use cases
    let create_role = Arc::new(CreateRole::new(role_repo.clone()));
    let assign_role = Arc::new(AssignRoleToUser::new(role_repo.clone()));
    let list_roles = Arc::new(ListRoles::new(role_repo.clone()));
    let update_role = Arc::new(UpdateRole::new(role_repo.clone()));
    let delete_role = Arc::new(DeleteRole::new(role_repo.clone()));
    let list_permissions = Arc::new(ListPermissions::new(role_repo.clone()));

    // Audit
    let audit_repo: Arc<dyn IAuditRepository> = Arc::new(SqliteAuditRepository::new(pool));

    AppState {
        register_user,
        login_user,
        list_users,
        update_user,
        delete_user,
        create_session,
        session_repo,
        get_user_by_id,
        create_lead,
        lead_repo,
        create_role,
        assign_role,
        list_roles,
        update_role,
        delete_role,
        list_permissions,
        role_repo,
        audit_repo,
    }
}
