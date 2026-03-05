use crate::domain::{entities::user::User, value_objects::email::Email};
use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

/// IUserRepository es un Puerto en la Arquitectura Hexagonal.
///
/// Define el contrato que la capa de aplicación utiliza para interactuar
/// con la persistencia de usuarios, sin conocer la implementación subyacente
/// (SQL, NoSQL, etc.).
///
/// El trait es `Send + Sync` para poder ser compartido de forma segura entre hilos,
/// lo cual es un requisito para el estado de Axum.
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>>;
}