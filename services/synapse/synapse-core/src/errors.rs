// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Entity not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("Permission denied")]
    Forbidden,

    #[error("Infrastructure error: {0}")]
    Infrastructure(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl CoreError {
    pub fn infrastructure<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Infrastructure(err.to_string())
    }
    pub fn unknown<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Unknown(err.to_string())
    }
}
