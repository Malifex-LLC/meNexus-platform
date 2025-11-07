// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error("constraint violation: {0}")]
    Constraint(String),

    #[error("not found")]
    NotFound,

    #[error("unavailable")]
    Unavailable,

    #[error("other error")]
    Other(String),
}
