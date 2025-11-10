// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: Uuid,
    pub created_at: OffsetDateTime,
    pub event_type: String,
    pub module_kind: Option<String>,
    pub module_slug: Option<String>,
    pub agent: PublicKey,
    pub target: Option<ObjectRef>,
    pub previous: Option<Uuid>,
    pub content: Option<String>,
    pub artifacts: Option<Vec<ArtifactUri>>,
    pub metadata: Option<HashMap<String, String>>,
    pub links: Option<Vec<String>>,
    pub data: Option<Vec<u8>>,
    pub expiration: Option<OffsetDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectRef {
    Synapse(Uuid),
    Agent(PublicKey),
    Event(Uuid),
    Artifact(String), // URI/CID
    External(String), // URL/ID
    Custom { kind: String, id: String },
}

pub type PublicKey = String;
pub type ArtifactUri = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    Moderator,
    Member,
    Guest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynapseSettings {
    pub privacy: PrivacyLevel,
    pub allowed_modules: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PrivacyLevel {
    Public,
    Private,
    InviteOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaRef {
    pub cid: String,
    pub media_type: MediaType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelConfig {
    pub name: Option<String>,
    pub description: Option<String>,
    pub privacy: PrivacyLevel,
    pub participants: Vec<PublicKey>,
    pub encryption: Option<EncryptionConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptionConfig {
    pub protocol: String, // e.g., "double_ratchet"
    pub key_material: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CoreEvent {
    CreateContent {
        content: String,
        media: Vec<MediaRef>,
        parent_post: Option<Uuid>,
    },
    Custom {
        module_kind: String,
        module_slug: String,
        action: String,
        payload: Vec<u8>,
    },
}
