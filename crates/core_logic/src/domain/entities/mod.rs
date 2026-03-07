// crates/core_logic/src/domain/entities/mod.rs
//! # Módulo de Entidades
//!
//! Este módulo agrupa y exporta todas las `Entidades` del dominio.
//! Las `Entidades` son los objetos principales del negocio, que tienen una
//! identidad única y un ciclo de vida.
//!
//! ## Responsabilidades
//! - Definir la estructura y las reglas de negocio de los agregados principales.
//!
//! ## Dependencias
//! - `value_objects`: Para componer las entidades con atributos validados.
//! - `chrono`: Para el manejo de fechas.

pub mod session;
pub mod user;

pub use session::Session;
pub use user::User;
