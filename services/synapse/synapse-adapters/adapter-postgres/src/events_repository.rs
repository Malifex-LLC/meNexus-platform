// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use sqlx::{Pool, Postgres};
use synapse_core::domain::events::Event;
use synapse_core::errors::CoreError;
use synapse_core::ports::events::EventRepository;

pub struct PostgresEventsRepository {
    pool: Pool<Postgres>,
}

impl PostgresEventsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl EventRepository for PostgresEventsRepository {
    async fn create(&self, event: Event) -> Result<Event, CoreError> {
        let row = sqlx::query!(
            r#"
            INSERT INTO events (event_id, event_type, agent_public_key)
            VALUES ($1, $2, $3)
            RETURNING event_id, event_type, agent_public_key
            "#,
            event.event_id,
            event.event_type,
            event.agent_public_key
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| CoreError::infrastructure(err))?;

        Ok(Event {
            event_id: row.event_id,
            event_type: row.event_type,
            agent_public_key: row.agent_public_key,
        })
    }
}
