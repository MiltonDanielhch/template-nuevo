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