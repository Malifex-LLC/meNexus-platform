// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::TransportConfig;
use base64;
use libp2p::Multiaddr;
use libp2p::identity::{Keypair, PeerId};
use libp2p::ping;
use libp2p_kad::{self, Event as KadEvent, store::MemoryStore};
use libp2p_swarm_derive::NetworkBehaviour;
use std::env;
use synapse_core::errors::CoreError;
use tracing::debug;

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

pub fn parse_config_from_env() -> Result<TransportConfig, CoreError> {
    let keypair: Keypair = match env::var("LIBP2P_KEY") {
        Ok(key_b64) => {
            let key_bytes = base64::decode(key_b64).expect("LIBP2P_KEY is not valid base64");
            Keypair::from_protobuf_encoding(&key_bytes)
                .expect("LIBP2P_KEY is not a valid libp2p protobuf key")
        }
        Err(_) => Keypair::generate_ed25519(),
    };

    let peer_id: PeerId = keypair.public().to_peer_id();
    let addr_str = format!("/ip4/127.0.0.1/tcp/63443/p2p/{}", peer_id);
    let _addr: Multiaddr = addr_str.parse().expect("Invalid Multiaddr");

    // Parse bootstrap list (ignore empty entries)
    let bootstrap_strings = env::var("BOOTSTRAP_LIST").unwrap_or_default();
    let mut bootstrap_addrs: Vec<Multiaddr> = Vec::new();
    for s in bootstrap_strings
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let addr: Multiaddr = s.parse().expect("Invalid Multiaddr in BOOTSTRAP_LIST");
        bootstrap_addrs.push(addr);
    }

    // Read listen port from env (default 0 = random)
    let listen_port_str = env::var("LIBP2P_PORT").unwrap_or_else(|_| "0".to_string());
    let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", listen_port_str)
        .parse()
        .expect("Invalid listen Multiaddr");

    debug!("Local peer id: {peer_id}");

    Ok(TransportConfig {
        keypair,
        bootstrap_addrs,
        listen_addr,
    })
}
