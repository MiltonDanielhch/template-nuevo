// crates/api_server/src/routes.rs
//! # Módulo de Rutas
//!
//! Define el router principal de la aplicación Axum y asocia las rutas
//! con sus respectivos handlers.

use crate::{config::di::AppState, entry_points::api::v1::user_handlers::register_user_handler};
use axum::routing::{Router, post};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_user_handler))
        .with_state(app_state)
}
