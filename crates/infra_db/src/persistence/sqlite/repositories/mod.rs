// crates/infra_db/src/persistence/sqlite/repositories/mod.rs
//! # Módulo de Repositorios de SQLite
//!
//! Este módulo agrupa y exporta todas las implementaciones concretas
//! de los repositorios (Adaptadores) que utilizan SQLite como backend.
//!
//! ## Responsabilidades
//! - Contener los archivos que implementan los traits de repositorio definidos en `core_logic`.
//!
//! ## Dependencias
//! - `sqlite_user_repo`: La implementación para `IUserRepository`.

pub mod sqlite_session_repo;
pub mod sqlite_user_repo;

pub use sqlite_session_repo::SqliteSessionRepository;
pub use sqlite_user_repo::SqliteUserRepository;
