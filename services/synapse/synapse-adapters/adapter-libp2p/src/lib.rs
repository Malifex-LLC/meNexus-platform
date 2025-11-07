// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod config;
pub mod discovery;
pub mod errors;
pub mod swarm;
pub mod transport;

use crate::{config::parse_config, transport::Libp2pTransport};
use synapse_config::SynapseConfig;
use synapse_core::TransportError;

pub async fn create_libp2p_transport(
    config: SynapseConfig,
) -> Result<Libp2pTransport, TransportError> {
    let transport_config = parse_config(&config)?;
    let transport = Libp2pTransport::new(transport_config);
    Ok(transport)
}
