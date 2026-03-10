// crates/api_server/src/entry_points/api/v1/user_handlers.rs
//! # Handlers de Usuario (API v1)
//!
//! Este módulo contiene los handlers de Axum para las rutas relacionadas con usuarios.
//! Cumple con el rol de "Adaptador Primario" en la Arquitectura Hexagonal.
//!
//! ## Responsabilidades
//! - Deserializar los datos de la petición (ej. JSON del body).
//! - Extraer el `AppState` para acceder a los casos de uso.
//! - Invocar el caso de uso correspondiente con los datos de entrada.
//! - Mapear los resultados (`Result`) del caso de uso a respuestas HTTP,
//!   utilizando el tipo `ApiError` para una gestión de errores clara.
//! - Serializar la respuesta del caso de uso a JSON.

use crate::{
    config::di::AppState,
    entry_points::{api::v1::errors::ApiError, auth::CurrentUser},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use core_logic::{
    application::use_cases::user::{
        login::LoginUserCommand, register::RegisterUserCommand, update::UpdateUserCommand,
    },
    domain::entities::user::User,
};
use serde::{Deserialize, Serialize};

/// DTO (Data Transfer Object) para la petición de registro de usuario.
/// Se utiliza para deserializar el JSON de entrada.
#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub role: Option<String>,
}

/// DTO para la respuesta exitosa de registro de usuario.
/// Se utiliza para serializar la respuesta a JSON.
#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub role: String,
}

impl From<User> for RegisterUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            username: user.username().clone(),
            role: "User".to_string(), // TODO: Cargar del repo de roles
        }
    }
}

/// Handler para `POST /register`.
///
/// Recibe los datos del nuevo usuario, los pasa al caso de uso `RegisterUser`
/// y devuelve el usuario creado o un error HTTP apropiado.
pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, ApiError> {
    let command = RegisterUserCommand {
        email: payload.email,
        password: payload.password,
        username: payload.username,
    };

    let new_user = state.register_user.execute(command).await?;

    Ok(Json(new_user.into()))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: String,
    pub email: String,
    pub token: String,
}

impl LoginResponse {
    pub fn from_user_and_token(user: User, token: String) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            token,
        }
    }
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let command = LoginUserCommand {
        email: payload.email,
        password: payload.password,
    };

    let user = state.login_user.execute(command).await?;

    let token = state
        .create_session
        .execute(user.clone(), None, None)
        .await?;

    Ok(Json(LoginResponse::from_user_and_token(
        user,
        token.to_string(),
    )))
}

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub roles: Vec<String>,
    pub email_verified: bool,
}

impl From<(User, Vec<String>)> for MeResponse {
    fn from((user, roles): (User, Vec<String>)) -> Self {
        let primary_role = roles.first().cloned().unwrap_or_else(|| "User".to_string());
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            username: user.username().clone(),
            avatar_url: user.avatar_url().clone(),
            role: primary_role,
            roles,
            email_verified: user.is_email_verified(),
        }
    }
}

#[derive(Serialize)]
pub struct UserRoleResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub async fn get_my_roles_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<Vec<UserRoleResponse>>, ApiError> {
    let roles = state
        .role_repo
        .get_user_roles(current_user.user.id())
        .await?;
    let response: Vec<UserRoleResponse> = roles
        .into_iter()
        .map(|r| UserRoleResponse {
            id: r.id().to_string(),
            name: r.name().to_string(),
            description: r.description().map(|s| s.to_string()),
        })
        .collect();
    Ok(Json(response))
}

pub async fn me_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<MeResponse>, ApiError> {
    let roles = state
        .role_repo
        .get_user_permissions(current_user.user.id())
        .await?
        .iter()
        .map(|p| p.name().to_string())
        .collect();

    Ok(Json((current_user.user, roles).into()))
}

pub async fn update_me_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<MeResponse>, ApiError> {
    let command = UpdateUserCommand {
        id: current_user.user.id().clone(),
        username: payload.username,
        email: payload.email,
        password: payload.password,
        avatar_url: payload.avatar_url,
    };

    let user = state.update_user.execute(command).await?;
    let roles = state
        .role_repo
        .get_user_permissions(user.id())
        .await?
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    Ok(Json((user, roles).into()))
}

pub async fn logout_handler(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<StatusCode, ApiError> {
    use core_logic::domain::value_objects::SessionToken;
    let token = SessionToken::new(current_user.user.id().to_string())
        .map_err(|e| ApiError(anyhow::anyhow!(e)))?;

    state.session_repo.revoke(&token).await?;

    Ok(StatusCode::NO_CONTENT)
}

// ---- CRUD Handlers ----

/// GET /api/v1/users
pub async fn list_users_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
) -> Result<Json<Vec<MeResponse>>, ApiError> {
    let users = state.list_users.execute().await?;
    let mut response = Vec::with_capacity(users.len());
    for user in users {
        let user_roles = state.role_repo.get_user_roles(user.id()).await?;
        let roles: Vec<String> = user_roles.iter().map(|r| r.name().to_string()).collect();
        response.push((user, roles).into());
    }
    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

/// PUT /api/v1/users/:id
pub async fn update_user_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id_str): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<MeResponse>, ApiError> {
    use core_logic::domain::value_objects::user_id::UserId;

    let user_id = UserId::new_from_string(id_str).map_err(|e| ApiError(anyhow::anyhow!(e)))?;

    let command = UpdateUserCommand {
        id: user_id,
        username: payload.username,
        email: payload.email,
        password: payload.password,
        avatar_url: payload.avatar_url,
    };

    let user = state.update_user.execute(command).await?;
    let roles = state
        .role_repo
        .get_user_permissions(user.id())
        .await?
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    Ok(Json((user, roles).into()))
}

/// DELETE /api/v1/users/:id
pub async fn delete_user_handler(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id_str): Path<String>,
) -> Result<StatusCode, ApiError> {
    use core_logic::domain::value_objects::user_id::UserId;

    let user_id = UserId::new_from_string(id_str).map_err(|e| ApiError(anyhow::anyhow!(e)))?;

    state.delete_user.execute(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
