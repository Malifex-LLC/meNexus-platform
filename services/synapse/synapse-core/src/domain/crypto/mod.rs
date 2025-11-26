// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoChallenge {
    pub id: Uuid,
    pub agent: String,
    pub nonce: String,
    pub created_at: OffsetDateTime,
    pub expires_at: OffsetDateTime,
}

pub struct CryptoChallengeBuilder {
    id: Option<Uuid>,
    agent: Option<String>,
    nonce: Option<String>,
    created_at: Option<OffsetDateTime>,
    expires_at: Option<OffsetDateTime>,
}

impl CryptoChallenge {
    pub fn builder() -> CryptoChallengeBuilder {
        CryptoChallengeBuilder {
            id: None,
            agent: None,
            nonce: None,
            created_at: None,
            expires_at: None,
        }
    }
    pub fn new() -> CryptoChallengeBuilder {
        Self::builder()
    }
}

impl CryptoChallengeBuilder {
    pub fn with_agent(mut self, agent: impl Into<String>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    pub fn build(self) -> CryptoChallenge {
        CryptoChallenge {
            id: Uuid::new_v4(),
            agent: self.agent.unwrap_or_default(),
            nonce: self.nonce.unwrap_or_default(),
            created_at: OffsetDateTime::now_utc(),
            expires_at: OffsetDateTime::now_utc() + Duration::from_mins(2),
        }
    }
}
