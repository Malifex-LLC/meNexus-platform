// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::errors::AppError;
use crate::state::AppState;

use dashmap::DashMap;

#[derive(Serialize)]
pub struct FederationResult {
    known_peers: Arc<DashMap<String, String>>,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/peers", get(list_peers))
}

async fn list_peers(State(_app): State<AppState>) -> Result<Json<FederationResult>, AppError> {
    Ok(Json(FederationResult {
        known_peers: _app.known_peers,
    }))
}
