// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use synapse_core::ports::config::errors::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SynapseConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Libp2p error: {0}")]
    Libp2p(#[from] libp2p::identity::DecodingError),
    #[error("parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse url error: {0}")]
    PasreUrl(#[from] url::ParseError),
    #[error("var error: {0}")]
    Var(#[from] std::env::VarError),
    #[error("serde_json error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<SynapseConfigError> for ConfigError {
    fn from(error: SynapseConfigError) -> Self {
        match error {
            SynapseConfigError::Io(e) => ConfigError::Io(e),
            SynapseConfigError::Libp2p(e) => ConfigError::Other(e.to_string()),
            SynapseConfigError::ParseInt(e) => ConfigError::Other(e.to_string()),
            SynapseConfigError::Var(e) => ConfigError::Other(e.to_string()),
            SynapseConfigError::SerdeJson(e) => ConfigError::SerdeJson(e),
            SynapseConfigError::PasreUrl(e) => ConfigError::Url(e),
            SynapseConfigError::Other(e) => ConfigError::Other(e),
        }
    }
}
