// crates/core_logic/src/application/use_cases/user/register.rs
//! # Caso de Uso: RegisterUser
//!
//! Orquesta la lógica de negocio para registrar un nuevo usuario.
//! Este es el corazón de la capa de aplicación para esta funcionalidad.
//!
//! ## Responsabilidades
//! 1. Recibir datos de entrada (un `RegisterUserCommand`).
//! 2. Validar y crear los `Value Objects` necesarios (ej. `Email`).
//! 3. Invocar al servicio de hashing (`IHasher`) para asegurar la contraseña.
//! 4. Crear la entidad de dominio `User`.
//! 5. Invocar al repositorio (`IUserRepository`) para persistir el nuevo usuario.
//! 6. Devolver el resultado.
//!
//! ## Dependencias (Inyectadas)
//! - `IUserRepository`: Para la persistencia de usuarios.
//! - `IHasher`: Para el hashing de contraseñas.
//!
//! Como se puede ver, este caso de uso depende de abstracciones (puertos), no de
//! implementaciones concretas, cumpliendo con la Arquitectura Hexagonal.

use crate::domain::{
    entities::user::User,
    errors::DomainError,
    interfaces::{IHasher, IUserRepository},
    value_objects::Email,
};
use anyhow::{Context, Result};
use std::sync::Arc;

/// DTO para el comando de registrar un nuevo usuario.
pub struct RegisterUserCommand {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
}

/// Caso de uso para registrar un nuevo usuario.
/// Orquesta la lógica de hashear la contraseña y guardar el usuario en la base de datos.
pub struct RegisterUser {
    user_repo: Arc<dyn IUserRepository>,
    hasher: Arc<dyn IHasher>,
}

impl RegisterUser {
    /// Crea una nueva instancia del caso de uso `RegisterUser`.
    pub fn new(user_repo: Arc<dyn IUserRepository>, hasher: Arc<dyn IHasher>) -> Self {
        Self { user_repo, hasher }
    }

    /// Ejecuta el caso de uso.
    pub async fn execute(&self, command: RegisterUserCommand) -> Result<User> {
        // 1. Validar y crear Value Objects
        let email =
            Email::parse(command.email).context("Error al parsear el email en el caso de uso")?;

        // 2. Verificar que el email no esté ya en uso
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists(email.as_str().to_string()).into());
        }

        // 3. Hashear la contraseña
        let password_hash = self
            .hasher
            .hash(&command.password)
            .await
            .context("Error al hashear la contraseña")?;

        // 4. Crear la entidad de dominio User
        let new_user = User::new(email, password_hash, command.username);

        // 5. Guardar el usuario usando el repositorio
        self.user_repo
            .save(&new_user)
            .await
            .context("Error al guardar el usuario en la base de datos")?;

        // 6. Devolver la entidad creada (o un DTO de respuesta si se prefiere)
        Ok(new_user)
    }
}
