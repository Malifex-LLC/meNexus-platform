// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;

use adapter_libp2p::transport::Libp2pTransport;
use async_trait::async_trait;
use synapse_core::{
    PersistenceError, TransportError,
    domain::profiles::Profile,
    ports::profiles::profile_repository::{ProfileDiscovery, ProfilesRepository},
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

pub struct ProfileDiscoveryTransport {
    transport: Arc<Libp2pTransport>,
}

impl ProfileDiscoveryTransport {
    pub fn new(transport: Arc<Libp2pTransport>) -> Self {
        Self { transport }
    }
}

#[async_trait]
impl ProfileDiscovery for ProfileDiscoveryTransport {
    async fn announce(&self, profile_pk: &str) -> Result<(), TransportError> {
        self.transport.announce_profile(profile_pk).await?;
        Ok(())
    }

    async fn providers(&self, profile_pk: &str) -> Result<Vec<String>, TransportError> {
        let peers = self.transport.lookup_profile_providers(profile_pk).await?;
        Ok(peers)
    }
}
