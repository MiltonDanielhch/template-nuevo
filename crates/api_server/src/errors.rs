//! # Manejador de Errores de la API
//!
//! Este módulo define el tipo `AppError` que centraliza el manejo de errores
//! en la capa de la API. Implementa `IntoResponse` para traducir los errores
//! internos (incluyendo los `DomainError` de `core_logic`) en respuestas
//! HTTP apropiadas con sus códigos de estado.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use core_logic::domain::errors::DomainError;
use serde_json::json;
use tracing::{error, warn};

/// El tipo de error unificado para toda la aplicación API.
/// Envuelve un `anyhow::Error` para máxima flexibilidad.
pub struct AppError(pub anyhow::Error);

/// Permite que `?` convierta cualquier `anyhow::Error` en nuestro `AppError`.
impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

/// La magia de la sintonía: traduce `AppError` a una respuesta HTTP.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Buscamos la causa raíz para ver si es un error de dominio conocido.
        let (status, error_message) =
            if let Some(domain_error) = self.0.downcast_ref::<DomainError>() {
                match domain_error {
                    DomainError::UserAlreadyExists(email) => (
                        StatusCode::CONFLICT,
                        format!("El email '{}' ya está en uso.", email),
                    ),
                    DomainError::ValidationError(details) => (
                        StatusCode::BAD_REQUEST,
                        format!("Error de validación: {}", details),
                    ),
                    DomainError::InvalidCredentials => (
                        StatusCode::UNAUTHORIZED,
                        "Credenciales inválidas.".to_string(),
                    ),
                }
            } else {
                // Si no es un error de dominio, es un error 500 inesperado.
                error!("🚨 Error interno no manejado: {:?}", self.0);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Ha ocurrido un error interno en el servidor.".to_string(),
                )
            };

        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
