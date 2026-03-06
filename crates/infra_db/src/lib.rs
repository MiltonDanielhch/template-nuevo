//! crates/infra_db/src/lib.rs
//! # Crate de Infraestructura (`infra_db`)
//!
//! Este crate contiene los "Adaptadores de Salida" de la Arquitectura Hexagonal.
//! Es el puente entre la lógica de negocio pura (`core_logic`) y las
//! tecnologías concretas del mundo exterior, como bases de datos y servicios de terceros.
//!
//! ## Responsabilidades
//! - Implementar los `traits` (puertos) definidos en `core_logic`.
//! - Contener toda la lógica de interacción con la base de datos (ej. SQLx para SQLite).
//! - Contener implementaciones para servicios externos (ej. hashing, envío de emails).
//! - Exponer públicamente las implementaciones concretas (structs) para que el
//!   `Composition Root` en `api_server` pueda construirlas e inyectarlas.

pub mod external_services;
pub mod persistence;

// Exponemos públicamente el repositorio concreto para que el 'Composition Root' (en api_server) pueda instanciarlo.
pub use persistence::sqlite::repositories::sqlite_user_repo::SqliteUserRepository;
// Exponemos públicamente el hasher concreto para que el 'Composition Root' pueda instanciarlo.
pub use external_services::Argon2idHasher;
