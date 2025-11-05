// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod config;

use async_trait::async_trait;
use libp2p::Swarm;
use synapse_core::ports::federation::SnpTransport;

pub struct Libp2pSnpTransport {
    swarm: String,
}

#[async_trait]
impl SnpTransport for Libp2pSnpTransport {}
