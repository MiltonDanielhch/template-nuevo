//! crates/core_logic/src/domain/interfaces/mod.rs
//! # Módulo de Interfaces (Puertos)
//!
//! Este módulo define los "Puertos" en la Arquitectura Hexagonal.
//! Contiene los `traits` que la lógica de negocio (`application` y `domain`)
//! utiliza para comunicarse con el mundo exterior (infraestructura).
//!
//! ## Responsabilidades
//! - Definir contratos abstractos para la persistencia, servicios externos, etc.
//! - Ser la única dependencia del dominio hacia el exterior.
//!
//! ## Dependencias
//! - Ninguna, más allá de los tipos definidos en el propio dominio.

pub mod hasher;
pub mod user_repo;

// Exponemos los traits (puertos) que el dominio define.
pub use self::hasher::IHasher;
pub use self::user_repo::IUserRepository;