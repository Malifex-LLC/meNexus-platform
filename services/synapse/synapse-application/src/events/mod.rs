// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod event_service;

use std::collections::HashMap;
use synapse_core::{
    domain::events::{Event, ObjectRef},
    errors::CoreError,
};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct CreateEventCommand {
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

#[derive(Clone, Debug, Default)]
pub struct CreateRemoteEventCommand {
    pub synapse_public_key: String,
    pub event: CreateEventCommand,
}

#[async_trait::async_trait]
pub trait CreateLocalEventUseCase: Send + Sync {
    async fn execute(&self, cmd: CreateEventCommand) -> Result<Event, CoreError>;
}

#[async_trait::async_trait]
pub trait CreateRemoteEventUseCase: Send + Sync {
    async fn execute(&self, cmd: CreateRemoteEventCommand) -> Result<Vec<Event>, CoreError>;
}
