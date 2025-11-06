// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use libp2p::multiaddr::Protocol;
use libp2p::{Multiaddr, Swarm};
use std::env;

use crate::config::Libp2pBehaviour;

pub fn setup_bootstrap(swarm: &mut Swarm<Libp2pBehaviour>, bootstrap_list: Vec<Multiaddr>) {
    for mut addr in bootstrap_list {
        if let Some(Protocol::P2p(peer_id)) = addr.pop() {
            let bootstrap_peer = peer_id;
            swarm
                .behaviour_mut()
                .kad
                .add_address(&bootstrap_peer, addr.clone());
            if let Err(e) = swarm.dial(addr) {
                eprintln!("Failed to dial bootstrap: {e}");
            }
        } else {
            eprintln!("Bootstrap addr missing /p2p suffix");
        }
    }
    if let Err(e) = swarm.behaviour_mut().kad.bootstrap() {
        eprintln!("Bootstrap failed: {e}");
    }
}
