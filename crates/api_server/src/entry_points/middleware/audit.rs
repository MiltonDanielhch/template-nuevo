use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use core_logic::domain::entities::audit::{AuditLog, CreateAuditLogCommand};
use std::sync::Arc;

use crate::config::di::AppState;

pub async fn audit_middleware(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let uri = request.uri().to_string();

    // Extraer información del headers
    let ip_address = get_client_ip(&request);
    let user_agent = get_user_agent(&request);

    // Ejecutar la request
    let response = next.run(request).await;

    // Solo auditar acciones sensibles
    let action = format!("{} {}", method, uri);
    if is_sensitive_action(&action) {
        let audit_log = AuditLog::new(CreateAuditLogCommand {
            user_id: None,
            action: action.clone(),
            resource: extract_resource(&uri),
            resource_id: extract_resource_id(&uri),
            payload: Some(format!("Status: {}", response.status())),
            ip_address,
            user_agent,
        });

        // Fire and forget - no bloqueamos la respuesta
        let audit_repo = state.audit_repo.clone();
        tokio::spawn(async move {
            let _ = audit_repo.log_action(audit_log).await;
        });
    }

    response
}

fn get_client_ip(request: &Request) -> Option<String> {
    if let Some(forwarded) = request.headers().get("x-forwarded-for")
        && let Ok(forwarded_str) = forwarded.to_str()
    {
        return Some(
            forwarded_str
                .split(',')
                .next()
                .unwrap_or_default()
                .to_string(),
        );
    }

    if let Some(real_ip) = request.headers().get("x-real-ip")
        && let Ok(ip) = real_ip.to_str()
    {
        return Some(ip.to_string());
    }

    None
}

fn get_user_agent(request: &Request) -> Option<String> {
    request
        .headers()
        .get("user-agent")
        .and_then(|ua| ua.to_str().ok())
        .map(|s| s.to_string())
}

fn is_sensitive_action(action: &str) -> bool {
    let sensitive_patterns = [
        "POST /api/v1/auth/login",
        "POST /api/v1/auth/register",
        "POST /api/v1/auth/logout",
        "DELETE /api/v1/users",
        "PUT /api/v1/users",
        "POST /api/v1/roles",
        "PUT /api/v1/roles",
        "DELETE /api/v1/roles",
        "POST /api/v1/me",
        "PUT /api/v1/me",
    ];

    sensitive_patterns.iter().any(|p| action.starts_with(p))
}

fn extract_resource(uri: &str) -> String {
    let parts: Vec<&str> = uri.split('/').collect();
    parts.last().unwrap_or(&"unknown").to_string()
}

fn extract_resource_id(uri: &str) -> Option<String> {
    let parts: Vec<&str> = uri.split('/').collect();
    if parts.len() >= 5 && parts[4].len() == 36 {
        Some(parts[4].to_string())
    } else {
        None
    }
}
