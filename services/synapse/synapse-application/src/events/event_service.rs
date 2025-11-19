// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use async_trait::async_trait;
use std::sync::Arc;
use synapse_config::SynapseConfig;
use synapse_config::get_synapse_config;
use synapse_core::CoreError;
use synapse_core::domain::events::Event;
use synapse_core::ports::events::event_repository::EventRepository;
use synapse_core::ports::federation::FederationTransport;
use synapse_core::ports::federation::MessageHandler;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct LocalEventService<R: EventRepository> {
    ingest: Arc<EventIngestService<R>>,
}

impl<R: EventRepository> LocalEventService<R> {
    pub fn new(ingest: Arc<EventIngestService<R>>) -> Self {
        Self { ingest }
    }
}

#[async_trait]
impl<R: EventRepository + Send + Sync> CreateLocalEventUseCase for LocalEventService<R> {
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

pub struct EventIngestService<R: EventRepository> {
    repo: Arc<R>,
}

impl<R: EventRepository> EventIngestService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn ingest(&self, event: Event) -> Result<Event, CoreError> {
        let stored = self.repo.record(event).await.map_err(CoreError::from)?;
        Ok(stored)
    }
}

#[async_trait]
impl<R: EventRepository + Send + Sync> MessageHandler for EventIngestService<R> {
    async fn handle_message(&self, event: Event) -> Result<Option<Vec<Event>>, CoreError> {
        match event.event_type.as_str() {
            "synapse:get_public_key" => {
                self.ingest(event).await?;
                let config = get_synapse_config().unwrap();
                let public_key = config.identity.public_key;
                let res_event = Event {
                    id: Uuid::new_v4(),
                    created_at: OffsetDateTime::now_utc(),
                    event_type: "synapse:return_public_key".to_string(),
                    module_kind: None,
                    module_slug: None,
                    agent: public_key.clone(),
                    target: None,
                    previous: None,
                    content: Some(public_key.clone()),
                    artifacts: None,
                    metadata: None,
                    links: None,
                    data: None,
                    expiration: None,
                };
                let events = vec![res_event];
                Ok(Some(events))
            }
            "synapse:list_all_events" => {
                let events = self.repo.retrieve("all".to_string()).await.unwrap();
                Ok(events)
            }
            _ => {
                self.ingest(event).await?;
                let config = get_synapse_config().unwrap();
                let public_key = config.identity.public_key;
                let res_event = Event {
                    id: Uuid::new_v4(),
                    created_at: OffsetDateTime::now_utc(),
                    event_type: "synapse:failed_to_handle_message".to_string(),
                    module_kind: None,
                    module_slug: None,
                    agent: public_key.clone(),
                    target: None,
                    previous: None,
                    content: Some(public_key.clone()),
                    artifacts: None,
                    metadata: None,
                    links: None,
                    data: None,
                    expiration: None,
                };
                let events = vec![res_event];
                Ok(Some(events))
            }
        }
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
    async fn execute(
        &self,
        cmd: CreateRemoteEventCommand,
    ) -> Result<Option<Vec<Event>>, CoreError> {
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
