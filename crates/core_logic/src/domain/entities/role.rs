// crates/core_logic/src/domain/entities/role.rs
//! # Entidad: Role
//!
//! Representa un Rol en el dominio. Un Rol es un conjunto nombrado de permisos
//! que puede ser asignado a usuarios para controlar su acceso al sistema.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{Timestamp, Uuid};

/// Identificador único de un Rol (UUIDv7).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleId(String);

impl RoleId {
    pub fn new() -> Self {
        let ts = Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts).to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RoleId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RoleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Identificador único de un Permiso (UUIDv7).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PermissionId(String);

impl PermissionId {
    pub fn new() -> Self {
        let ts = Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts).to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PermissionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for PermissionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Entidad Permission: representa un permiso granular del sistema.
/// Ejemplo: "users:read", "users:write", "roles:manage".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    id: PermissionId,
    name: String,
    description: Option<String>,
}

impl Permission {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: PermissionId::new(),
            name,
            description,
        }
    }

    pub fn from_persistence(id: PermissionId, name: String, description: Option<String>) -> Self {
        Self { id, name, description }
    }

    pub fn id(&self) -> &PermissionId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Entidad Role: representa un rol que agrupa permisos y puede asignarse a usuarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    id: RoleId,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    permissions: Vec<Permission>,
}

impl Role {
    /// Constructor para nuevos roles.
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: RoleId::new(),
            name,
            description,
            created_at: Utc::now(),
            permissions: Vec::new(),
        }
    }

    /// Constructor para reconstruir desde la persistencia.
    pub fn from_persistence(
        id: RoleId,
        name: String,
        description: Option<String>,
        created_at: DateTime<Utc>,
        permissions: Vec<Permission>,
    ) -> Self {
        Self { id, name, description, created_at, permissions }
    }

    pub fn id(&self) -> &RoleId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    /// Verifica si el rol contiene un permiso específico por nombre.
    pub fn has_permission(&self, permission_name: &str) -> bool {
        self.permissions.iter().any(|p| p.name() == permission_name)
    }
}
