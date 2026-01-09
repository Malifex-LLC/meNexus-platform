// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use synapse_core::PersistenceError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PostgresAdapterError {
    #[error("IO error: ")]
    Io(String),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("other error: {0}")]
    Other(String),
}

impl From<PostgresAdapterError> for PersistenceError {
    fn from(error: PostgresAdapterError) -> Self {
        match error {
            PostgresAdapterError::Io(e) => PersistenceError::Io(e.to_string()),
            PostgresAdapterError::Sqlx(e) => PersistenceError::Io(e.to_string()),
            PostgresAdapterError::Migrate(e) => PersistenceError::Io(e.to_string()),
            PostgresAdapterError::Other(e) => PersistenceError::Other(e.to_string()),
        }
    }
}
