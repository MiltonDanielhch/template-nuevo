use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

const MAX_REQUESTS: usize = 100;
const WINDOW_SECS: u64 = 60;

#[derive(Default)]
pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn check_rate_limit(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let window_start = now - Duration::from_secs(WINDOW_SECS);

        let entry = self.requests.entry(ip.to_string()).or_default();

        entry.retain(|&time| time > window_start);

        if entry.len() >= MAX_REQUESTS {
            return false;
        }

        entry.push(now);
        true
    }
}

pub async fn rate_limit_middleware(
    State(state): State<Arc<RwLock<RateLimiter>>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let uri = request.uri().to_string();

    // Skip rate limiting for health checks and static files
    if uri == "/health" || uri.starts_with("/_astro") || uri.starts_with("/favicon") {
        return next.run(request).await;
    }

    let ip = get_client_ip(&request).unwrap_or_else(|| "unknown".to_string());

    {
        let mut limiter = state.write().await;
        if !limiter.check_rate_limit(&ip) {
            return Response::builder()
                .status(429)
                .header("Content-Type", "application/json")
                .header("Retry-After", WINDOW_SECS.to_string())
                .body(Body::from(
                    r#"{"error":"Too many requests. Please try again later."}"#,
                ))
                .unwrap();
        }
    }

    next.run(request).await
}

fn get_client_ip(request: &Request<Body>) -> Option<String> {
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

    request
        .extensions()
        .get::<axum::extract::connect_info::ConnectInfo<std::net::SocketAddr>>()
        .map(|ci| ci.0.ip().to_string())
}
