// crates/core_logic/src/domain/value_objects/lead_id.rs
//! # Value Object: LeadId
//!
//! Identificador único para un lead capturado desde la landing.
//!
//! Usa UUIDv7 para mantener orden cronológico y evitar colisiones.

use std::fmt::{Display, Formatter};

use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LeadId(String);

impl Display for LeadId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LeadId {
    pub fn new() -> Self {
        let ts = Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts).to_string())
    }

    pub fn new_from_string(uuid_str: String) -> Result<Self, DomainError> {
        // Podríamos validar el formato del UUID aquí si fuera necesario.
        Ok(Self(uuid_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for LeadId {
    fn default() -> Self {
        Self::new()
    }
}
