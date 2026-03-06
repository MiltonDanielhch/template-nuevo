// crates/infra_db/src/external_services/hashing.rs
//! # Adaptador Argon2idHasher
//!
//! Implementación concreta (Adaptador) del puerto `IHasher`.
//! Utiliza la librería `argon2` para proporcionar un hashing de contraseñas
//! robusto y seguro.
//!
//! Dependencias:
//! - `core_logic`: Para acceder al trait `IHasher` y al `PasswordHash`.
//! - `anyhow`: Para el manejo de errores.
//! - `argon2`: La librería criptográfica que hace el trabajo pesado.
//! - `rand`: Para generar la `salt` aleatoria.
//! - `tokio`: Para ejecutar la operación de hashing (intensiva en CPU) en un hilo bloqueante.

use anyhow::Result;
use argon2::{
    Argon2, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use async_trait::async_trait;
use core_logic::domain::interfaces::IHasher;
use core_logic::domain::value_objects::PasswordHash;

/// Adaptador que implementa el puerto `IHasher` utilizando el algoritmo Argon2id.
#[derive(Default)]
pub struct Argon2idHasher;

#[async_trait]
impl IHasher for Argon2idHasher {
    /// Hashea la contraseña usando Argon2id con una salt aleatoria.
    /// La operación es intensiva en CPU y se ejecuta en un hilo bloqueante.
    async fn hash(&self, password: &str) -> Result<PasswordHash> {
        let password_bytes = password.as_bytes().to_owned();
        let hash = tokio::task::spawn_blocking(move || {
            let salt = SaltString::generate(&mut OsRng);
            Argon2::default()
                .hash_password(&password_bytes, &salt)
                .map(|h| h.to_string())
                .map_err(|e| anyhow::anyhow!("Error al hashear la contraseña: {}", e))
        })
        .await??;

        PasswordHash::new(hash).map_err(Into::into)
    }

    /// Verifica la contraseña contra el hash.
    /// La operación es intensiva en CPU y se ejecuta en un hilo bloqueante.
    async fn verify(&self, password: &str, hash: &PasswordHash) -> Result<bool> {
        let hash_str = hash.as_str().to_owned();
        let password_bytes = password.as_bytes().to_owned();

        let is_valid = tokio::task::spawn_blocking(move || -> Result<bool> {
            let parsed_hash = argon2::PasswordHash::new(&hash_str)
                .map_err(|e| anyhow::anyhow!("Error al parsear el hash de la contraseña: {}", e))?;
            Ok(Argon2::default()
                .verify_password(&password_bytes, &parsed_hash)
                .is_ok())
        })
        .await??;

        Ok(is_valid)
    }
}
