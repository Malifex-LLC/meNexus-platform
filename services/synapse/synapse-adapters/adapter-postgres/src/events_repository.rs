// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, query};
use synapse_core::PersistenceError;
use synapse_core::domain::events::{Event, ObjectRef};
use synapse_core::ports::events::event_repository::{EventFilter, EventRepository};
use time::OffsetDateTime;
use uuid::Uuid;

pub struct PostgresEventsRepository {
    pool: Pool<Postgres>,
}

impl PostgresEventsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(Debug)]
struct EventRow {
    id: Uuid,
    created_at: OffsetDateTime,
    event_type: String,
    module_kind: Option<String>,
    module_slug: Option<String>,
    agent: String,
    agent_signature: Option<String>,
    // the alias `"target?: JsonValue"` maps to this field
    target: Option<JsonValue>,
    previous: Option<Uuid>,
    content: Option<String>,
    // `"artifacts?: Vec<String>"` maps here
    artifacts: Option<Vec<String>>,
    // `"metadata?: JsonValue"` maps here
    metadata: Option<JsonValue>,
    // `"links?: Vec<String>"` maps here
    links: Option<Vec<String>>,
    // `"data?: Vec<u8>"` maps here
    data: Option<Vec<u8>>,
    expiration: Option<OffsetDateTime>,
}

#[async_trait]
impl EventRepository for PostgresEventsRepository {
    async fn record(&self, event: Event) -> Result<Event, PersistenceError> {
        // --- Prepare SQL-friendly values ---

        // target -> Option<serde_json::Value>
        let target_json: Option<JsonValue> = event
            .target
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| PersistenceError::Other(e.to_string()))?;

        // metadata -> Option<serde_json::Value>
        let metadata_json: Option<JsonValue> = event
            .metadata
            .as_ref()
            .map(serde_json::to_value)
            .transpose()
            .map_err(|e| PersistenceError::Other(e.to_string()))?;

        // artifacts: Option<Vec<String>>
        let artifacts_arr: Option<Vec<String>> = event.artifacts.clone();

        // links: Option<Vec<String>>
        let links_arr: Option<Vec<String>> = event.links.clone();

        // data: Option<Vec<u8>>
        let data_bytes: Option<Vec<u8>> = event.data.clone();

        // --- INSERT + RETURN ---

        let row = sqlx::query!(
            r#"
            INSERT INTO events
                (id, created_at, event_type, module_kind, module_slug, agent, agent_signature,
                 target, previous, content, artifacts, metadata, links, data, expiration)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7,
                 $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING
                id,
                created_at,
                event_type,
                module_kind,
                module_slug,
                agent,
                agent_signature,
                target      as "target?: JsonValue",     -- nullable JSON -> Option<JsonValue>
                previous,
                content,
                artifacts   as "artifacts?: Vec<String>",-- nullable TEXT[] -> Option<Vec<String>>
                metadata    as "metadata?: JsonValue",    -- nullable JSON -> Option<JsonValue>
                links       as "links?: Vec<String>",     -- nullable TEXT[] -> Option<Vec<String>>
                data        as "data?: Vec<u8>",          -- nullable BYTEA  -> Option<Vec<u8>>
                expiration
            "#,
            event.id,
            event.created_at, // OffsetDateTime <-> TIMESTAMPTZ
            event.event_type,
            event.module_kind,
            event.module_slug,
            event.agent,
            event.agent_signature,
            target_json, // Option<JsonValue>
            event.previous,
            event.content,
            artifacts_arr.as_deref(), // Option<Vec<String>> -> Option<&[String]>
            metadata_json,            // Option<JsonValue>
            links_arr.as_deref(),     // Option<Vec<String>> -> Option<&[String]>
            data_bytes.as_deref(),    // Option<Vec<u8>> -> Option<&[u8]>
            event.expiration
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| PersistenceError::Other(err.to_string()))?;

        // --- Convert SQL row back into your Event types ---

