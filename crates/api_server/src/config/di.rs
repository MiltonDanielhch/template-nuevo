// crates/api_server/src/config/di.rs
//! # Inyección de Dependencias (Composition Root)
//!
//! Este módulo es el **corazón de la inversión de control** y el único lugar
//! en toda la aplicación donde las implementaciones concretas se conocen y se instancian.
//! Actúa como el "Composition Root" de la Arquitectura Hexagonal.
//!
//! ## Responsabilidades
//! - Crear y configurar el pool de conexiones a la base de datos.
//! - Instanciar los repositorios concretos (ej. `SqliteUserRepository`).
//! - Instanciar los servicios externos concretos (ej. `Argon2idHasher`).
//! - Envolver las instancias en `Arc<dyn Trait>` para:
//!   1.  **Abstracción:** El resto de la app solo conoce el `trait`, no la implementación.
//!   2.  **Compartición Segura:** `Arc` permite compartir la misma instancia entre múltiples
//!       hilos (requests) de forma segura y eficiente, evitando clonaciones costosas.
//! - Construir el `AppState` que se compartirá en toda la aplicación Axum.

use core_logic::{
    application::use_cases::user::{
        CreateSession, delete::DeleteUser, get_user_by_id::GetUserById, list::ListUsers,
        login::LoginUser, register::RegisterUser, update::UpdateUser,
    },
    domain::interfaces::ISessionRepository,
};
use infra_db::{Argon2idHasher, SqliteSessionRepository, SqliteUserRepository};
use std::sync::Arc;

/// Estado de la aplicación compartido a través de los handlers de Axum.
/// Contiene todas las dependencias (casos de uso, repositorios, etc.)
/// que los `entry_points` (handlers) necesitan para funcionar.
#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<RegisterUser>,
    pub login_user: Arc<LoginUser>,
    pub list_users: Arc<ListUsers>,
    pub update_user: Arc<UpdateUser>,
    pub delete_user: Arc<DeleteUser>,
    pub create_session: Arc<CreateSession>,
    pub session_repo: Arc<dyn ISessionRepository>,
    pub get_user_by_id: Arc<GetUserById>,
}

pub fn create_app_state(pool: sqlx::SqlitePool) -> AppState {
    let user_repo = Arc::new(SqliteUserRepository::new(pool.clone()));
    let session_repo = Arc::new(SqliteSessionRepository::new(pool.clone()));
    let hasher = Arc::new(Argon2idHasher {});

    let register_user = Arc::new(RegisterUser::new(user_repo.clone(), hasher.clone()));
    let login_user = Arc::new(LoginUser::new(user_repo.clone(), hasher.clone()));
    let list_users = Arc::new(ListUsers::new(user_repo.clone()));
    let update_user = Arc::new(UpdateUser::new(user_repo.clone(), hasher.clone()));
    let delete_user = Arc::new(DeleteUser::new(user_repo.clone()));
    let create_session = Arc::new(CreateSession::new(session_repo.clone()));
    let get_user_by_id = Arc::new(GetUserById::new(user_repo.clone()));

    AppState {
        register_user,
        login_user,
        list_users,
        update_user,
        delete_user,
        create_session,
        session_repo,
        get_user_by_id,
    }
}
