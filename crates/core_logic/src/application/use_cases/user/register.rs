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
    interfaces::{IHasher, IUserRepository, IRoleRepository},
    value_objects::Email,
};
use anyhow::{Context, Result};
use std::sync::Arc;

/// DTO para el comando de registrar un nuevo usuario.
pub struct RegisterUserCommand {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub role: Option<String>,
}

/// Caso de uso para registrar un nuevo usuario.
/// Orquesta la lógica de hashear la contraseña y guardar el usuario en la base de datos.
pub struct RegisterUser {
    user_repo: Arc<dyn IUserRepository>,
    hasher: Arc<dyn IHasher>,
    role_repo: Arc<dyn IRoleRepository>,
}

impl RegisterUser {
    /// Crea una nueva instancia del caso de uso `RegisterUser`.
    pub fn new(user_repo: Arc<dyn IUserRepository>, hasher: Arc<dyn IHasher>, role_repo: Arc<dyn IRoleRepository>) -> Self {
        Self { user_repo, hasher, role_repo }
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
        let mut new_user = User::new(email, password_hash, command.username);

        // 4.1 Set avatar_url if provided
        if let Some(avatar_url) = command.avatar_url {
            new_user.set_avatar_url(Some(avatar_url));
        }

        // 5. Guardar el usuario usando el repositorio
        self.user_repo
            .save(&new_user)
            .await
            .context("Error al guardar el usuario en la base de datos")?;

        // 6. Asignar rol si se proporcionó
        if let Some(role_name) = command.role {
            if let Ok(Some(role)) = self.role_repo.find_role_by_name(&role_name).await {
                let _ = self.role_repo.assign_role_to_user(new_user.id(), role.id()).await;
            }
        }

        // 7. Devolver la entidad creada
        Ok(new_user)
    }
}
