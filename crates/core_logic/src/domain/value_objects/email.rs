// crates/core_logic/src/domain/value_objects/email.rs
//! # Value Object: Email
//!
//! Representa una dirección de correo electrónico validada.
//! Su propósito es garantizar que cualquier `Email` en el sistema
//! ha pasado las comprobaciones de formato necesarias.
//!
//! ## Lógica de Negocio
//! - No puede estar vacío.
//! - Debe tener un formato de email válido (comprobado con una expresión regular).
//!
//! ## Dependencias
//! - `DomainError`: Para devolver un error de dominio si la validación falla.
//! - `regex`: Para la validación del formato.
//! - `OnceLock`: Para compilar la regex una sola vez y mejorar el rendimiento (Sintonía 3026).
//! - `serde`: Para poder serializar y deserializar el objeto.

use crate::domain::errors::DomainError;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(value: String) -> Result<Self, DomainError> {
        if value.trim().is_empty() {
            return Err(DomainError::InvalidEmail("Email vacío".to_string()));
        }

        // Regex compilado una sola vez para rendimiento (Sintonía 3026)
        static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap()
        });

        if !regex.is_match(&value) {
            return Err(DomainError::InvalidEmail(value));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
