// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request")]
    BadRequest,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<synapse_core::errors::CoreError> for AppError {
    fn from(err: synapse_core::errors::CoreError) -> Self {
        match err {
            synapse_core::errors::CoreError::NotFound => AppError::NotFound,
            synapse_core::errors::CoreError::Validation(_) => AppError::BadRequest,
            synapse_core::errors::CoreError::Forbidden => AppError::Forbidden,
            synapse_core::errors::CoreError::Infrastructure(_) => AppError::Internal,
            synapse_core::errors::CoreError::Unknown(_) => AppError::Internal,
        }
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
