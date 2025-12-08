// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "ssr")]
use synapse_application::events::{CreateLocalEventUseCase, CreateRemoteEventUseCase};
use synapse_core::domain::events::Event;
use synapse_core::domain::events::ObjectRef;
use synapse_core::ports::profiles::profile_repository::{
    ProfileDiscovery, ProfilesDocStore, ProfilesRepository,
};
use time::OffsetDateTime;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct PostsDeps {
    pub repo: std::sync::Arc<dyn synapse_core::ports::events::event_repository::EventRepository>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
    pub doc_store: Arc<dyn ProfilesDocStore>,
    pub profile_repo: Arc<dyn ProfilesRepository>,
    pub profile_discovery: Arc<dyn ProfileDiscovery>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: String,
    pub author_public_key: String,
    pub author_name: String,
    pub author_handle: String,
    pub author_avatar: String,
    pub timestamp: String,
    pub content: String,
    pub posted_in: String,
    pub likes: u32,
    pub comments: u32,
    pub liked: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListPostsRequest {
    pub event_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListPostsResult {
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListPostsForChannelRequest {
    pub event_type: String,
    pub channel: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListPostsForChannelResult {
    pub posts: Vec<Post>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListRemotePostsRequest {
    pub event_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ListRemotePostsResult {
    pub events: Vec<Event>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreatePostRequest {
    pub event_type: String,
    pub agent: String,
    pub module_kind: Option<String>,
    pub module_slug: Option<String>,
    pub target: Option<ObjectRef>,
    pub previous: Option<Uuid>,
    pub content: Option<String>,
    pub artifacts: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub links: Option<Vec<String>>,
    pub data: Option<Vec<u8>>,
    pub expiration: Option<OffsetDateTime>,
}
