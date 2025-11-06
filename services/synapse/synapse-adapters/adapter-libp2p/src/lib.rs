// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod config;
pub mod discovery;
pub mod swarm;
pub mod transport;

use crate::{config::parse_config_from_env, transport::Libp2pTransport};
use libp2p::{Multiaddr, identity::Keypair};
use synapse_core::errors::CoreError;

#[derive(Clone)]
pub struct TransportConfig {
    pub keypair: Keypair,
    pub bootstrap_addrs: Vec<Multiaddr>,
    pub listen_addr: Multiaddr,
}

pub async fn create_libp2p_transport() -> Result<Libp2pTransport, CoreError> {
    let config = parse_config_from_env().unwrap();
    let transport = Libp2pTransport::new(config);
    Ok(transport)
}
