// crates/core_logic/src/application/use_cases/user/login.rs
//! # Caso de Uso: LoginUser
//!
//! Orquesta la lógica de negocio para autenticar un usuario existente.
//! Este es el corazón de la capa de aplicación para esta funcionalidad.
//!
//! ## Responsabilidades
//! 1. Recibir datos de entrada (un `LoginUserCommand`).
//! 2. Buscar al usuario por email usando el repositorio.
//! 3. Verificar la contraseña usando el servicio de hashing.
//! 4. Devolver la entidad de dominio si las credenciales son válidas.
//!
//! ## Dependencias (Inyectadas)
//! - `IUserRepository`: Para buscar usuarios por email.
//! - `IHasher`: Para verificar la contraseña.
//!
//! Como se puede ver, este caso de uso depende de abstracciones (puertos), no de
//! implementaciones concretas, cumpliendo con la Arquitectura Hexagonal.

use crate::domain::{
    entities::user::User,
    errors::DomainError,
    interfaces::{IHasher, IUserRepository},
    value_objects::Email,
};
use anyhow::Result;
use std::sync::Arc;

pub struct LoginUserCommand {
    pub email: String,
    pub password: String,
}

pub struct LoginUser {
    user_repo: Arc<dyn IUserRepository>,
    hasher: Arc<dyn IHasher>,
}

impl LoginUser {
    pub fn new(user_repo: Arc<dyn IUserRepository>, hasher: Arc<dyn IHasher>) -> Self {
        Self { user_repo, hasher }
    }

    pub async fn execute(&self, command: LoginUserCommand) -> Result<User, DomainError> {
        let email = Email::parse(command.email).map_err(|_| DomainError::InvalidCredentials)?;

        let user = self
            .user_repo
            .find_by_email(&email)
            .await
            .map_err(|_| DomainError::InvalidCredentials)?
            .ok_or(DomainError::InvalidCredentials)?;

        let is_valid = self
            .hasher
            .verify(&command.password, user.password_hash())
            .await
            .map_err(|_| DomainError::InvalidCredentials)?;

        if is_valid {
            Ok(user)
        } else {
            Err(DomainError::InvalidCredentials)
        }
    }
}
