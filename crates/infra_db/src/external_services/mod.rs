// crates/infra_db/src/external_services/mod.rs
//! # Módulo de Servicios Externos
//!
//! Este módulo contiene adaptadores para servicios que no son la base de datos
//! principal, pero que son necesarios para la infraestructura.
//!
//! Ejemplos:
//! - Servicios de Hashing de contraseñas (Argon2id, Bcrypt).
//! - Servicios de envío de email (Resend, SendGrid).
//! - Servicios de almacenamiento de archivos (S3, Cloudflare R2).

pub mod hashing;

pub use hashing::Argon2idHasher;
