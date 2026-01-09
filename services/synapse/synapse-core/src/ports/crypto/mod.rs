// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{PersistenceError, domain::crypto::CryptoChallenge};

#[async_trait]
pub trait CryptoRepository: Send + Sync {
    async fn store_challenge(
        &self,
        challenge: CryptoChallenge,
    ) -> Result<CryptoChallenge, PersistenceError>;
    async fn get_challenge(&self, id: Uuid) -> Result<Option<CryptoChallenge>, PersistenceError>;
}
