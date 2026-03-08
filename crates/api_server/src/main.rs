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
use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::{net::SocketAddr, path::Path};
use tokio::net::TcpSocket;

#[tokio::main]
async fn main() {
    // Carga las variables de entorno desde el archivo .env
    dotenv().ok();

    // Cargar configuración del servidor (Puerto y Host)
    let config = ServerConfig::from_env();

    // --- Sintonía de Base de Datos ---
    // 1. Crear el pool de conexiones a la base de datos.
    let db_url = env::var("DATABASE_URL")
        .expect("Error Crítico: La variable de entorno DATABASE_URL no está definida.");
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Error Crítico: No se pudo conectar a la base de datos.");

    // 2. Ejecutar las migraciones de SQLx.
    // La ruta debe ser relativa al WORKDIR del contenedor (`/app`).
    println!("📡 Sintonizando la base de datos: aplicando migraciones...");

    // Sintonía de Rutas: Buscamos las migraciones tanto en local (Workspace) como en Docker.
    let migrations_path = if Path::new("./migrations").exists() {
        Path::new("./migrations")
    } else if Path::new("crates/infra_db/migrations").exists() {
        Path::new("crates/infra_db/migrations")
    } else {
        panic!("Error Crítico: No se encontraron las migraciones. Se buscaron en './migrations' (Docker) y 'crates/infra_db/migrations' (Local).");
    };

    let migrator = Migrator::new(migrations_path)
        .await
        .expect("Error Crítico: No se pudo inicializar el migrador.");
    migrator
        .run(&pool)
        .await
        .expect("Error Crítico: Fallaron las migraciones de la base de datos.");
    println!("✅ Migraciones aplicadas correctamente.");

    // 1. Construir el AppState (Inyección de Dependencias)
    let app_state = create_app_state(pool);

    // 2. Crear el Router de la aplicación
    let app = create_router(app_state);

    // 3. Iniciar el servidor
    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: SocketAddr = addr
        .parse()
        .expect("No se pudo parsear la dirección del servidor");

    // Sintonía de Sockets: Usamos TcpSocket para configurar SO_REUSEADDR.
    // Esto permite reiniciar el servidor rápidamente en desarrollo sin esperar
    // a que el sistema operativo libere el socket, evitando el error "Address already in use".
    let socket = if socket_addr.is_ipv4() {
        TcpSocket::new_v4()
    } else {
        TcpSocket::new_v6()
    }
    .expect("No se pudo crear el socket TCP");
    socket
        .set_reuseaddr(true)
        .expect("No se pudo configurar SO_REUSEADDR");
    socket.bind(socket_addr).unwrap_or_else(|e| panic!("Error Crítico: No se pudo enlazar a {}: {}", addr, e));
    let listener = socket.listen(1024).expect("No se pudo escuchar en el socket");

    println!(
        "🚀 Servidor API 3026 listo para la sintonía en {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app)
        .await
        .expect("Error Crítico: El servidor falló en tiempo de ejecución.");
}
