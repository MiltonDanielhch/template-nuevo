// crates/core_logic/src/application/mod.rs
//! # Capa de Aplicación
//!
//! Este módulo define la capa de aplicación en la Arquitectura Hexagonal.
//! Contiene los casos de uso que orquestan la lógica de dominio para
//! realizar tareas específicas del negocio.
//!
//! ## Responsabilidades
//! - Orquestar la obtención y persistencia de entidades de dominio a través de los puertos.
//! - No contener lógica de negocio, solo la secuencia de pasos.
//!
//! ## Dependencias
//! - `domain`: Para acceder a las entidades, VOs y puertos.

pub mod use_cases;