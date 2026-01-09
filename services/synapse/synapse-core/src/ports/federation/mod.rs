// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use async_trait::async_trait;

use crate::CoreError;
use crate::{TransportError, domain::events::Event};

// Outbound port
#[async_trait]
pub trait FederationTransport: Send + Sync {
    async fn send_message(
        &self,
        synapse_public_key: String,
        event: Event,
    ) -> Result<Vec<Event>, TransportError>;
}

// Inbound port
#[async_trait]
pub trait MessageHandler: Send + Sync {
    async fn handle_message(&self, event: Event) -> Result<Vec<Event>, CoreError>;
}
