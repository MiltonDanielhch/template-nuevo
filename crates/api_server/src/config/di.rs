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

use anyhow::Result;
use core_logic::{
    application::use_cases::user::{
        CreateSession, get_user_by_id::GetUserById, login::LoginUser, register::RegisterUser,
    },
    domain::interfaces::{IHasher, ISessionRepository, IUserRepository},
};
use infra_db::{Argon2idHasher, SqliteSessionRepository, SqliteUserRepository};
use sqlx::SqlitePool;
use std::sync::Arc;

/// Estado de la aplicación compartido a través de los handlers de Axum.
/// Contiene todas las dependencias (casos de uso, repositorios, etc.)
/// que los `entry_points` (handlers) necesitan para funcionar.
#[derive(Clone)]
pub struct AppState {
    pub register_user: Arc<RegisterUser>,
    pub login_user: Arc<LoginUser>,
    pub create_session: Arc<CreateSession>,
    pub session_repo: Arc<dyn ISessionRepository>,
    pub get_user_by_id: Arc<GetUserById>,
}

/// Construye y devuelve el estado de la aplicación (`AppState`).
/// Aquí es donde se realiza toda la "magia" de la inyección de dependencias.
pub async fn create_app_state() -> Result<AppState> {
    // 1. Cargar configuración (ej. desde variables de entorno)
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Inicializar recursos externos (Pool de DB)
    let pool = SqlitePool::connect(&database_url).await?;

    // 3. Instanciar adaptadores de infraestructura (implementaciones concretas)
    let user_repo: Arc<dyn IUserRepository> = Arc::new(SqliteUserRepository::new(pool.clone()));
    let session_repo: Arc<dyn ISessionRepository> = Arc::new(SqliteSessionRepository::new(pool));
    let hasher: Arc<dyn IHasher> = Arc::new(Argon2idHasher {});

    // 4. Instanciar casos de uso de la aplicación, inyectando las dependencias
    let register_user = Arc::new(RegisterUser::new(user_repo.clone(), hasher.clone()));
    let login_user = Arc::new(LoginUser::new(user_repo.clone(), hasher.clone()));
    let create_session = Arc::new(CreateSession::new(session_repo.clone()));
    let get_user_by_id = Arc::new(GetUserById::new(user_repo.clone()));

    // 5. Construir y devolver el estado de la aplicación
    Ok(AppState {
        register_user,
        login_user,
        create_session,
        session_repo,
        get_user_by_id,
    })
}
