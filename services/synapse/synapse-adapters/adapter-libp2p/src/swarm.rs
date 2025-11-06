// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::Libp2pEvent;
use crate::discovery::setup_bootstrap;
use crate::{config::Libp2pBehaviour, transport::TransportConfig};
use futures::StreamExt;
use libp2p::{
    noise, ping,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux,
};
use libp2p_kad::{self, Config as KadConfig, Mode, store::MemoryStore};
use std::time::Duration;
use synapse_core::errors::CoreError;
use tracing::debug;

pub fn create_swarm(config: TransportConfig) -> Result<Swarm<Libp2pBehaviour>, CoreError> {
    debug!("Creating swarm for config: {config:?}");
    let kad_cfg = KadConfig::default();
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )
        .unwrap()
        .with_behaviour(|key| {
            let mut kad = libp2p_kad::Behaviour::with_config(
                key.public().to_peer_id(),
                MemoryStore::new(key.public().to_peer_id()),
                kad_cfg,
            );
            kad.set_mode(Some(Mode::Server));
            Ok(Libp2pBehaviour {
                ping: ping::Behaviour::default(),
                kad,
            })
        })
        .unwrap()
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();
    swarm.listen_on(config.listen_addr);
    setup_bootstrap(&mut swarm, config.bootstrap_addrs);
    debug!("Libp2p swarm created");
    Ok(swarm)
}

pub async fn run_swarm(mut swarm: Swarm<Libp2pBehaviour>) {
    debug!("Running swarm...");
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(Libp2pEvent::Ping(event)) => println!("{event:?}"),
            SwarmEvent::Behaviour(Libp2pEvent::Kad(event)) => println!("{event:?}"),
            SwarmEvent::ConnectionEstablished { peer_id, .. } => println!("Connected to {peer_id}"),
            SwarmEvent::ConnectionClosed { peer_id, .. } => println!("Disconnected from {peer_id}"),
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                if let Some(id) = peer_id {
                    println!("Dial error for {id}: {error}");
                } else {
                    println!("Dial error: {error}");
                }
            }
            _ => {}
        }
    }
}
