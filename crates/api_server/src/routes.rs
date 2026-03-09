// crates/api_server/src/routes.rs
//! # Módulo de Rutas
//!
//! Define el router principal de la aplicación Axum y asocia las rutas
//! con sus respectivos handlers.

use crate::{
    config::di::AppState,
    entry_points::api::v1::user_handlers::{
        delete_user_handler, list_users_handler, login_user_handler, logout_handler, me_handler,
        register_user_handler, update_user_handler,
    },
};
use axum::routing::{Router, delete, get, post, put};
use tower_http::trace::TraceLayer;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/me", get(me_handler))
        .route("/logout", post(logout_handler))
        .route("/users", get(list_users_handler))
        .route("/users/{id}", put(update_user_handler))
        .route("/users/{id}", delete(delete_user_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
