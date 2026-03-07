// crates/api_server/src/main.rs
//! # Punto de Entrada del Servidor API (Main)
//!
//! Este es el binario principal que arranca el servidor web, el punto de inicio
//! de la ejecución del binario `api_server`.
//!
//! ## Responsabilidades
//! - Cargar las variables de entorno desde `.env` (Sintonía 3026).
//! - Inicializar el `AppState` a través del "Composition Root" (`di.rs`).
//! - Crear el `Router` de Axum con las rutas de la API.
//! - Iniciar el servidor y escuchar en un puerto específico (ej. 8080).
//!
//! ## Sintonía 3026
//! - Utiliza `tokio::main` para arrancar el runtime asíncrono.
//! - El uso de `expect` es aceptable aquí, ya que si la configuración
//!   esencial (como la URL de la base de datos o el puerto) falla, la aplicación
//!   no puede arrancar y debe entrar en pánico inmediatamente.

use api_server::{
    config::{di::create_app_state, env::ServerConfig},
    routes::create_router,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();

    // Cargar configuración del servidor (Puerto y Host)
    let config = ServerConfig::from_env();

    // 1. Construir el AppState (Inyección de Dependencias)
    let app_state = create_app_state()
        .await
        .expect("Error Crítico: No se pudo crear el AppState. Revisa la conexión a la DB y las variables de entorno.");

    // 2. Crear el Router de la aplicación
    let app = create_router(app_state);

    // 3. Iniciar el servidor
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        panic!("Error Crítico: No se pudo enlazar a {}: {}", addr, e);
    });

    println!(
        "🚀 Servidor API 3026 listo para la sintonía en {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app)
        .await
        .expect("Error Crítico: El servidor falló en tiempo de ejecución.");
}
