use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("El email '{0}' no es válido.")]
    InvalidEmail(String),

    #[error("El hash de la contraseña no es válido: {0}")]
    InvalidPasswordHash(String),
}