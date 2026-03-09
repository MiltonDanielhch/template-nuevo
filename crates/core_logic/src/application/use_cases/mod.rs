// crates/core_logic/src/application/use_cases/mod.rs
//! # Módulo de Casos de Uso
//!
//! Agrupa los casos de uso por contexto de dominio. Cada sub-módulo
//! representa un área de negocio, como `user`, `role`, etc.
//!
//! Esta organización ayuda a mantener la cohesión y a encontrar rápidamente
//! la lógica de aplicación relacionada.

pub mod role;
pub mod user;
