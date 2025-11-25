// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::PersistenceError;
use crate::domain::profiles::Profile;
use async_trait::async_trait;

#[async_trait::async_trait]
pub trait ProfilesRepository: Send + Sync {
    async fn get_profile(&self, public_key: &str) -> Result<Option<Profile>, PersistenceError>;
    async fn upsert_profile(&self, profile: &Profile) -> Result<Profile, PersistenceError>;
    async fn delete_profile(&self, public_key: &str) -> Result<(), PersistenceError>;
    // async fn search_profiles(
    //     &self,
    //     query: &str,
    //     limit: u32,
    //     offset: u32,
    // ) -> Result<Vec<Profile>, CoreError>;
}

#[async_trait]
pub trait ProfilesDocStore: Send + Sync {
    async fn get_doc(&self, public_key: &str) -> Result<Option<Vec<u8>>, PersistenceError>;
    async fn upsert_doc(&self, public_key: &str, doc_bytes: &[u8]) -> Result<(), PersistenceError>;
    async fn delete_doc(&self, public_key: &str) -> Result<(), PersistenceError>;
}
