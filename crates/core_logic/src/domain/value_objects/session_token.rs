// crates/core_logic/src/domain/value_objects/session_token.rs
//! # Value Object: SessionToken
//!
//! Representa un token de sesión seguro.
//! Su propósito es garantizar la validación y generación segura de tokens.

use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn new(token: String) -> Result<Self, DomainError> {
        if token.trim().is_empty() {
            return Err(DomainError::ValidationError("Token no puede estar vacío".to_string()));
        }
        if token.len() < 32 {
            return Err(DomainError::ValidationError("Token muy corto".to_string()));
        }
        Ok(Self(token))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SessionToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
