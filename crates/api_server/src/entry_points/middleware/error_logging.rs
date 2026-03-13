use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tracing::{error, warn};

use crate::config::di::AppState;

pub async fn error_logging_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let uri = request.uri().to_string();
    let request_id = uuid::Uuid::new_v4().to_string();

    let response = next.run(request).await;

    let status = response.status();
    let is_5xx = status.is_server_error();
    let is_lead_endpoint = uri.contains("/landing/leads");

    if is_5xx {
        error!(
            request_id = %request_id,
            method = %method,
            uri = %uri,
            status = %status,
            "Server error occurred"
        );

        let audit_repo = state.audit_repo.clone();
        use core_logic::domain::entities::audit::{AuditLog, CreateAuditLogCommand};
        let audit_log = AuditLog::new(CreateAuditLogCommand {
            user_id: None,
            action: format!("ERROR {} {}", method, uri),
            resource: "error".to_string(),
            resource_id: Some(request_id),
            payload: Some(format!("Status: {}", status)),
            ip_address: None,
            user_agent: None,
        });

        tokio::spawn(async move {
            let _ = audit_repo.log_action(audit_log).await;
        });
    } else if is_lead_endpoint && status.is_success() {
        warn!(
            method = %method,
            uri = %uri,
            status = %status,
            "Lead endpoint accessed"
        );
    }

    response
}
