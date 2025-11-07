// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use thiserror::Error;
use url;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing {0}")]
    Missing(&'static str),

    #[error("invalid {0}: {1}")]
    Invalid(&'static str, String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Convert(#[from] std::convert::Infallible),

    #[error("other error")]
    Other(String),
}
