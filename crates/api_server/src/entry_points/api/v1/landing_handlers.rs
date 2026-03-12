// crates/api_server/src/entry_points/api/v1/landing_handlers.rs
//! Handlers para la landing page (captura de leads).
//!
//! Permite la recepción de leads desde el frontend y su persistencia en la base de datos.

use crate::{config::di::AppState, entry_points::api::v1::errors::ApiError};
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Html,
};
use core_logic::application::use_cases::lead::create::CreateLeadCommand;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

static LEAD_RATE_LIMIT: Lazy<Mutex<HashMap<String, (Instant, u32)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Deserialize)]
pub struct CreateLeadRequest {
    pub email: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub honeypot: Option<String>,
}

pub async fn create_lead_handler(
    State(state): State<AppState>,
    Form(payload): Form<CreateLeadRequest>,
) -> Result<(StatusCode, Html<String>), ApiError> {
    // Anti-spam honeypot: if populated, treat as bot submission and ignore.
    if payload
        .honeypot
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .is_some()
    {
        let html = r#"<div class='rounded-2xl border border-green-200 bg-green-50 p-6 text-center'>
    <h3 class='text-lg font-semibold text-green-800'>¡Gracias!</h3>
    <p class='mt-2 text-sm text-green-700'>Te avisaremos por correo cuando lancemos.</p>
</div>"#;
        return Ok((StatusCode::CREATED, Html(html.to_string())));
    }

    // Rate limit by email: max 5 submissions per hour.
    let email_key = payload.email.trim().to_lowercase();
    {
        let mut rate = LEAD_RATE_LIMIT.lock().await;
        let now = Instant::now();
        let entry = rate.entry(email_key.clone()).or_insert((now, 0));
        if now.duration_since(entry.0) > Duration::from_secs(60 * 60) {
            *entry = (now, 0);
        }
        if entry.1 >= 5 {
            let html = r#"<div class='rounded-2xl border border-rose-200 bg-rose-50 p-6 text-center'>
        <h3 class='text-lg font-semibold text-rose-800'>Demasiados intentos</h3>
        <p class='mt-2 text-sm text-rose-700'>Intenta de nuevo en 1 hora o contáctanos para asistencia.</p>
    </div>"#;
            return Ok((StatusCode::TOO_MANY_REQUESTS, Html(html.to_string())));
        }
        entry.1 += 1;
    }

    let command = CreateLeadCommand {
        email: payload.email,
        name: payload.name,
    };

    let _lead = state.create_lead.execute(command).await?;

    // Devolver fragmento HTML para que HTMX lo reemplace.
    let html = r#"<div class='rounded-2xl border border-green-200 bg-green-50 p-6 text-center'>
        <h3 class='text-lg font-semibold text-green-800'>¡Gracias!</h3>
        <p class='mt-2 text-sm text-green-700'>Te avisaremos por correo cuando lancemos.</p>
    </div>"#;

    Ok((StatusCode::CREATED, Html(html.to_string())))
}
