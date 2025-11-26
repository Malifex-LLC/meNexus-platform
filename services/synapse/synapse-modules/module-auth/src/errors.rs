// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, http::StatusCode, response::IntoResponse};
use synapse_core::CoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleAuthError {
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

impl From<synapse_core::errors::CoreError> for ModuleAuthError {
    fn from(err: synapse_core::errors::CoreError) -> Self {
        match err {
            CoreError::Transport(_) => ModuleAuthError::Internal("Transport error".to_string()),
            CoreError::Crypto(_) => ModuleAuthError::Internal("Crypto error".to_string()),
            CoreError::Persistence(_) => ModuleAuthError::Internal("Persistence error".to_string()),
            CoreError::Config(_) => ModuleAuthError::Internal("Config error".to_string()),
            CoreError::Validation(_) => ModuleAuthError::BadRequest("Validation error".to_string()),
            CoreError::Authentication(_) => {
                ModuleAuthError::BadRequest("Authentication error".to_string())
            }
            CoreError::Authorization(_) => {
                ModuleAuthError::Forbidden("Authorization error".to_string())
            }
            CoreError::NotFound(_) => ModuleAuthError::NotFound("NotFound error".to_string()),
            CoreError::Conflict(_) => ModuleAuthError::BadRequest("Conflict error".to_string()),
            CoreError::Timeout(_) => ModuleAuthError::BadRequest("Timeout error".to_string()),
            CoreError::Unavailable(_) => {
                ModuleAuthError::BadRequest("Unavailable error".to_string())
            }
            CoreError::RateLimited(_) => {
                ModuleAuthError::BadRequest("RateLimited error".to_string())
            }
            CoreError::Other(_) => ModuleAuthError::Other("Other error".to_string()),
        }
    }
}

impl IntoResponse for ModuleAuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ModuleAuthError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ModuleAuthError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ModuleAuthError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ModuleAuthError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ModuleAuthError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
