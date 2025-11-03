// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod events_repository;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use synapse_core::errors::CoreError;

fn map_sqlx_err(err: sqlx::Error) -> CoreError {
    match err {
        sqlx::Error::RowNotFound => CoreError::NotFound,
        _ => CoreError::infrastructure(err),
    }
}

fn map_migrate_err(err: sqlx::migrate::MigrateError) -> CoreError {
    CoreError::infrastructure(err)
}

pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, CoreError> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await
        .map_err(map_sqlx_err)
}

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), CoreError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(map_migrate_err)
}
