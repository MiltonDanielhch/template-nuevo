// crates/core_logic/src/domain/interfaces/hasher.rs
//! # Puerto IHasher
//!
//! Define el contrato (la interfaz) para los servicios de hashing de contraseñas.
//! Al ser un `trait` en el `core_logic`, permite que la lógica de aplicación
//! dependa de una abstracción y no de una implementación concreta.
//!
//! Dependencias:
//! - `anyhow`: Para el manejo de errores.
//! - `async_trait`: Para permitir métodos `async` en el trait.
//! - `PasswordHash`: El Value Object que representa un hash seguro.

use crate::domain::value_objects::PasswordHash;
use anyhow::Result;
use async_trait::async_trait;

/// Puerto para el servicio de hashing de contraseñas.
/// Define un contrato para hashear y verificar contraseñas,
/// permitiendo que la implementación concreta (ej. Argon2id, Bcrypt) sea intercambiable.
#[async_trait]
pub trait IHasher: Send + Sync {
    /// Hashea una contraseña en texto plano.
    async fn hash(&self, password: &str) -> Result<PasswordHash>;
    /// Verifica una contraseña en texto plano contra un hash existente.
    async fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool>;
}
