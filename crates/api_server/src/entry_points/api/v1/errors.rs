// crates/api_server/src/entry_points/api/v1/errors.rs
//! # Errores de la API v1
//!
//! Define la estructura de errores para la capa de API, permitiendo
//! una conversión limpia de errores de aplicación a respuestas HTTP.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use core_logic::domain::errors::DomainError;
use serde_json::json;

/// Estructura de error unificada para la API.
/// Esto nos permite convertir diferentes tipos de errores en respuestas HTTP consistentes.
pub struct ApiError(pub anyhow::Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Hacemos "downcasting" para identificar el tipo de error específico.
        if let Some(domain_error) = self.0.downcast_ref::<DomainError>() {
            let (status, error_message) = match domain_error {
                // Se corrige el patrón para que coincida con la variante que tiene datos.
                // El `_` ignora el email específico del error, que no necesitamos en la respuesta JSON.
                DomainError::UserAlreadyExists(_) => {
                    (StatusCode::CONFLICT, "El email ya está en uso.")
                }
                DomainError::InvalidCredentials => {
                    (StatusCode::UNAUTHORIZED, "Credenciales inválidas.")
                }
                DomainError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
                DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
                _ => (
                    StatusCode::BAD_REQUEST,
                    "Error en los datos proporcionados.",
                ),
            };
            return (status, Json(json!({ "error": error_message }))).into_response();
        }

        // Para cualquier otro tipo de error (infraestructura, etc.), devolvemos un 500 genérico.
        // En producción, es crucial loggear `self.0` aquí para no perder visibilidad.
        eprintln!("🚨 Error interno no manejado: {:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Ha ocurrido un error interno en el servidor."})),
        )
            .into_response()
    }
}

/// Permite usar el operador `?` en los handlers de Axum que devuelven `Result<_, ApiError>`.
/// Cualquier error que se pueda convertir a `anyhow::Error` se convertirá en nuestro `ApiError`.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
