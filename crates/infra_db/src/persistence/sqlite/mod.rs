// crates/infra_db/src/persistence/sqlite/mod.rs
//! # Módulo de Persistencia SQLite
//!
//! Este módulo agrupa toda la lógica específica para interactuar con una
//! base de datos SQLite. Contiene los adaptadores de repositorio y los
//! modelos de datos de la base de datos.
//!
//! ## Responsabilidades
//! - Contener las implementaciones concretas de los puertos de persistencia para SQLite.
//! - Definir las estructuras que mapean directamente a las tablas de la base de datos.
//!
//! ## Dependencias
//! - `models`: Estructuras que representan las tablas (ej. `DbUser`).
//! - `repositories`: Implementaciones de los traits de repositorio (ej. `SqliteUserRepository`).

pub mod models;
pub mod repositories;