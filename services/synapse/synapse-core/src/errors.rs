// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::ports::config::errors::ConfigError;
use crate::ports::crypto::errors::CryptoError;
use crate::ports::federation::errors::TransportError;
use crate::ports::persistence::errors::PersistenceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error(transparent)]
    Crypto(#[from] CryptoError),

    #[error(transparent)]
    Persistence(#[from] PersistenceError),

    #[error(transparent)]
    Config(#[from] ConfigError),

    #[error("validation: {0}")]
    Validation(String),

    #[error("authentication failed: {0}")]
    Authentication(String),

    #[error("permission denied: {0}")]
    Authorization(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("timeout: {0}")]
    Timeout(String),

    #[error("service unavailable: {0}")]
    Unavailable(String),

    #[error("rate limited: {0}")]
    RateLimited(String),

    #[error("other error: {0}")]
    Other(String),
}

impl CoreError {
    pub fn transport<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Transport(TransportError::Other(err.to_string()))
    }
    pub fn crypto<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Crypto(CryptoError::Other(err.to_string()))
    }
    pub fn persistence<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Persistence(PersistenceError::Other(err.to_string()))
    }
    pub fn config<E: std::fmt::Display>(err: E) -> Self {
        CoreError::Config(ConfigError::Other(err.to_string()))
    }
}
