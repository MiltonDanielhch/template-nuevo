// crates/core_logic/src/domain/entities/lead.rs
//! # Entidad: Lead
//!
//! Representa un registro de contacto capturado desde la landing.
//!
//! Esta entidad es simple y sirve principalmente para almacenar datos de contacto.

use crate::domain::value_objects::{Email, LeadId};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Lead {
    id: LeadId,
    email: Email,
    name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// DTO para reconstruir un `Lead` desde la persistencia.
pub struct LeadPersistenceData {
    pub id: LeadId,
    pub email: Email,
    pub name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Lead {
    pub fn new(email: Email, name: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: LeadId::new(),
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_from_persistence(data: LeadPersistenceData) -> Self {
        Self {
            id: data.id,
            email: data.email,
            name: data.name,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }

    pub fn id(&self) -> &LeadId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}
