// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use synapse_core::PersistenceError;
use synapse_core::domain::crypto::CryptoChallenge;
use synapse_core::ports::crypto::CryptoRepository;
use uuid::Uuid;

pub struct PostgresCryptoRepository {
    pool: Pool<Postgres>,
}

impl PostgresCryptoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CryptoRepository for PostgresCryptoRepository {
    async fn store_challenge(
        &self,
        challenge: CryptoChallenge,
    ) -> Result<CryptoChallenge, PersistenceError> {
        let row = sqlx::query!(
            r#"
        INSERT INTO challenges (id, agent, nonce, created_at, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, agent, nonce, created_at, expires_at
        "#,
            challenge.id,
            challenge.agent,
            challenge.nonce,
            challenge.created_at,
            challenge.expires_at,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;

        let stored = CryptoChallenge {
            id: row.id,
            agent: row.agent,
            nonce: row.nonce,
            created_at: row.created_at,
            expires_at: row.expires_at,
        };

        Ok(stored)
    }

    async fn get_challenge(&self, id: Uuid) -> Result<Option<CryptoChallenge>, PersistenceError> {
        let row = sqlx::query!(
            r#"
        SELECT id, agent, nonce, created_at, expires_at
        FROM challenges
        WHERE id = $1
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;

        let stored = CryptoChallenge {
            id: row.id,
            agent: row.agent,
            nonce: row.nonce,
            created_at: row.created_at,
            expires_at: row.expires_at,
        };

        Ok(Some(stored))
    }
}
