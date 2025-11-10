// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use synapse_core::CoreEvent;
use time::OffsetDateTime;
use uuid::Uuid;

pub struct SnpMessage {
    version: String,
    id: Uuid,
    correlation_id: Uuid,
    destination: Destination,
    agent_public_key: String,
    timestamp: OffsetDateTime,
    payload: CoreEvent,
    signature: String,
}

pub enum Destination {
    Local,
    Synapse { id: String },
    Multicast { topic: String },
}
