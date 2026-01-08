// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use module_profiles::http::ProfileDoc;
use sqlx::{Pool, Postgres, postgres::PgRow, query};
use synapse_core::PersistenceError;
use synapse_core::domain::auth::Session;
use synapse_core::domain::crypto::CryptoChallenge;
use synapse_core::domain::profiles::Profile;
use synapse_core::ports::auth::SessionRepository;
use synapse_core::ports::profiles::profile_repository::{ProfilesDocStore, ProfilesRepository};
use uuid::Uuid;

pub struct PostgresAuthRepository {
    pool: Pool<Postgres>,
}

impl PostgresAuthRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresAuthRepository {
    async fn store_session(&self, session: Session) -> Result<Session, PersistenceError> {
        let row = sqlx::query!(
            r#"
        INSERT INTO sessions (id, agent, created_at, expires_at, revoked)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, agent, created_at, expires_at, revoked
        "#,
            session.id,
            session.agent,
            session.created_at,
            session.expires_at,
            session.revoked
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(Session {
            id: row.id,
            agent: row.agent,
            created_at: row.created_at,
            expires_at: row.expires_at,
            revoked: row.revoked,
        })
    }
    async fn get_session(&self, id: Uuid) -> Result<Session, PersistenceError> {
        let row = sqlx::query!(
            r#"
        SELECT id, agent, created_at, expires_at, revoked
        FROM sessions
        WHERE id = $1

        "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(Session {
            id: row.id,
            agent: row.agent,
            created_at: row.created_at,
            expires_at: row.expires_at,
            revoked: row.revoked,
        })
    }
    async fn revoke_session(&self, id: Uuid) -> Result<Session, PersistenceError> {
        let row = sqlx::query!(
            r#"
            UPDATE sessions
            SET revoked = TRUE
            WHERE id = $1
            RETURNING id, agent, created_at, expires_at, revoked
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(Session {
            id: row.id,
            agent: row.agent,
            created_at: row.created_at,
            expires_at: row.expires_at,
            revoked: row.revoked,
        })
    }
    async fn delete_session(&self, id: Uuid) -> Result<(), PersistenceError> {
        let row = sqlx::query!(
            r#"
        DELETE FROM sessions 
        WHERE id = $1
        "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(())
    }
}
