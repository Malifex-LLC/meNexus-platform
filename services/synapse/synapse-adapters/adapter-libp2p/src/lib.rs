// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod config;
pub mod control;
pub mod discovery;
pub mod errors;
pub mod swarm;
pub mod transport;

use crate::{config::parse_config, transport::Libp2pTransport};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;
use synapse_config::SynapseConfig;
use synapse_core::TransportError;
use synapse_core::ports::federation::MessageHandler;

pub async fn create_libp2p_transport(
    config: SynapseConfig,
    handler: Arc<dyn MessageHandler + Send + Sync>,
    known_peers: Arc<DashMap<String, String>>,
) -> Result<Libp2pTransport, TransportError> {
    let transport_config = parse_config(&config)?;
    let transport = Libp2pTransport::new(transport_config, handler, known_peers);
    Ok(transport)
}

pub async fn initialize_p2p(
    config: SynapseConfig,
    handler: Arc<dyn MessageHandler + Send + Sync>,
    known_peers: Arc<DashMap<String, String>>,
) -> Result<Libp2pTransport, TransportError> {
    let mut transport = create_libp2p_transport(config, handler, known_peers).await?;
    transport.start().await?;
    Ok(transport)
}
