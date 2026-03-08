// crates/api_server/src/entry_points/auth.rs
//! # Autenticación y Extractores
//!
//! Este módulo contiene funciones para extraer el usuario actual desde el token.

use crate::config::di::AppState;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use core_logic::domain::value_objects::SessionToken;

#[derive(Clone)]
pub struct CurrentUser {
    pub user: core_logic::domain::entities::user::User,
}

// Implementación concreta para AppState.
// Esto simplifica la inyección y evita problemas con genéricos complejos en los handlers.
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // En Axum 0.8 con with_state, el estado se pasa directamente en `state`.
        // Ya no necesitamos buscar en extensions.

        let authorization = parts.headers.get(AUTHORIZATION).cloned().ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "Token de autorización requerido"})),
            )
        })?;

        let token_header = authorization.to_str().map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "Token inválido"})),
            )
        })?;

        let token_string: String = token_header
            .strip_prefix("Bearer ")
            .map(|s: &str| s.to_string())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({"error": "Formato de token inválido. Use: Bearer <token>"})),
                )
            })?;

        let session_token = SessionToken::new(token_string).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": e.to_string()})),
            )
        })?;

        let session = state
            .session_repo
            .find_by_token(&session_token)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(serde_json::json!({"error": "Error interno"})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({"error": "Sesión no válida o expirada"})),
                )
            })?;

        let user = state
            .get_user_by_id
            .execute(session.user_id.clone())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(serde_json::json!({"error": "Error interno"})),
                )
            })?;

        Ok(CurrentUser { user })
    }
}
