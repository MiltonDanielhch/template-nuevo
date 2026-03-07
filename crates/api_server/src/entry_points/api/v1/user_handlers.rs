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
use axum::{Json, extract::State, http::StatusCode};
use core_logic::{
    application::use_cases::user::{login::LoginUserCommand, register::RegisterUserCommand},
    domain::entities::user::User,
};
use serde::{Deserialize, Serialize};

/// DTO (Data Transfer Object) para la petición de registro de usuario.
/// Se utiliza para deserializar el JSON de entrada.
#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub password: String,
}

/// DTO para la respuesta exitosa de registro de usuario.
/// Se utiliza para serializar la respuesta a JSON.
#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub id: String,
    pub email: String,
}

impl From<User> for RegisterUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
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
    pub email_verified: bool,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id().to_string(),
            email: user.email().to_string(),
            username: user.username().clone(),
            email_verified: user.is_email_verified(),
        }
    }
}

pub async fn me_handler(
    _state: State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<MeResponse>, ApiError> {
    Ok(Json(current_user.user.into()))
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
