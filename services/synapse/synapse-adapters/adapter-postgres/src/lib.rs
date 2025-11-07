// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod error;
pub mod events_repository;

use crate::error::PostgresAdapterError;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<Pool<Postgres>, PostgresAdapterError> {
    Ok(PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(database_url)
        .await?)
}

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), PostgresAdapterError> {
    Ok(sqlx::migrate!("./migrations").run(pool).await?)
}
