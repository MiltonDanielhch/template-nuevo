// crates/api_server/src/entry_points/middleware/rbac.rs
//! # Middleware de RBAC
//!
//! Implementa el extractor `RequirePermission` para proteger rutas
//! basadas en permisos específicos.

use crate::{config::di::AppState, entry_points::auth::CurrentUser};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use std::marker::PhantomData;

/// Define el nombre de un permiso.
pub trait Permission {
    const NAME: &'static str;
}

/// Extractor que verifica si el usuario tiene un permiso específico.
/// Uso: `RequirePermission<UsersRead>` donde `UsersRead` implementa `Permission`.
pub struct RequirePermission<P: Permission> {
    _marker: PhantomData<P>,
}

impl<P: Permission> FromRequestParts<AppState> for RequirePermission<P> {
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Obtener el usuario actual
        let current_user = CurrentUser::from_request_parts(parts, state).await?;

        // 2. Verificar el permiso
        let has_permission = state
            .role_repo
            .user_has_permission(current_user.user.id(), P::NAME)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(serde_json::json!({"error": "Error interno verificando permisos"})),
                )
            })?;

        if !has_permission {
            return Err((
                StatusCode::FORBIDDEN,
                axum::Json(serde_json::json!({
                    "error": format!("No tienes el permiso requerido: {}", P::NAME)
                })),
            ));
        }

        Ok(Self {
            _marker: PhantomData,
        })
    }
}

// Ejemplo de permisos como marcadores
pub struct RolesWrite;
impl Permission for RolesWrite {
    const NAME: &'static str = "roles:write";
}

pub struct RolesRead;
impl Permission for RolesRead {
    const NAME: &'static str = "roles:read";
}
