// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
#[cfg(feature = "ssr")]
use synapse_core::ports::profiles::profile_repository::ProfileDiscovery;
#[cfg(feature = "ssr")]
use synapse_core::{
    CoreError, PersistenceError,
    domain::{
        events::{Event, ObjectRef},
        profiles::Profile,
    },
    ports::{
        modules::Module,
        profiles::profile_repository::{ProfilesDocStore, ProfilesRepository},
    },
};

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct ProfilesDeps {
    pub doc_store: Arc<dyn ProfilesDocStore>,
    pub profile_repo: Arc<dyn ProfilesRepository>,
    pub profile_discovery: Arc<dyn ProfileDiscovery>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
}
