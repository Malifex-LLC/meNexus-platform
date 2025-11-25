// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, http::StatusCode, response::IntoResponse};
use synapse_core::CoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleProfilesError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("IO error: {0}")]
    Other(String),
}

impl From<synapse_core::errors::CoreError> for ModuleProfilesError {
    fn from(err: synapse_core::errors::CoreError) -> Self {
        match err {
            CoreError::Transport(_) => ModuleProfilesError::Internal("Transport error".to_string()),
            CoreError::Crypto(_) => ModuleProfilesError::Internal("Crypto error".to_string()),
            CoreError::Persistence(_) => {
                ModuleProfilesError::Internal("Persistence error".to_string())
            }
            CoreError::Config(_) => ModuleProfilesError::Internal("Config error".to_string()),
            CoreError::Validation(_) => {
                ModuleProfilesError::BadRequest("Validation error".to_string())
            }
            CoreError::Authentication(_) => {
                ModuleProfilesError::BadRequest("Authentication error".to_string())
            }
            CoreError::Authorization(_) => {
                ModuleProfilesError::Forbidden("Authorization error".to_string())
            }
            CoreError::NotFound(_) => ModuleProfilesError::NotFound("NotFound error".to_string()),
            CoreError::Conflict(_) => ModuleProfilesError::BadRequest("Conflict error".to_string()),
            CoreError::Timeout(_) => ModuleProfilesError::BadRequest("Timeout error".to_string()),
            CoreError::Unavailable(_) => {
                ModuleProfilesError::BadRequest("Unavailable error".to_string())
            }
            CoreError::RateLimited(_) => {
                ModuleProfilesError::BadRequest("RateLimited error".to_string())
            }
            CoreError::Other(_) => ModuleProfilesError::Other("Other error".to_string()),
        }
    }
}

impl IntoResponse for ModuleProfilesError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ModuleProfilesError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ModuleProfilesError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ModuleProfilesError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ModuleProfilesError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ModuleProfilesError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
