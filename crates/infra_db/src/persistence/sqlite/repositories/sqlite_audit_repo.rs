use async_trait::async_trait;
use core_logic::domain::entities::audit::AuditLog;
use core_logic::domain::interfaces::audit_repo::IAuditRepository;
use sqlx::{Row, SqlitePool};

pub struct SqliteAuditRepository {
    pool: SqlitePool,
}

impl SqliteAuditRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IAuditRepository for SqliteAuditRepository {
    async fn log_action(&self, audit_log: AuditLog) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (id, user_id, action, resource, resource_id, payload, ip_address, user_agent, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(audit_log.id.to_string())
        .bind(audit_log.user_id)
        .bind(audit_log.action)
        .bind(audit_log.resource)
        .bind(audit_log.resource_id)
        .bind(audit_log.payload)
        .bind(audit_log.ip_address)
        .bind(audit_log.user_agent)
        .bind(audit_log.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_logs_for_user(&self, user_id: &str) -> Result<Vec<AuditLog>, anyhow::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, action, resource, resource_id, payload, ip_address, user_agent, created_at
            FROM audit_logs
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let logs = rows
            .into_iter()
            .map(|row| {
                let id: String = row.get(0);
                let user_id: Option<String> = row.get(1);
                let action: String = row.get(2);
                let resource: String = row.get(3);
                let resource_id: Option<String> = row.get(4);
                let payload: Option<String> = row.get(5);
                let ip_address: Option<String> = row.get(6);
                let user_agent: Option<String> = row.get(7);
                let created_at_str: String = row.get(8);

                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                AuditLog {
                    id: uuid::Uuid::parse_str(&id).unwrap_or_else(|_| uuid::Uuid::nil()),
                    user_id,
                    action,
                    resource,
                    resource_id,
                    payload,
                    ip_address,
                    user_agent,
                    created_at,
                }
            })
            .collect();

        Ok(logs)
    }

    async fn get_logs_for_resource(
        &self,
        resource: &str,
        resource_id: &str,
    ) -> Result<Vec<AuditLog>, anyhow::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, action, resource, resource_id, payload, ip_address, user_agent, created_at
            FROM audit_logs
            WHERE resource = ? AND resource_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(resource)
        .bind(resource_id)
        .fetch_all(&self.pool)
        .await?;

        let logs = rows
            .into_iter()
            .map(|row| {
                let id: String = row.get(0);
                let user_id: Option<String> = row.get(1);
                let action: String = row.get(2);
                let resource: String = row.get(3);
                let resource_id: Option<String> = row.get(4);
                let payload: Option<String> = row.get(5);
                let ip_address: Option<String> = row.get(6);
                let user_agent: Option<String> = row.get(7);
                let created_at_str: String = row.get(8);

                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                AuditLog {
                    id: uuid::Uuid::parse_str(&id).unwrap_or_else(|_| uuid::Uuid::nil()),
                    user_id,
                    action,
                    resource,
                    resource_id,
                    payload,
                    ip_address,
                    user_agent,
                    created_at,
                }
            })
            .collect();

        Ok(logs)
    }
}
