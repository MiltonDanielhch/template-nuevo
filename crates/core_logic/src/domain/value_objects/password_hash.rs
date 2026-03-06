// crates/core_logic/src/domain/value_objects/password_hash.rs
//! # Value Object: PasswordHash
//!
//! Representa un hash de contraseña, no una contraseña en texto plano.
//! Su propósito es proporcionar seguridad de tipos, evitando que accidentalmente
//! se pueda usar un hash como si fuera texto plano o viceversa.
//!
//! ## Lógica de Negocio
//! - Un hash de contraseña no puede estar vacío.
//!
//! ## Dependencias
//! - `DomainError`: Para devolver un error de dominio si la validación falla.
//! - `serde`: Para poder serializar y deserializar el objeto (ej. en APIs).

use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Result<Self, DomainError> {
        if hash.trim().is_empty() {
            return Err(DomainError::InvalidPasswordHash("El hash de la contraseña no puede estar vacío".to_string()));
        }
        Ok(Self(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}