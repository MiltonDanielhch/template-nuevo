// crates/core_logic/src/domain/errors.rs
//! # Errores de Dominio
//!
//! Este módulo define los errores específicos del dominio de negocio.
//! A diferencia de `anyhow::Error`, que se usa para errores inesperados o de
//! infraestructura, `DomainError` representa fallos de validación de reglas
//! de negocio que son predecibles y manejables.
//!
//! ## Responsabilidades
//! - Proporcionar tipos de error claros y específicos para cada regla de negocio.
//!
//! ## Dependencias
//! - `thiserror`: Para derivar el trait `std::error::Error` de forma declarativa.

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("El email '{0}' no es válido.")]
    InvalidEmail(String),

    #[error("El hash de la contraseña no es válido: {0}")]
    InvalidPasswordHash(String),
}
