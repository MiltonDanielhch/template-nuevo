// crates/api_server/src/routes.rs
//! # Módulo de Rutas
//!
//! Define el router principal de la aplicación Axum y asocia las rutas
//! con sus respectivos handlers.

use crate::{
    config::di::AppState,
    entry_points::api::v1::user_handlers::{
        login_user_handler, logout_handler, me_handler, register_user_handler,
    },
};
use axum::routing::{Router, get, post};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/me", get(me_handler))
        .route("/logout", post(logout_handler))
        .with_state(app_state)
}
