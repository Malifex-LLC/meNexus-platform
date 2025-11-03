// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::state::AppState;
use synapse_application::events::CreateEventCommand;
use synapse_application::events::CreateEventUseCase;
use synapse_core::domain::events::Event;
use tracing::debug;

#[derive(Deserialize)]
struct CreateEventRequest {
    event_type: String,
    agent_public_key: String,
}

#[derive(Serialize)]
pub struct EventResult {
    event: Event,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/events", post(create_event))
}

async fn create_event(
    State(_app): State<AppState>,
    Json(body): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<EventResult>), AppError> {
    let cmd = CreateEventCommand {
        event_type: body.event_type,
        agent_public_key: body.agent_public_key,
    };
    let created = _app.create_event.execute(cmd).await?;
    Ok((StatusCode::CREATED, Json(EventResult { event: created })))
}
