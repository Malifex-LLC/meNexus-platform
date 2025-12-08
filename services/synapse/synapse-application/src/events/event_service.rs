// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use async_trait::async_trait;
use std::sync::Arc;
use synapse_core::CoreError;
use synapse_core::domain::events::Event;
use synapse_core::ports::events::event_repository::EventRepository;
use synapse_core::ports::federation::FederationTransport;
use synapse_core::ports::federation::MessageHandler;
use synapse_core::ports::modules::ModuleRegistry;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct LocalEventService<R: EventRepository, T: ModuleRegistry> {
    ingest: Arc<EventIngestService<R, T>>,
}

impl<R: EventRepository, T: ModuleRegistry> LocalEventService<R, T> {
    pub fn new(ingest: Arc<EventIngestService<R, T>>) -> Self {
        Self { ingest }
    }
}

#[async_trait]
impl<R: EventRepository + Send + Sync, T: ModuleRegistry + Send + Sync> CreateLocalEventUseCase
    for LocalEventService<R, T>
{
    async fn execute(&self, cmd: CreateEventCommand) -> Result<Event, CoreError> {
        if cmd.event_type.trim().is_empty() {
            return Err(CoreError::persistence(
                "event_type must not be empty".to_string(),
            ));
        }
        if cmd.agent.trim().is_empty() {
            return Err(CoreError::persistence(
                "agent_public_key must not be empty".to_string(),
            ));
        }

        let event = Event {
            id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            event_type: cmd.event_type,
            module_kind: cmd.module_kind,
            module_slug: cmd.module_slug,
            agent: cmd.agent,
            target: cmd.target,
            previous: cmd.previous,
            content: cmd.content,
            artifacts: cmd.artifacts,
            metadata: cmd.metadata,
            links: cmd.links,
            data: cmd.data,
            expiration: cmd.expiration,
        };

        Ok(self.ingest.ingest(event).await?)
    }
}

pub struct EventIngestService<R: EventRepository, T: ModuleRegistry> {
    registry: Arc<T>,
    repo: Arc<R>,
}

impl<R: EventRepository, T: ModuleRegistry> EventIngestService<R, T> {
    pub fn new(repo: Arc<R>, registry: Arc<T>) -> Self {
        Self { registry, repo }
    }

    pub async fn ingest(&self, event: Event) -> Result<Event, CoreError> {
        let stored = self.repo.record(event).await.map_err(CoreError::from)?;
        Ok(stored)
    }
}

#[async_trait]
impl<R: EventRepository + Send + Sync, T: ModuleRegistry + Send + Sync> MessageHandler
    for EventIngestService<R, T>
{
    async fn handle_message(&self, event: Event) -> Result<Vec<Event>, CoreError> {
        let replies = match event.module_kind.as_deref() {
            Some(kind) => {
                let module = self.registry.get(kind).unwrap();
                module.handle_event(&event).await?
            }
            None => {
                return Err(CoreError::Validation(
                    "unable to validate module_kind".to_string(),
                ));
            }
        };

        self.ingest(event).await?;
        Ok(replies)
    }
}

pub struct RemoteEventService<T: FederationTransport> {
    transport: Arc<T>,
}

impl<T: FederationTransport> RemoteEventService<T> {
    pub fn new(transport: Arc<T>) -> Self {
        Self { transport }
    }
}

#[async_trait]
impl<T: FederationTransport + Send + Sync> CreateRemoteEventUseCase for RemoteEventService<T> {
    async fn execute(&self, cmd: CreateRemoteEventCommand) -> Result<Vec<Event>, CoreError> {
        if cmd.event.event_type.trim().is_empty() {
            return Err(CoreError::transport(
                "event_type must not be empty".to_string(),
            ));
        }
        if cmd.synapse_public_key.trim().is_empty() {
            return Err(CoreError::transport(
                "agent_public_key must not be empty".to_string(),
            ));
        }
        if cmd.event.agent.trim().is_empty() {
            return Err(CoreError::transport(
                "agent_public_key must not be empty".to_string(),
            ));
        }

        let event = Event {
            id: Uuid::new_v4(),
            created_at: OffsetDateTime::now_utc(),
            event_type: cmd.event.event_type,
            module_kind: cmd.event.module_kind,
            module_slug: cmd.event.module_slug,
            agent: cmd.event.agent,
            target: cmd.event.target,
            previous: cmd.event.previous,
            content: cmd.event.content,
            artifacts: cmd.event.artifacts,
            metadata: cmd.event.metadata,
            links: cmd.event.links,
            data: cmd.event.data,
            expiration: cmd.event.expiration,
        };

        Ok(self
            .transport
            .send_message(cmd.synapse_public_key, event)
            .await?)
    }
}
