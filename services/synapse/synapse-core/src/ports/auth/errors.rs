// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("key not found")]
    KeyNotFound,

    #[error("key decode failed")]
    KeyDecode(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("invalid signature")]
    InvalidSignature,

    #[error("nonce/replay protection failed")]
    NonceReplay,

    #[error("other error")]
    Other(String),
}
