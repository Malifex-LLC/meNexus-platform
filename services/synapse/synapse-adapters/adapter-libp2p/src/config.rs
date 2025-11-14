// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::Libp2pAdapterError;
use crate::transport::TransportConfig;

use base64::{Engine as _, engine::general_purpose};

use libp2p::ping;
use libp2p::request_response::Event as ReqResEvent;
use libp2p::request_response::json::Behaviour as JsonBehaviour;
use libp2p::{Multiaddr, identity};

use libp2p_kad::{self, Event as KadEvent, store::MemoryStore};
use libp2p_swarm_derive::NetworkBehaviour;

use serde::{Deserialize, Serialize};
use synapse_config::SynapseConfig;
use synapse_core::domain::events::Event;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "Libp2pEvent")]
pub struct Libp2pBehaviour {
    //pub ping: ping::Behaviour,
    pub kad: libp2p_kad::Behaviour<MemoryStore>,
    pub req_res: JsonBehaviour<RpcRequest, RpcResponse>,
}

type JsonReqResEvent = ReqResEvent<RpcRequest, RpcResponse>;

#[derive(Debug)]
pub enum Libp2pEvent {
    Ping(ping::Event),
    Kad(KadEvent),
    ReqRes(JsonReqResEvent),
}

impl From<ping::Event> for Libp2pEvent {
    fn from(e: ping::Event) -> Self {
        Libp2pEvent::Ping(e)
    }
}
impl From<KadEvent> for Libp2pEvent {
    fn from(e: KadEvent) -> Self {
        Libp2pEvent::Kad(e)
    }
}
impl From<JsonReqResEvent> for Libp2pEvent {
    fn from(e: JsonReqResEvent) -> Self {
        Libp2pEvent::ReqRes(e)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    pub action: String,
    pub event: Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    pub ok: bool,
    pub event: Option<Event>,
}

pub fn parse_config(config: &SynapseConfig) -> Result<TransportConfig, Libp2pAdapterError> {
    let s = std::fs::read_to_string(&config.identity.private_key_path)?;
    let b = general_purpose::STANDARD.decode(s.trim())?;
    let keypair = identity::Keypair::from_protobuf_encoding(&b)?;

    let mut bootstrap_addrs: Vec<Multiaddr> = Vec::new();
    for s in config.p2p.bootstrap.clone() {
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
