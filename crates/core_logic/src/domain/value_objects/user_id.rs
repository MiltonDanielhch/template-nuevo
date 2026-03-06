// crates/core_logic/src/domain/value_objects/user_id.rs
//! # Value Object: UserId
//!
//! Representa el identificador único de un usuario.
//! Su propósito es encapsular la lógica de generación y validación de IDs,
//! asegurando que sean únicos y consistentes en todo el sistema.
//!
//! ## Lógica de Negocio
//! - Utiliza UUIDv7 para la generación de nuevos IDs. Esto es óptimo para
//!   claves primarias en bases de datos, ya que son cronológicamente ordenables
//!   y evitan la fragmentación de índices.
//!
//! ## Dependencias
//! - `DomainError`: Para un futuro manejo de errores de validación.
//! - `uuid`: Para la generación y manipulación de UUIDs.
//! - `serde`: Para poder serializar y deserializar el objeto.

use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new() -> Self {
        let ts = Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts).to_string())
    }

    pub fn new_from_string(uuid_str: String) -> Result<Self, DomainError> {
        // Aquí podríamos añadir validación de formato UUID si fuera necesario.
        Ok(Self(uuid_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}
