// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::events::{CreateEventCommand, CreateEventUseCase};
use std::sync::Arc;
use synapse_core::domain::events::Event;
use synapse_core::ports::events::event_repository::EventRepository;
use synapse_core::{CoreError, PersistenceError};
use uuid::Uuid;

pub struct EventService<R: EventRepository> {
    repo: Arc<R>,
}

impl<R: EventRepository> EventService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R: EventRepository + Send + Sync> CreateEventUseCase for EventService<R> {
    async fn execute(&self, cmd: CreateEventCommand) -> Result<Event, CoreError> {
        if cmd.event_type.trim().is_empty() {
            return Err(CoreError::persistence(
                "event_type must not be empty".to_string(),
            ));
        }
        if cmd.agent_public_key.trim().is_empty() {
            return Err(CoreError::persistence(
                "agent_public_key must not be empty".to_string(),
            ));
        }

        let event = Event {
            event_id: Uuid::new_v4(),
            event_type: cmd.event_type,
            agent_public_key: cmd.agent_public_key,
        };

        Ok(self.repo.create(event).await?)
    }
}
