// crates/core_logic/src/lib.rs

//! # Core Logic Crate
//!
//! El corazón de la aplicación. Contiene la lógica de negocio pura,
//! agnóstica a la infraestructura (web, base de datos, etc.).
//!
//! Se divide en dos capas principales:
//! - **Domain**: Entidades, Value Objects, e interfaces (Puertos).
//! - **Application**: Casos de uso que orquestan la lógica del dominio.
pub mod adapters;
pub mod application;
pub mod domain;
pub mod proto_generated;
