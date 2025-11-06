// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::TransportConfig;
use crate::swarm::{create_swarm, run_swarm};
use async_trait::async_trait;
use libp2p::{Multiaddr, identity};
use synapse_core::{errors::CoreError, ports::federation::SnpTransport};

pub struct Libp2pTransport {
    config: TransportConfig,
}

impl Libp2pTransport {
    pub fn new(config: TransportConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self) -> Result<(), CoreError> {
        let swarm = create_swarm(self.config.clone());
        tokio::spawn(async move { run_swarm(swarm.unwrap()).await });
        Ok(())
    }
}

#[async_trait]
impl SnpTransport for Libp2pTransport {}
