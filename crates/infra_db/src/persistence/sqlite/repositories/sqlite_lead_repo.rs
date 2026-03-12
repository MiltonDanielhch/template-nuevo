// crates/infra_db/src/persistence/sqlite/repositories/sqlite_lead_repo.rs
//! # Adaptador SqliteLeadRepository
//!
//! Implementa el puerto `ILeadRepository` utilizando SQLite.

use anyhow::Result;
use async_trait::async_trait;
use sqlx::SqlitePool;

use core_logic::domain::{entities::lead::Lead, interfaces::ILeadRepository};

use crate::persistence::sqlite::models::DbLead;

/// Repositorio de leads en SQLite.
#[derive(Clone)]
pub struct SqliteLeadRepository {
    pool: SqlitePool,
}

impl SqliteLeadRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ILeadRepository for SqliteLeadRepository {
    async fn save(&self, lead: &Lead) -> Result<()> {
        let db_lead = DbLead {
            id: lead.id().to_string(),
            email: lead.email().to_string(),
            name: lead.name().clone(),
            source: None,
            created_at: lead.created_at().naive_utc(),
            updated_at: lead.updated_at().naive_utc(),
        };

        sqlx::query(
            r#"
            INSERT INTO leads (id, email, name, source, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                email = EXCLUDED.email,
                name = EXCLUDED.name,
                source = EXCLUDED.source,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(&db_lead.id)
        .bind(&db_lead.email)
        .bind(&db_lead.name)
        .bind(&db_lead.source)
        .bind(db_lead.created_at)
        .bind(db_lead.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
