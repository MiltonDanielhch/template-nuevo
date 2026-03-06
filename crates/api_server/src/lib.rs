// crates/api_server/src/lib.rs
//! # Crate `api_server` (Biblioteca)
//!
//! Este archivo es la raíz de la biblioteca del crate binario `api_server`.
//! Organiza todos los módulos internos que componen el servidor web.
//!
//! ## Responsabilidades
//! - Definir y organizar los módulos del servidor:
//!   - `config`: Para la inyección de dependencias (Composition Root) y la configuración.
//!   - `entry_points`: Para los manejadores de rutas (handlers) de Axum.
//!   - `routes`: Para la definición del enrutador de la API.
//! - Definir el estado compartido de la aplicación (`AppState`).
//!
//! ## Dependencias
//! - `core_logic`: Para acceder a los casos de uso.
//! - `infra_db`: Para acceder a las implementaciones concretas de los repositorios.
//! - `axum`: Para construir el servidor web.

pub mod config;
pub mod entry_points;
pub mod routes;
