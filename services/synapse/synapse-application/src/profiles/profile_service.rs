// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;

use async_trait::async_trait;
use synapse_core::{
    PersistenceError, domain::profiles::Profile,
    ports::profiles::profile_repository::ProfilesRepository,
};

pub struct ProfileService {
    repo: Arc<dyn ProfilesRepository>,
}

#[async_trait]
impl ProfilesRepository for ProfileService {
    async fn get_profile(&self, public_key: &str) -> Result<Option<Profile>, PersistenceError> {
        self.repo.get_profile(public_key).await
    }
    async fn upsert_profile(&self, profile: &Profile) -> Result<Profile, PersistenceError> {
        let updated = self.repo.upsert_profile(profile).await?;
        Ok(updated)
    }
    async fn delete_profile(&self, public_key: &str) -> Result<(), PersistenceError> {
        self.repo.delete_profile(public_key).await
    }
}
