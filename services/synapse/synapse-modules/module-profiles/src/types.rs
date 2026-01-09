// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use synapse_core::ports::profiles::profile_repository::ProfileDiscovery;
use synapse_core::{
    domain::profiles::Profile,
    ports::profiles::profile_repository::{ProfilesDocStore, ProfilesRepository},
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

#[derive(Deserialize)]
pub struct ProfileRegisterRequest {
    pub public_key: String,
    pub display_name: String,
    pub handle: String,
}

#[derive(Deserialize)]
pub struct ProfileFetchRequest {
    pub public_key: String,
}

#[derive(Serialize)]
pub struct ProfileFetchResult {
    pub profile: Option<Profile>,
}

#[derive(Deserialize)]
pub struct SetProfileRequest {
    pub handle: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub avatar_url: Option<String>,
}
