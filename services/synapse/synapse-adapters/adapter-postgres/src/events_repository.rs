use async_trait::async_trait;
use serde_json::Value as JsonValue;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, query};
use synapse_core::PersistenceError;
use synapse_core::domain::events::{Event, ObjectRef};
use synapse_core::ports::events::event_repository::EventRepository;

pub struct PostgresEventsRepository {
    pool: Pool<Postgres>,
}

impl PostgresEventsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
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
                (id, created_at, event_type, module_kind, module_slug, agent,
                 target, previous, content, artifacts, metadata, links, data, expiration)
            VALUES
                ($1, $2, $3, $4, $5, $6,
                 $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING
                id,
                created_at,
                event_type,
                module_kind,
                module_slug,
                agent,
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

    async fn retrieve(&self, event_type: String) -> Result<Option<Vec<Event>>, PersistenceError> {
        match event_type.as_str() {
            "all" => {
                let rows = sqlx::query!(
                    r#"
                        SELECT
                        id,
                        created_at,
                        event_type,
                        module_kind,
                        module_slug,
                        agent,
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
                .map_err(|e| PersistenceError::Other(e.to_string()))?;

                let events = rows
                    .into_iter()
                    .map(|row| {
                        let target: Option<ObjectRef> = row
                            .target
                            .map(serde_json::from_value)
                            .transpose()
                            .map_err(|e| PersistenceError::Other(e.to_string()))?;

                        let metadata = row
                            .metadata
                            .map(serde_json::from_value)
                            .transpose()
                            .map_err(|e| PersistenceError::Other(e.to_string()))?;

                        Ok(Event {
                            id: row.id,
                            created_at: row.created_at,
                            event_type: row.event_type,
                            module_kind: row.module_kind,
                            module_slug: row.module_slug,
                            agent: row.agent,
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

                Ok(Some(events))
            }
            "posts:create_post" => {
                let rows = sqlx::query!(
                    r#"
                            SELECT
                            id,
                            created_at,
                            event_type,
                            module_kind,
                            module_slug,
                            agent,
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
                    "posts:create_post"
                )
                .fetch_all(&self.pool)
                .await
                .map_err(|e| PersistenceError::Other(e.to_string()))?;

                let events = rows
                    .into_iter()
                    .map(|row| {
                        let target: Option<ObjectRef> = row
                            .target
                            .map(serde_json::from_value)
                            .transpose()
                            .map_err(|e| PersistenceError::Other(e.to_string()))?;

                        let metadata = row
                            .metadata
                            .map(serde_json::from_value)
                            .transpose()
                            .map_err(|e| PersistenceError::Other(e.to_string()))?;

                        Ok(Event {
                            id: row.id,
                            created_at: row.created_at,
                            event_type: row.event_type,
                            module_kind: row.module_kind,
                            module_slug: row.module_slug,
                            agent: row.agent,
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

                Ok(Some(events))
            }
            _ => Ok(None),
        }
    }
}
