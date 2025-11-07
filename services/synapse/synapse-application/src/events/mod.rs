// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod event_service;

use synapse_core::{domain::events::Event, errors::CoreError};

#[derive(Clone, Debug)]
pub struct CreateEventCommand {
    pub event_type: String,
    pub agent_public_key: String,
}

#[async_trait::async_trait]
pub trait CreateEventUseCase: Send + Sync {
    async fn execute(&self, cmd: CreateEventCommand) -> Result<Event, CoreError>;
}
