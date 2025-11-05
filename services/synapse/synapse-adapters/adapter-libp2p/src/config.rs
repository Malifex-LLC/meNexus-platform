// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use futures::StreamExt;
use std::time::Duration;
use std::{env, error::Error};

use libp2p::{
    Multiaddr, PeerId, identity,
    multiaddr::Protocol,
    noise, ping,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux,
};
use libp2p_kad::{
    self, BootstrapOk, Config as KadConfig, Event as KadEvent, Mode, store::MemoryStore,
};
use libp2p_mdns::{Config as MdnsConfig, Event as MdnsEvent, tokio::Behaviour as Mdns};
use libp2p_swarm_derive::NetworkBehaviour;
use synapse_core::errors::CoreError;
use tracing_subscriber::EnvFilter;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyEvent")]
pub struct MyBehaviour {
    pub ping: ping::Behaviour,
    pub mdns: Mdns,
    pub kad: libp2p_kad::Behaviour<MemoryStore>,
}

#[derive(Debug)]
pub enum MyEvent {
    Ping(ping::Event),
    Mdns(MdnsEvent),
    Kad(KadEvent),
}

impl From<ping::Event> for MyEvent {
    fn from(event: ping::Event) -> Self {
        MyEvent::Ping(event)
    }
}

impl From<MdnsEvent> for MyEvent {
    fn from(event: MdnsEvent) -> Self {
        MyEvent::Mdns(event)
    }
}

impl From<KadEvent> for MyEvent {
    fn from(event: KadEvent) -> Self {
        MyEvent::Kad(event)
    }
}

pub fn create_swarm(
    bootstrap_addrs: Vec<Multiaddr>,
    keypair: Option<identity::Keypair>,
) -> Result<Swarm<MyBehaviour>, Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let keypair = keypair.unwrap_or_else(identity::Keypair::generate_ed25519);
    let peer_id = keypair.public().to_peer_id();

    let kad_cfg = KadConfig::default();

    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let mdns = Mdns::new(MdnsConfig::default(), key.public().to_peer_id())?;
            let mut kad = libp2p_kad::Behaviour::with_config(
                key.public().to_peer_id(),
                MemoryStore::new(key.public().to_peer_id()),
                kad_cfg,
            );
            kad.set_mode(Some(Mode::Server));
            Ok(MyBehaviour {
                ping: ping::Behaviour::default(),
                mdns,
                kad,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut has_bootstrap = false;
    for mut addr in bootstrap_addrs {
        if let Some(Protocol::P2p(peer_id)) = addr.pop() {
            let bootstrap_peer = peer_id;
            swarm
                .behaviour_mut()
                .kad
                .add_address(&bootstrap_peer, addr.clone());
            has_bootstrap = true;
            if let Err(e) = swarm.dial(addr) {
                eprintln!("Failed to dial bootstrap: {e}");
            }
        } else {
            eprintln!("Bootstrap addr missing /p2p suffix");
        }
    }

    if has_bootstrap {
        if let Err(e) = swarm.behaviour_mut().kad.bootstrap() {
            eprintln!("Bootstrap failed: {e}");
        }
    }

    Ok(swarm)
}

pub async fn run_swarm(mut swarm: Swarm<MyBehaviour>) -> Result<(), CoreError> {
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(MyEvent::Ping(event)) => println!("{event:?}"),
            SwarmEvent::Behaviour(MyEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                for (peer_id, addr) in peers {
                    println!("mDNS discovered peer {peer_id} at {addr}");
                    if let Err(e) = swarm.dial(addr.clone()) {
                        println!("Failed to dial {peer_id}: {e}");
                    }
                }
            }
            SwarmEvent::Behaviour(MyEvent::Mdns(MdnsEvent::Expired(peers))) => {
                for (peer_id, addr) in peers {
                    println!("mDNS expired peer {peer_id} at {addr}");
                }
            }
            SwarmEvent::Behaviour(MyEvent::Kad(event)) => println!("{event:?}"),
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

pub async fn initialize() -> Result<(), CoreError> {
    // Make both arms return a Keypair (no Option, no nested Result)
    let keypair: identity::Keypair = match env::var("LIBP2P_KEY") {
        Ok(key_b64) => {
            let key_bytes = base64::decode(key_b64).expect("LIBP2P_KEY is not valid base64");
            identity::Keypair::from_protobuf_encoding(&key_bytes)
                .expect("LIBP2P_KEY is not a valid libp2p protobuf key")
        }
        Err(_) => identity::Keypair::generate_ed25519(),
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

    // Build and run swarm
    let mut swarm = create_swarm(bootstrap_addrs, Some(keypair)).unwrap();

    // If create_swarm doesn't already listen, do it here:
    // swarm.listen_on(listen_addr).expect("listen_on failed");

    run_swarm(swarm);
    Ok(())
}
