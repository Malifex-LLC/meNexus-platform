// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, http::StatusCode, response::IntoResponse};
use synapse_core::CoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
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

impl From<synapse_core::errors::CoreError> for AppError {
    fn from(err: synapse_core::errors::CoreError) -> Self {
        match err {
            CoreError::Transport(_) => AppError::Internal("Transport error".to_string()),
            CoreError::Crypto(_) => AppError::Internal("Crypto error".to_string()),
            CoreError::Persistence(_) => AppError::Internal("Persistence error".to_string()),
            CoreError::Config(_) => AppError::Internal("Config error".to_string()),
            CoreError::Validation(_) => AppError::BadRequest("Validation error".to_string()),
            CoreError::Authentication(_) => {
                AppError::BadRequest("Authentication error".to_string())
            }
            CoreError::Authorization(_) => AppError::Forbidden("Authorization error".to_string()),
            CoreError::NotFound(_) => AppError::NotFound("NotFound error".to_string()),
            CoreError::Conflict(_) => AppError::BadRequest("Conflict error".to_string()),
            CoreError::Timeout(_) => AppError::BadRequest("Timeout error".to_string()),
            CoreError::Unavailable(_) => AppError::BadRequest("Unavailable error".to_string()),
            CoreError::RateLimited(_) => AppError::BadRequest("RateLimited error".to_string()),
            CoreError::Other(_) => AppError::Other("Other error".to_string()),
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
