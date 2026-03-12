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
use serde::Deserialize;

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
