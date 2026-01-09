// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Io error: {0}")]
    Io(String),
    #[error("invalid address: {0}")]
    InvalidAddress(String),
    #[error("protocol violation: {0}")]
    Protocol(String),
    #[error("handshake failed: {0}")]
    Handshake(String),
    #[error("unreachable peer")]
    Unreachable,
    #[error("other error: {0}")]
    Other(String),
}
