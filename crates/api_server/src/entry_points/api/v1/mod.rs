// crates/api_server/src/entry_points/api/v1/mod.rs
//! # API Versión 1 (`/api/v1`)
//!
//! Este módulo agrupa todos los componentes relacionados con la versión 1 de la API.
//!
//! ## Responsabilidades
//! - Contener los `handlers` (controladores) para las rutas de esta versión.
//! - Definir los tipos de `errores` específicos de la API para esta versión.
//! - Servir como punto de organización para el versionado. Si en el futuro
//!   se crea una `v2`, se crearía una carpeta paralela `v2/`.

pub mod errors;
pub mod landing_handlers;
pub mod role_handlers;
pub mod user_handlers;
