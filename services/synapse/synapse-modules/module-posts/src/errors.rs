// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

#[cfg(feature = "ssr")]
use axum::{Json, http::StatusCode, response::IntoResponse};
#[cfg(feature = "ssr")]
use synapse_core::CoreError;
#[cfg(feature = "ssr")]
use thiserror::Error;

#[cfg(feature = "ssr")]
#[derive(Error, Debug)]
pub enum ModulePostsError {
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

#[cfg(feature = "ssr")]
impl From<synapse_core::errors::CoreError> for ModulePostsError {
    fn from(err: synapse_core::errors::CoreError) -> Self {
        match err {
            CoreError::Transport(_) => ModulePostsError::Internal("Transport error".to_string()),
            CoreError::Crypto(_) => ModulePostsError::Internal("Crypto error".to_string()),
            CoreError::Persistence(_) => {
                ModulePostsError::Internal("Persistence error".to_string())
            }
            CoreError::Config(_) => ModulePostsError::Internal("Config error".to_string()),
            CoreError::Validation(_) => {
                ModulePostsError::BadRequest("Validation error".to_string())
            }
            CoreError::Authentication(_) => {
                ModulePostsError::BadRequest("Authentication error".to_string())
            }
            CoreError::Authorization(_) => {
                ModulePostsError::Forbidden("Authorization error".to_string())
            }
            CoreError::NotFound(_) => ModulePostsError::NotFound("NotFound error".to_string()),
            CoreError::Conflict(_) => ModulePostsError::BadRequest("Conflict error".to_string()),
            CoreError::Timeout(_) => ModulePostsError::BadRequest("Timeout error".to_string()),
            CoreError::Unavailable(_) => {
                ModulePostsError::BadRequest("Unavailable error".to_string())
            }
            CoreError::RateLimited(_) => {
                ModulePostsError::BadRequest("RateLimited error".to_string())
            }
            CoreError::Other(_) => ModulePostsError::Other("Other error".to_string()),
        }
    }
}

#[cfg(feature = "ssr")]
impl IntoResponse for ModulePostsError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ModulePostsError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ModulePostsError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            ModulePostsError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ModulePostsError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ModulePostsError::Other(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
