// crates/core_logic/src/domain/value_objects/mod.rs
//! # Módulo de Value Objects
//!
//! Este módulo agrupa y exporta todos los `Value Objects` del dominio.
//! Los `Value Objects` son tipos que encapsulan lógica de validación
//! para atributos de las entidades, garantizando que solo datos válidos
//! puedan existir en el núcleo del sistema.
//!
//! No tiene dependencias externas, solo depende de otros módulos del dominio si es necesario.

pub mod email;
pub mod lead_id;
pub mod password_hash;
pub mod session_token;
pub mod user_id;

// Re-exportamos los tipos para facilitar su importación en otras partes del crate.
// Ejemplo: `use crate::domain::value_objects::Email` en lugar de `use crate::domain::value_objects::email::Email`.
pub use email::Email;
pub use lead_id::LeadId;
pub use password_hash::PasswordHash;
pub use session_token::SessionToken;
pub use user_id::UserId;
