use crate::domain::entities::audit::AuditLog;
use async_trait::async_trait;

#[async_trait]
pub trait IAuditRepository: Send + Sync {
    async fn log_action(&self, audit_log: AuditLog) -> Result<(), anyhow::Error>;
    async fn get_logs_for_user(&self, user_id: &str) -> Result<Vec<AuditLog>, anyhow::Error>;
    async fn get_logs_for_resource(
        &self,
        resource: &str,
        resource_id: &str,
    ) -> Result<Vec<AuditLog>, anyhow::Error>;
}