        // target JSON -> Option<ObjectRef>
        let target: Option<ObjectRef> = row.target.and_then(|v| serde_json::from_value(v).ok());

        // metadata JSON -> Option<HashMap<String, String>>
        let metadata = row.metadata.and_then(|v| serde_json::from_value(v).ok());

        // links already returned as Option<Vec<String>>
        let links: Option<Vec<String>> = row.links;

        Ok(Event {
            id: row.id,
            created_at: row.created_at,
            event_type: row.event_type,
            module_kind: row.module_kind,
            module_slug: row.module_slug,
            agent: row.agent,
            agent_signature: row.agent_signature,
            target,
            previous: row.previous,
            content: row.content,
            artifacts: row.artifacts, // now Option<Vec<String>>
            metadata,
            links,          // Option<Vec<Url>>
            data: row.data, // Option<Vec<u8>>
            expiration: row.expiration,
        })
    }

    async fn retrieve(&self, filter: EventFilter) -> Result<Vec<Event>, PersistenceError> {
        let EventFilter {
            event_type,
            module_kind,
            module_slug,
        } = filter;

        let rows: Vec<EventRow> = match (
            event_type.as_deref(),
            module_kind.as_deref(),
            module_slug.as_deref(),
        ) {
            // 1) No filters: all events
            (None, None, None) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                "#
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 2) Only event_type
            (Some(event_type), None, None) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE event_type = $1
                "#,
                event_type
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 3) Only module_kind
            (None, Some(module_kind), None) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE module_kind = $1
                "#,
                module_kind
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 4) Only module_slug
            (None, None, Some(module_slug)) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE module_slug = $1
                "#,
                module_slug
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 5) event_type + module_kind
            (Some(event_type), Some(module_kind), None) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE event_type = $1
                  AND module_kind = $2
                "#,
                event_type,
                module_kind
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 6) event_type + module_slug
            (Some(event_type), None, Some(module_slug)) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE event_type = $1
                  AND module_slug = $2
                "#,
                event_type,
                module_slug
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 7) module_kind + module_slug
            (None, Some(module_kind), Some(module_slug)) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE module_kind = $1
                  AND module_slug = $2
                "#,
                module_kind,
                module_slug
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,

            // 8) event_type + module_kind + module_slug
            (Some(event_type), Some(module_kind), Some(module_slug)) => sqlx::query_as!(
                EventRow,
                r#"
                SELECT
                    id,
                    created_at,
                    event_type,
                    module_kind,
                    module_slug,
                    agent,
                    agent_signature,
                    target      as "target?: JsonValue",
                    previous,
                    content,
                    artifacts   as "artifacts?: Vec<String>",
                    metadata    as "metadata?: JsonValue",
                    links       as "links?: Vec<String>",
                    data        as "data?: Vec<u8>",
                    expiration
                FROM events
                WHERE event_type = $1
                  AND module_kind = $2
                  AND module_slug = $3
                "#,
                event_type,
                module_kind,
                module_slug
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|err| PersistenceError::Other(err.to_string()))?,
        };

        // Map EventRow -> Event
        let events = rows
            .into_iter()
            .map(|row| {
                let target: Option<ObjectRef> = row
                    .target
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(|err| PersistenceError::Other(err.to_string()))?;

                let metadata = row
                    .metadata
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(|err| PersistenceError::Other(err.to_string()))?;

                Ok(Event {
                    id: row.id,
                    created_at: row.created_at,
                    event_type: row.event_type,
                    module_kind: row.module_kind,
                    module_slug: row.module_slug,
                    agent: row.agent,
                    agent_signature: row.agent_signature,
                    target,
                    previous: row.previous,
                    content: row.content,
                    artifacts: row.artifacts,
                    metadata,
                    links: row.links,
                    data: row.data,
                    expiration: row.expiration,
                })
            })
            .collect::<Result<Vec<_>, PersistenceError>>()?;

        Ok(events)
    }
}
