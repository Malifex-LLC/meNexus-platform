// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
use synapse_core::domain::events::Event;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SnpMessage {
    pub version: String,
    pub id: Uuid,
    pub correlation_id: Uuid,
    pub destination: Destination,
    pub agent_public_key: String,
    pub timestamp: OffsetDateTime,
    pub payload: SnpPayload,
    pub signature: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum SnpPayload {
    Command {
        action: String,
        event: Event,
    },
    Reply {
        ok: bool,
        events: Option<Vec<Event>>,
        error: Option<String>,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Destination {
    Local,
    Synapse { id: String },
    Multicast { topic: String },
}
