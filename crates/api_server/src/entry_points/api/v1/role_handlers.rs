// crates/api_server/src/entry_points/api/v1/role_handlers.rs
//! # Handlers de Roles (API v1)
//!
//! Expone los endpoints RBAC: crear roles, listar roles, asignar roles a usuarios.

use crate::{
    config::di::AppState,
    entry_points::{
        api::v1::errors::ApiError,
        auth::CurrentUser,
        middleware::rbac::{RequirePermission, RolesRead, RolesWrite},
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use core_logic::domain::entities::role::{Role, RoleId};
use serde::{Deserialize, Serialize};

// ---- DTOs ----

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Serialize)]
pub struct PermissionResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<PermissionResponse>,
}

impl From<Role> for RoleResponse {
    fn from(role: Role) -> Self {
        Self {
            id: role.id().to_string(),
            name: role.name().to_string(),
            description: role.description().map(|s| s.to_string()),
            permissions: role
                .permissions()
                .iter()
                .map(|p| PermissionResponse {
                    id: p.id().to_string(),
                    name: p.name().to_string(),
                    description: p.description().map(|s| s.to_string()),
                })
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: String,
}

// ---- Handlers ----

/// POST /api/v1/roles
/// Crea un nuevo rol. Solo usuarios autenticados.
pub async fn create_role_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesWrite>,
    Json(payload): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<RoleResponse>), ApiError> {
    use core_logic::domain::entities::role::PermissionId;

    let perm_ids: Vec<PermissionId> = payload
        .permissions
        .into_iter()
        .map(PermissionId::from_string)
        .collect();

    let role = state
        .create_role
        .execute(payload.name, payload.description, perm_ids)
        .await?;

    Ok((StatusCode::CREATED, Json(role.into())))
}

/// GET /api/v1/roles
/// Lista todos los roles del sistema.
pub async fn list_roles_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesRead>,
) -> Result<Json<Vec<RoleResponse>>, ApiError> {
    let roles = state.list_roles.execute().await?;
    let response: Vec<RoleResponse> = roles.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

/// POST /api/v1/users/:user_id/roles
/// Asigna un rol a un usuario por su ID.
pub async fn assign_role_to_user_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesWrite>,
    Path(user_id_str): Path<String>,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<StatusCode, ApiError> {
    use core_logic::domain::value_objects::UserId;

    let user_id = UserId::new_from_string(user_id_str).map_err(|e| ApiError(anyhow::anyhow!(e)))?;
    let role_id = RoleId::from_string(payload.role_id);

    state.assign_role.execute(user_id, role_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/users/me/permissions
/// Retorna los permisos del usuario autenticado.
pub async fn get_my_permissions_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<Vec<String>>, ApiError> {
    let permissions = state
        .role_repo
        .get_user_permissions(current_user.user.id())
        .await?;

    let names: Vec<String> = permissions.iter().map(|p| p.name().to_string()).collect();
    Ok(Json(names))
}
#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

/// PUT /api/v1/roles/:id
/// Actualiza un rol existente.
pub async fn update_role_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesWrite>,
    Path(role_id_str): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>, ApiError> {
    use core_logic::domain::entities::role::{PermissionId, RoleId};

    let role_id = RoleId::from_string(role_id_str);
    let perm_ids: Vec<PermissionId> = payload
        .permissions
        .into_iter()
        .map(PermissionId::from_string)
        .collect();

    let role = state
        .update_role
        .execute(role_id, payload.name, payload.description, perm_ids)
        .await?;

    Ok(Json(role.into()))
}

/// DELETE /api/v1/roles/:id
/// Elimina un rol.
pub async fn delete_role_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesWrite>,
    Path(role_id_str): Path<String>,
) -> Result<StatusCode, ApiError> {
    use core_logic::domain::entities::role::RoleId;
    let role_id = RoleId::from_string(role_id_str);
    state.delete_role.execute(role_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/permissions
/// Lista todos los permisos del sistema.
pub async fn list_permissions_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    _perm: RequirePermission<RolesRead>,
) -> Result<Json<Vec<PermissionResponse>>, ApiError> {
    let permissions = state.list_permissions.execute().await?;
    let response: Vec<PermissionResponse> = permissions
        .into_iter()
        .map(|p| PermissionResponse {
            id: p.id().to_string(),
            name: p.name().to_string(),
            description: p.description().map(|s| s.to_string()),
        })
        .collect();
    Ok(Json(response))
}
