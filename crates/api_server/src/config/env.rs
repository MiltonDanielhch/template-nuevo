//! # Gestión de Variables de Entorno
//!
//! Este módulo centraliza la lectura y validación de variables de entorno
//! para evitar llamadas dispersas a `std::env::var` y garantizar la sintonía.

use std::env;

/// Estructura que contiene la configuración del servidor
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl ServerConfig {
    /// Carga la configuración desde variables de entorno con valores por defecto seguros
    pub fn from_env() -> Self {
        let port = env::var("SERVER_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8081); // Puerto por defecto: 8081

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        Self { port, host }
    }
}
