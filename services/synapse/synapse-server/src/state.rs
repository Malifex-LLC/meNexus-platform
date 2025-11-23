// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use dashmap::DashMap;
use std::sync::Arc;
use synapse_application::events::CreateLocalEventUseCase;
use synapse_application::events::CreateRemoteEventUseCase;
use synapse_core::ports::events::event_repository::EventRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn EventRepository>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
    pub known_peers: Arc<DashMap<String, String>>,
}
