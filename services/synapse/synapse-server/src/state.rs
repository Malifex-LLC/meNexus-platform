// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use dashmap::DashMap;
use std::sync::Arc;
use synapse_application::events::CreateLocalEventUseCase;
use synapse_application::events::CreateRemoteEventUseCase;
use synapse_application::profiles::profile_service::ProfileDiscoveryTransport;
use synapse_core::ports::auth::SessionRepository;
use synapse_core::ports::crypto::CryptoRepository;
use synapse_core::ports::events::event_repository::EventRepository;
use synapse_core::ports::profiles::profile_repository::ProfilesDocStore;
use synapse_core::ports::profiles::profile_repository::ProfilesRepository;

use synapse_core::ports::profiles::profile_repository::ProfileDiscovery;

#[derive(Clone)]
pub struct AppState {
    pub event_repo: Arc<dyn EventRepository>,
    pub crypto_repo: Arc<dyn CryptoRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub profile_doc_store: Arc<dyn ProfilesDocStore>,
    pub profile_repo: Arc<dyn ProfilesRepository>,
    pub profile_discovery: Arc<dyn ProfileDiscovery>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
    pub known_peers: Arc<DashMap<String, String>>,
}
