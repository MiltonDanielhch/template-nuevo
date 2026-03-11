use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    pub payload: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateAuditLogCommand {
    pub user_id: Option<String>,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    pub payload: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuditLog {
    pub fn new(command: CreateAuditLogCommand) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: command.user_id,
            action: command.action,
            resource: command.resource,
            resource_id: command.resource_id,
            payload: command.payload,
            ip_address: command.ip_address,
            user_agent: command.user_agent,
            created_at: Utc::now(),
        }
    }
}
