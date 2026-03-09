// crates/api_server/src/routes.rs
//! # Módulo de Rutas
//!
//! Define el router principal de la aplicación Axum y asocia las rutas
//! con sus respectivos handlers.

use crate::{
    config::di::AppState,
    entry_points::api::v1::{
        role_handlers::{
            assign_role_to_user_handler, create_role_handler, delete_role_handler,
            get_my_permissions_handler, list_permissions_handler, list_roles_handler,
            update_role_handler,
        },
        user_handlers::{
            delete_user_handler, list_users_handler, login_user_handler, logout_handler,
            me_handler, register_user_handler, update_me_handler, update_user_handler,
        },
    },
};
use axum::routing::{Router, get, post, put};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        // Auth & User routes
        .route("/register", post(register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/me", get(me_handler))
        .route("/me", put(update_me_handler))
        .route("/logout", post(logout_handler))
        .route("/users", get(list_users_handler))
        .route("/users/{id}", get(me_handler)) // reuse me logic for one user if needed, or just list
        .route("/users/{id}", axum::routing::put(update_user_handler))
        .route("/users/{id}", axum::routing::delete(delete_user_handler))
        // RBAC routes
        .route("/roles", post(create_role_handler))
        .route("/roles", get(list_roles_handler))
        .route("/roles/{id}", put(update_role_handler))
        .route("/roles/{id}", axum::routing::delete(delete_role_handler))
        .route("/permissions", get(list_permissions_handler))
        .route("/users/{user_id}/roles", post(assign_role_to_user_handler))
        .route("/users/me/permissions", get(get_my_permissions_handler))
        .with_state(app_state)
}
