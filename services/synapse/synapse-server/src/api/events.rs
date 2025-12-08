// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::extract::Path;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::errors::AppError;
use crate::state::AppState;
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use synapse_core::domain::events::Event;
use synapse_core::domain::events::ObjectRef;

#[derive(Deserialize)]
struct CreateEventRequest {
    event_type: String,
    agent: String,
    module_kind: Option<String>,
    module_slug: Option<String>,
    target: Option<ObjectRef>,
    previous: Option<Uuid>,
    content: Option<String>,
    artifacts: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
    links: Option<Vec<String>>,
    data: Option<Vec<u8>>,
    expiration: Option<OffsetDateTime>,
}

#[derive(Serialize)]
pub struct LocalEventResult {
    event: Event,
}

#[derive(Serialize)]
pub struct RemoteEventResult {
    event: Vec<Event>,
}

#[derive(Deserialize)]
struct ListRemoteEventsRequest {
    event_type: String,
}

#[derive(Serialize)]
struct ListRemoteEventsResult {
    events: Vec<Event>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/events", post(create_local_event))
        .route(
            "/synapses/{synapse_public_key}/events",
            post(create_remote_event),
        )
        .route(
            "/synapses/{synapse_public_key}/events",
            get(list_remote_events),
        )
}

async fn create_local_event(
    State(_app): State<AppState>,
    Json(body): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<LocalEventResult>), AppError> {
    let cmd = CreateEventCommand {
        event_type: body.event_type,
        module_kind: body.module_kind,
        module_slug: body.module_slug,
        agent: body.agent,
        target: body.target,
        previous: body.previous,
        content: body.content,
        artifacts: body.artifacts,
        metadata: body.metadata,
        links: body.links,
        data: body.data,
        expiration: body.expiration,
        ..Default::default()
    };
    let created = _app.create_local_event.execute(cmd).await?;
    Ok((
        StatusCode::CREATED,
        Json(LocalEventResult { event: created }),
    ))
}

async fn create_remote_event(
    State(_app): State<AppState>,
    Path(synapse_public_key): Path<String>,
    Json(body): Json<CreateEventRequest>,
) -> Result<(StatusCode, Json<RemoteEventResult>), AppError> {
    let inner = CreateEventCommand {
        event_type: body.event_type,
        module_kind: body.module_kind,
        module_slug: body.module_slug,
        agent: body.agent,
        target: body.target,
        previous: body.previous,
        content: body.content,
        artifacts: body.artifacts,
        metadata: body.metadata,
        links: body.links,
        data: body.data,
        expiration: body.expiration,
        ..Default::default()
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let created = _app.create_remote_event.execute(cmd).await?;

    Ok((
        StatusCode::CREATED,
        Json(RemoteEventResult { event: created }),
    ))
}

async fn list_remote_events(
    State(_app): State<AppState>,
    Path(synapse_public_key): Path<String>,
    Json(body): Json<ListRemoteEventsRequest>,
) -> Result<(StatusCode, Json<ListRemoteEventsResult>), AppError> {
    let inner = CreateEventCommand {
        event_type: body.event_type,
        module_kind: Some("core".to_string()),
        module_slug: None,
        agent: "100".to_string(),
        target: None,
        previous: None,
        content: None,
        artifacts: None,
        metadata: None,
        links: None,
        data: None,
        expiration: None,
        ..Default::default()
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let results = _app.create_remote_event.execute(cmd).await?;

    Ok((
        StatusCode::CREATED,
        Json(ListRemoteEventsResult { events: results }),
    ))
}
