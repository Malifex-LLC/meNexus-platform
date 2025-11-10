// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::Libp2pEvent;
use crate::config::{RpcRequest, RpcResponse};
use crate::control::Control;
use crate::discovery::setup_bootstrap;
use crate::errors::Libp2pAdapterError;
use crate::{config::Libp2pBehaviour, transport::TransportConfig};
use futures::StreamExt;
use libp2p::StreamProtocol;
use libp2p::request_response::OutboundRequestId;
use libp2p::request_response::ProtocolSupport;
use libp2p::request_response::json;
use libp2p::request_response::{Event as ReqResEvent, Message as ReqResMessage};
use libp2p::{
    noise, ping,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux,
};
use libp2p_kad::{self, Config as KadConfig, Mode, store::MemoryStore};
use std::collections::HashMap;
use std::time::Duration;
use synapse_core::TransportError;
use tokio::sync::{mpsc, oneshot};
use tracing::info;

pub fn create_swarm(config: TransportConfig) -> Result<Swarm<Libp2pBehaviour>, Libp2pAdapterError> {
    info!("Creating swarm for config: {config:?}");
    let kad_cfg = KadConfig::default();
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let mut kad = libp2p_kad::Behaviour::with_config(
                key.public().to_peer_id(),
                MemoryStore::new(key.public().to_peer_id()),
                kad_cfg,
            );
            kad.set_mode(Some(Mode::Server));
            let protocols = [(
                StreamProtocol::new("/menexus/rpc/1.0.0"),
                ProtocolSupport::Full,
            )];
            let req_res =
                json::Behaviour::<RpcRequest, RpcResponse>::new(protocols, Default::default());
            Ok(Libp2pBehaviour {
                ping: ping::Behaviour::default(),
                kad,
                req_res,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();
    swarm.listen_on(config.listen_addr)?;
    setup_bootstrap(&mut swarm, config.bootstrap_addrs);
    info!("Libp2p swarm created");
    Ok(swarm)
}

pub async fn run_swarm(
    mut swarm: Swarm<Libp2pBehaviour>,
    mut rx: mpsc::Receiver<Control>,
) -> Result<(), Libp2pAdapterError> {
    info!("Running swarm...");
    //let mut known_peers: HashMap<PublicKey, PeerId> = HashMap::new();
    let mut pending: HashMap<
        OutboundRequestId,
        oneshot::Sender<Result<RpcResponse, TransportError>>,
    > = HashMap::new();

    loop {
        tokio::select! {
            Some(ctrl) = rx.recv() => {
                match ctrl {
                    Control::SendRpc { peer, request, ret } => {
                        let req_id = swarm.behaviour_mut().req_res.send_request(&peer, request);
                        pending.insert(req_id, ret);
                    }
                }
            },
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(Libp2pEvent::Ping(event)) => info!("{event:?}"),
                    SwarmEvent::Behaviour(Libp2pEvent::Kad(event)) => info!("{event:?}"),
                    SwarmEvent::ConnectionEstablished {
                        peer_id,
                        connection_id: _,
                        endpoint: _,
                        num_established: _,
                        concurrent_dial_errors: _,
                        established_in: _,
                    } => {
                        info!("Connected to {peer_id}");
                    }
                    SwarmEvent::ConnectionClosed { peer_id, .. } => info!("Disconnected from {peer_id}"),
                    SwarmEvent::Behaviour(Libp2pEvent::ReqRes(ev)) => match ev {
                        ReqResEvent::Message { message, .. } => match message {
                            ReqResMessage::Request { request, channel, .. } => {
                                // Inbound request: handle and reply
                                // TODO: call your app logic here; demo returns OK
                                let resp = RpcResponse { ok: true, message: None };
                                let _ = swarm.behaviour_mut().req_res.send_response(channel, resp);
                            }
                            ReqResMessage::Response { request_id, response } => {
                                if let Some(ch) = pending.remove(&request_id) {
                                    let _ = ch.send(Ok(response));
                                }
                            }
                        },
                        ReqResEvent::OutboundFailure { request_id, error, .. } => {
                            if let Some(ch) = pending.remove(&request_id) {
                                let err = Libp2pAdapterError::Io(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    format!("outbound error: {error}"),
                                ));
                                let _ = ch.send(Err(err.into()));
                            }
                        }
                        ReqResEvent::InboundFailure { error, .. } => {
                            tracing::warn!("inbound error: {error}");
                        }
                        ReqResEvent::ResponseSent { .. } => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
