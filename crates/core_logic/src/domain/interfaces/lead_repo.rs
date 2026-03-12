// crates/core_logic/src/domain/interfaces/lead_repo.rs
//! # Puerto ILeadRepository
//!
//! Contrato para persistencia de leads capturados desde la landing page.
//!
//! Permite que la aplicación decida dónde y cómo guardar los datos (SQL, NoSQL, archivo, etc.).

use crate::domain::entities::lead::Lead;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ILeadRepository: Send + Sync {
    /// Guarda un lead en la persistencia.
    async fn save(&self, lead: &Lead) -> Result<()>;
}
