// crates/infra_db/src/persistence/sqlite/repositories/sqlite_role_repo.rs
//! # SqliteRoleRepository
//!
//! Implementación concreta del trait `IRoleRepository` para SQLite.
//! Traduce entre entidades de dominio y DTOs de base de datos.

use anyhow::{Context, Result};
use async_trait::async_trait;
use core_logic::domain::{
    entities::role::{Permission, PermissionId, Role, RoleId},
    interfaces::IRoleRepository,
    value_objects::user_id::UserId,
};
use sqlx::SqlitePool;

use crate::persistence::sqlite::models::{DbPermission, DbRole};

pub struct SqliteRoleRepository {
    pool: SqlitePool,
}

impl SqliteRoleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Carga los permisos de un rol a partir de su ID.
    async fn load_permissions_for_role(&self, role_id: &str) -> Result<Vec<Permission>> {
        let rows = sqlx::query_as!(
            DbPermission,
            r#"
            SELECT p.id, p.name, p.description
            FROM permissions p
            INNER JOIN role_permissions rp ON rp.permission_id = p.id
            WHERE rp.role_id = ?
            "#,
            role_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Error cargando permisos del rol")?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Permission::from_persistence(
                    PermissionId::from_string(row.id),
                    row.name,
                    row.description,
                )
            })
            .collect())
    }

    /// Mapea un DbRole a la entidad Role del dominio (con sus permisos).
    async fn map_db_role_to_domain(&self, row: DbRole) -> Result<Role> {
        let permissions = self.load_permissions_for_role(&row.id).await?;
        let created_at = row
            .created_at
            .map(|dt| dt.and_utc())
            .unwrap_or_else(chrono::Utc::now);

        Ok(Role::from_persistence(
            RoleId::from_string(row.id),
            row.name,
            row.description,
            created_at,
            permissions,
        ))
    }
}

#[async_trait]
impl IRoleRepository for SqliteRoleRepository {
    async fn create_role(&self, role: &Role) -> Result<()> {
        let id = role.id().as_str();
        let name = role.name();
        let description = role.description();
        let now = chrono::Utc::now().naive_utc();
        sqlx::query!(
            r#"INSERT INTO roles (id, name, description, created_at) VALUES (?, ?, ?, ?)"#,
            id,
            name,
            description,
            now
        )
        .execute(&self.pool)
        .await
        .context("Error creando rol en DB")?;
        Ok(())
    }

    async fn find_role_by_id(&self, id: &RoleId) -> Result<Option<Role>> {
        let id_str = id.as_str();
        let maybe_row = sqlx::query_as!(
            DbRole,
            r#"SELECT id, name, description, created_at FROM roles WHERE id = ?"#,
            id_str
        )
        .fetch_optional(&self.pool)
        .await
        .context("Error buscando rol por ID")?;

        match maybe_row {
            None => Ok(None),
            Some(row) => Ok(Some(self.map_db_role_to_domain(row).await?)),
        }
    }

    async fn find_role_by_name(&self, name: &str) -> Result<Option<Role>> {
        let maybe_row = sqlx::query_as!(
            DbRole,
            r#"SELECT id, name, description, created_at FROM roles WHERE name = ?"#,
            name
        )
        .fetch_optional(&self.pool)
        .await
        .context("Error buscando rol por nombre")?;

        match maybe_row {
            None => Ok(None),
            Some(row) => Ok(Some(self.map_db_role_to_domain(row).await?)),
        }
    }

    async fn list_roles(&self) -> Result<Vec<Role>> {
        let rows = sqlx::query_as!(
            DbRole,
            r#"SELECT id, name, description, created_at FROM roles ORDER BY created_at ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .context("Error listando roles")?;

        let mut roles = Vec::with_capacity(rows.len());
        for row in rows {
            roles.push(self.map_db_role_to_domain(row).await?);
        }
        Ok(roles)
    }

    async fn assign_role_to_user(&self, user_id: &UserId, role_id: &RoleId) -> Result<()> {
        let uid = user_id.as_str();
        let rid = role_id.as_str();
        sqlx::query!(
            r#"INSERT OR IGNORE INTO user_roles (user_id, role_id) VALUES (?, ?)"#,
            uid,
            rid
        )
        .execute(&self.pool)
        .await
        .context("Error asignando rol al usuario")?;
        Ok(())
    }

    async fn remove_role_from_user(&self, user_id: &UserId, role_id: &RoleId) -> Result<()> {
        let uid = user_id.as_str();
        let rid = role_id.as_str();
        sqlx::query!(
            r#"DELETE FROM user_roles WHERE user_id = ? AND role_id = ?"#,
            uid,
            rid
        )
        .execute(&self.pool)
        .await
        .context("Error removiendo rol del usuario")?;
        Ok(())
    }

    async fn get_user_permissions(&self, user_id: &UserId) -> Result<Vec<Permission>> {
        let uid = user_id.as_str();
        let rows = sqlx::query_as!(
            DbPermission,
            r#"
            SELECT DISTINCT p.id, p.name, p.description
            FROM permissions p
            INNER JOIN role_permissions rp ON rp.permission_id = p.id
            INNER JOIN user_roles ur ON ur.role_id = rp.role_id
            WHERE ur.user_id = ?
            "#,
            uid
        )
        .fetch_all(&self.pool)
        .await
        .context("Error obteniendo permisos del usuario")?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Permission::from_persistence(
                    PermissionId::from_string(row.id),
                    row.name,
                    row.description,
                )
            })
            .collect())
    }

    async fn user_has_permission(&self, user_id: &UserId, permission_name: &str) -> Result<bool> {
        let uid = user_id.as_str();
        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count: i64"
            FROM permissions p
            INNER JOIN role_permissions rp ON rp.permission_id = p.id
            INNER JOIN user_roles ur ON ur.role_id = rp.role_id
            WHERE ur.user_id = ? AND p.name = ?
            "#,
            uid,
            permission_name
        )
        .fetch_one(&self.pool)
        .await
        .context("Error verificando permiso del usuario")?;

        Ok(result > 0)
    }

    async fn create_permission(&self, permission: &Permission) -> Result<()> {
        let id = permission.id().as_str();
        let name = permission.name();
        let description = permission.description();
        sqlx::query!(
            r#"INSERT INTO permissions (id, name, description) VALUES (?, ?, ?)"#,
            id,
            name,
            description
        )
        .execute(&self.pool)
        .await
        .context("Error creando permiso")?;
        Ok(())
    }

    async fn list_permissions(&self) -> Result<Vec<Permission>> {
        let rows = sqlx::query_as!(
            DbPermission,
            r#"SELECT id, name, description FROM permissions ORDER BY name ASC"#
        )
        .fetch_all(&self.pool)
        .await
        .context("Error listando permisos")?;

        Ok(rows
            .into_iter()
            .map(|row| {
                Permission::from_persistence(
                    PermissionId::from_string(row.id),
                    row.name,
                    row.description,
                )
            })
            .collect())
    }

    async fn assign_permission_to_role(
        &self,
        role_id: &RoleId,
        permission_id: &PermissionId,
    ) -> Result<()> {
        let rid = role_id.as_str();
        let pid = permission_id.as_str();
        sqlx::query!(
            r#"INSERT OR IGNORE INTO role_permissions (role_id, permission_id) VALUES (?, ?)"#,
            rid,
            pid
        )
        .execute(&self.pool)
        .await
        .context("Error asignando permiso al rol")?;
        Ok(())
    }
}
