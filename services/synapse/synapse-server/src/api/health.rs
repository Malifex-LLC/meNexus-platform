// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

use crate::errors::AppError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResult {
    status: String,
}
pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(get_health))
}

async fn get_health(State(_app): State<AppState>) -> Result<Json<HealthResult>, AppError> {
    Ok(Json(HealthResult {
        status: "Axum is running!".to_string(),
    }))
}
