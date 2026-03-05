use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("El email '{0}' no es válido.")]
    InvalidEmail(String),
    #[error("La contraseña es demasiado débil.")]
    WeakPassword,
    #[error("El nombre de usuario no puede estar vacío.")]
    EmptyName,
}
