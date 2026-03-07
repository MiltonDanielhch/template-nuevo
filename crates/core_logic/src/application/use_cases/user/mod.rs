// crates/core_logic/src/application/use_cases/user/mod.rs
//! # Casos de Uso de Usuario
//!
//! Contiene toda la lógica de aplicación relacionada con la entidad `User`,
//! como el registro, inicio de sesión, actualización de perfil, etc.

pub mod create_session;
pub mod get_user_by_id;
pub mod login;
pub mod register;

pub use create_session::CreateSession;
pub use get_user_by_id::GetUserById;
pub use login::LoginUser;
pub use register::RegisterUser;
