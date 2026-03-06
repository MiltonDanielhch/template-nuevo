// crates/infra_db/src/persistence/mod.rs
//! # Módulo de Persistencia
//!
//! Este módulo agrupa los diferentes adaptadores de persistencia
//! que la aplicación puede utilizar. Cada sub-módulo representa una
//! implementación para una base de datos específica (ej. SQLite, SurrealDB).
//!
//! ## Responsabilidades
//! - Actuar como un directorio para las implementaciones de la capa de persistencia.
//!
//! ## Dependencias
//! - `sqlite`: La implementación concreta para la base de datos SQLite.

pub mod sqlite;
