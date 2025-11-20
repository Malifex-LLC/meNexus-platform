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

pub struct EventBuilder {
    event_type: Option<String>,
    module_kind: Option<String>,
    module_slug: Option<String>,
    agent: Option<PublicKey>,
    target: Option<ObjectRef>,
    previous: Option<Uuid>,
    content: Option<String>,
    artifacts: Option<Vec<ArtifactUri>>,
    metadata: Option<HashMap<String, String>>,
    links: Option<Vec<String>>,
    data: Option<Vec<u8>>,
    expiration: Option<OffsetDateTime>,
}

impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder {
            event_type: None,
            module_kind: None,
            module_slug: None,
            agent: None,
            target: None,
            previous: None,
            content: None,
            artifacts: None,
            metadata: None,
            links: None,
            data: None,
            expiration: None,
        }
    }

    // Convenience alias to align with common builder naming.
    pub fn new() -> EventBuilder {
        Self::builder()
    }
}

impl EventBuilder {
    pub fn with_event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    pub fn with_module_kind(mut self, module_kind: impl Into<String>) -> Self {
        self.module_kind = Some(module_kind.into());
        self
    }

    pub fn with_module_slug(mut self, module_slug: impl Into<String>) -> Self {
        self.module_slug = Some(module_slug.into());
        self
    }

    pub fn with_agent(mut self, agent: impl Into<PublicKey>) -> Self {
        self.agent = Some(agent.into());
        self
    }

    pub fn with_target(mut self, target: ObjectRef) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_previous(mut self, previous: Uuid) -> Self {
        self.previous = Some(previous);
        self
    }

    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn with_artifacts(mut self, artifacts: Vec<ArtifactUri>) -> Self {
        self.artifacts = Some(artifacts);
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_links(mut self, links: Vec<String>) -> Self {
        self.links = Some(links);
        self
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn with_expiration(mut self, expiration: OffsetDateTime) -> Self {
        self.expiration = Some(expiration);
        self
    }

    pub fn build(self) -> Event {
        Event {
            id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            event_type: self.event_type.unwrap_or_default(),
            module_kind: self.module_kind,
            module_slug: self.module_slug,
            agent: self.agent.unwrap_or_default(),
            target: self.target,
            previous: self.previous,
            content: self.content,
            artifacts: self.artifacts,
            metadata: self.metadata,
            links: self.links,
            data: self.data,
            expiration: self.expiration,
        }
    }
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
