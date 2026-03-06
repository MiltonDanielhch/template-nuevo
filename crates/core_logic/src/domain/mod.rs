// crates/core_logic/src/domain/mod.rs
//! # Módulo de Dominio
//!
//! Este es el núcleo puro de la aplicación (Capa 1).
//! Contiene toda la lógica de negocio, reglas, entidades y contratos (interfaces),
//! sin ninguna dependencia de frameworks, bases de datos o librerías externas
//! que no sean utilitarias (como `serde` o `thiserror`).
//!
//! ## Responsabilidades
//! - Definir los objetos del negocio (`entities`, `value_objects`).
//! - Definir las reglas de validación y errores de negocio (`errors`).
//! - Definir los contratos para interactuar con el mundo exterior (`interfaces`).

pub mod entities;
pub mod errors;
pub mod interfaces;
pub mod value_objects;
