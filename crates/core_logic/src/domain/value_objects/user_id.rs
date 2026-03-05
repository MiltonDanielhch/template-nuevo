use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        let ts = Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts))
    }

    pub fn new_from_uuid(uuid: Uuid) -> Result<Self, DomainError> {
        Ok(Self(uuid))
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}