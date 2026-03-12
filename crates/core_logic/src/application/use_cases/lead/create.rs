// crates/core_logic/src/application/use_cases/lead/create.rs
//! Caso de uso: CreateLead
//!
//! Permite registrar un nuevo lead (contacto) en el sistema.

use crate::domain::{entities::lead::Lead, interfaces::ILeadRepository, value_objects::Email};
use anyhow::{Context, Result};
use std::sync::Arc;

/// DTO para crear un lead.
pub struct CreateLeadCommand {
    pub email: String,
    pub name: Option<String>,
}

/// Caso de uso que persiste un lead en el repositorio.
pub struct CreateLead {
    lead_repo: Arc<dyn ILeadRepository>,
}

impl CreateLead {
    pub fn new(lead_repo: Arc<dyn ILeadRepository>) -> Self {
        Self { lead_repo }
    }

    pub async fn execute(&self, command: CreateLeadCommand) -> Result<Lead> {
        let email = Email::parse(command.email).context("Error al parsear el email del lead")?;

        let lead = Lead::new(email, command.name);
        self.lead_repo
            .save(&lead)
            .await
            .context("Error al guardar el lead")?;
        Ok(lead)
    }
}
