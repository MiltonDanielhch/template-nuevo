// crates/api_server/tests/integration_tests.rs
//! # Tests de Integración para api_server
//!
//! Este módulo contiene tests de integración para verificar el correcto funcionamiento
//! de los endpoints de la API, asegurando que todas las capas (API, aplicación, dominio, infraestructura)
//! interactúan correctamente.
//!
//! ## Estrategia de Testing
//! - Se utiliza una base de datos SQLite en memoria (`sqlite::memory:`) para cada test,
//!   garantizando aislamiento y un estado inicial limpio.
//! - Se ejecutan las migraciones en la base de datos en memoria antes de cada test.
//! - Se utiliza `tower::ServiceExt` para llamar al `Router` de Axum directamente,
//!   sin necesidad de levantar un servidor HTTP real, lo que hace los tests más rápidos.

use api_server::{
    config::di::AppState,
    routes::create_router,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt; // Para `collect` y `to_bytes`
use serde_json::{json, Value};
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions};
use std::{path::Path, sync::Arc};
use std::sync::LazyLock;
use tokio::sync::OnceCell;
use tower::util::ServiceExt;
use std::env;

/// Migrador estático para las migraciones de SQLx.
/// Se inicializa una única vez para cargar las migraciones desde la ruta.
static MIGRATOR: LazyLock<OnceCell<Migrator>> = LazyLock::new(OnceCell::new);

/// Configura un router de Axum para tests, con una base de datos SQLite en memoria.
///
/// Cada llamada a esta función crea una nueva base de datos en memoria y ejecuta
/// las migraciones, asegurando un entorno de test aislado.
async fn setup_test_app() -> Router {
    // Inicializar el migrador si no se ha hecho ya.
    // La ruta de las migraciones debe ser relativa a la raíz del workspace.
    let migrator = MIGRATOR
        .get_or_init(|| async {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let migrations_path = Path::new(&manifest_dir).join("../infra_db/migrations");
            Migrator::new(migrations_path)
                .await
                .expect("Failed to create migrator")
        })
        .await;

    // Crear un pool de conexiones a una base de datos SQLite en memoria.
    let pool = SqlitePoolOptions::new()
        .max_connections(1) // Solo necesitamos una conexión para tests en memoria
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    // Ejecutar las migraciones en la base de datos en memoria.
    migrator.run(&pool).await.expect("Failed to run migrations");

    // Crear el AppState con el pool de la base de datos en memoria.
    let user_repo = Arc::new(infra_db::SqliteUserRepository::new(pool.clone()));
    let hasher = Arc::new(infra_db::Argon2idHasher::default());
    let register_user_use_case =
        Arc::new(core_logic::application::use_cases::user::register::RegisterUser::new(user_repo, hasher));

    let app_state = AppState {
        register_user: register_user_use_case,
    };

    create_router(app_state)
}

#[tokio::test]
async fn register_user_success() {
    let app = setup_test_app().await;

    let request_body = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/register")
                .header("Content-Type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json_body: Value = serde_json::from_slice(&body).unwrap();

    assert!(json_body["id"].is_string());
    assert_eq!(json_body["email"], "test@example.com");
}

#[tokio::test]
async fn register_user_duplicate_email() {
    let app = setup_test_app().await;

    let request_body = json!({"email": "duplicate@example.com", "password": "password123"});
    let response = app.clone().oneshot(Request::builder().method("POST").uri("/register").header("Content-Type", "application/json").body(Body::from(request_body.to_string())).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Intentar registrar el mismo usuario de nuevo
    let response = app.oneshot(Request::builder().method("POST").uri("/register").header("Content-Type", "application/json").body(Body::from(request_body.to_string())).unwrap()).await.unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json_body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json_body["error"], "El email ya está en uso.");
}
