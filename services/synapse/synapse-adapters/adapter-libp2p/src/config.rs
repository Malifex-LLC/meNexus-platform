// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::Libp2pAdapterError;
use crate::transport::TransportConfig;
use base64;
use base64::{Engine as _, engine::general_purpose};
use libp2p::Multiaddr;
use libp2p::ping;
use libp2p_kad::{self, Event as KadEvent, store::MemoryStore};
use libp2p_swarm_derive::NetworkBehaviour;
use synapse_config::SynapseConfig;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Libp2pEvent")]
pub struct Libp2pBehaviour {
    pub ping: ping::Behaviour,
    pub kad: libp2p_kad::Behaviour<MemoryStore>,
}

#[derive(Debug)]
pub enum Libp2pEvent {
    Ping(ping::Event),
    Kad(KadEvent),
}

impl From<ping::Event> for Libp2pEvent {
    fn from(event: ping::Event) -> Self {
        Libp2pEvent::Ping(event)
    }
}

impl From<KadEvent> for Libp2pEvent {
    fn from(event: KadEvent) -> Self {
        Libp2pEvent::Kad(event)
    }
}

pub fn parse_config(config: &SynapseConfig) -> Result<TransportConfig, Libp2pAdapterError> {
    let s = std::fs::read_to_string(&config.identity.key_path)?;
    let b = general_purpose::STANDARD.decode(s.trim())?;
    let keypair = libp2p::identity::Keypair::from_protobuf_encoding(&b)?;

    let bootstrap = config.p2p.bootstrap.clone();
    let mut bootstrap_addrs: Vec<Multiaddr> = Vec::new();
    for s in bootstrap {
        let addr: Multiaddr = s.parse().expect("Invalid Multiaddr in BOOTSTRAP_LIST");
        bootstrap_addrs.push(addr);
    }

    let listen_addr: Multiaddr = config.p2p.listen_addrs.clone();

    Ok(TransportConfig {
        keypair,
        bootstrap_addrs,
        listen_addr,
    })
}
