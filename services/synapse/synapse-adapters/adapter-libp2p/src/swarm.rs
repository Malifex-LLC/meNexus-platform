// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::Libp2pEvent;
use crate::control::Control;
use crate::discovery::setup_bootstrap;
use crate::errors::Libp2pAdapterError;
use crate::{config::Libp2pBehaviour, transport::TransportConfig};
use dashmap::DashMap;
use futures::StreamExt;
use libp2p::StreamProtocol;
use libp2p::request_response::OutboundRequestId;
use libp2p::request_response::ProtocolSupport;
use libp2p::request_response::{json, Config as ReqResConfig};
use libp2p::request_response::{Event as ReqResEvent, Message as ReqResMessage};
use libp2p::{
    noise, ping,
    swarm::{Swarm, SwarmEvent},
    tcp, yamux,
};
use libp2p_kad::{
    self, Config as KadConfig, Event as KademliaEvent, GetProvidersOk, Mode, QueryId, QueryResult,
    store::MemoryStore,
};
use protocol_snp::{
    Destination::{Local, Multicast, Synapse},
    SnpMessage,
    SnpPayload::{Command, Reply},
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use synapse_core::TransportError;
use synapse_core::domain::events::Event;
use synapse_core::ports::federation::MessageHandler;
use time::OffsetDateTime;
use tokio::sync::{mpsc, oneshot};
use tracing::info;
use uuid::Uuid;

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
                StreamProtocol::new("/menexus/snp/1.0.0"),
                ProtocolSupport::Full,
            )];
            // Use a longer request timeout to allow for network latency and handler I/O
            let req_res_config = ReqResConfig::default()
                .with_request_timeout(Duration::from_secs(120));
            let req_res =
                json::Behaviour::<SnpMessage, SnpMessage>::new(protocols, req_res_config);
            Ok(Libp2pBehaviour {
                //ping: ping::Behaviour::default(),
                kad,
                req_res,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();
    swarm.listen_on(config.listen_addr)?;

    // Add external/announce addresses so peers can reach us
    for addr in config.announce_addrs {
        info!("Adding external address: {addr}");
        swarm.add_external_address(addr);
    }

    setup_bootstrap(&mut swarm, config.bootstrap_addrs);
    info!("Libp2p swarm created");
    Ok(swarm)
}

pub async fn run_swarm(
    mut swarm: Swarm<Libp2pBehaviour>,
    mut rx: mpsc::Receiver<Control>,
    ctrl_tx: mpsc::Sender<Control>,
    handler: Arc<dyn MessageHandler + Send + Sync>,
    known_peers: Arc<DashMap<String, String>>,
) -> Result<(), Libp2pAdapterError> {
    info!("Running swarm...");
    let mut pending: HashMap<
        OutboundRequestId,
        oneshot::Sender<Result<SnpMessage, TransportError>>,
    > = HashMap::new();
    let mut provider_queries: HashMap<QueryId, oneshot::Sender<Vec<libp2p::PeerId>>> =
        HashMap::new();

    loop {
        tokio::select! {
            Some(ctrl) = rx.recv() => {
                match ctrl {
                    Control::SendSnp { peer, request, ret } => {
                        let req_id = swarm.behaviour_mut().req_res.send_request(&peer, request);
                        pending.insert(req_id, ret);
                    }
                    Control::SendResponse { channel, response } => {
                        let _ = swarm.behaviour_mut().req_res.send_response(channel, response);
                    }
                    Control::Provide {key} => {
                        if let Err(e) = swarm.behaviour_mut().kad.start_providing(key.into()) {
                            tracing::warn!("start_providing failed: {e:?}");
                        }
                    }
                    Control::QueryProviders { key, ret } => {
                        let query_id = swarm.behaviour_mut().kad.get_providers(key.into());
                        provider_queries.insert(query_id, ret);
                    }
                }
            },
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => info!("Listening on {address:?}"),
                    SwarmEvent::Behaviour(Libp2pEvent::Ping(event)) => info!("{event:?}"),
                    SwarmEvent::Behaviour(Libp2pEvent::Kad(event)) => match event {
                        KademliaEvent::OutboundQueryProgressed { id, result, .. } => {
                            if let Some(ret) = provider_queries.remove(&id) {
                                match result {
                                    QueryResult::GetProviders(Ok(ok)) => {
                                        match ok {
                                            GetProvidersOk::FoundProviders { providers, .. } => {
                                                let providers = providers.into_iter().collect();
                                                let _ = ret.send(providers);
                                            }
                                            GetProvidersOk::FinishedWithNoAdditionalRecord { .. } => {
                                                let _ = ret.send(Vec::new());
                                            }
                                        }
                                    }
                                    QueryResult::GetProviders(Err(_)) => {
                                        let _ = ret.send(Vec::new());
                                    }
                                    _ => {
                                        // Put sender back if this wasn't the final GetProviders result
                                        provider_queries.insert(id, ret);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    SwarmEvent::ConnectionEstablished {
                        peer_id,
                        connection_id: _,
                        endpoint: _,
                        num_established: _,
                        concurrent_dial_errors: _,
                        established_in: _,
                    } => {
                        info!("Connected to {peer_id}");
                        let req = SnpMessage {
                            version: "1.0.0".to_string(),
                            id: Uuid::new_v4(),
                            correlation_id: Uuid::new_v4(),
                            destination: Synapse { id: peer_id.to_string() },
                            agent_public_key: "agent_public_key".to_string(),
                            timestamp: OffsetDateTime::now_utc(),
                            payload: Command {
                                action: "synapse:get_public_key".to_string(),
                                event: Event::new()
                                    .with_event_type("synapse:get_public_key")
                                    .with_module_kind("core")
                                    .build()
                            },
                            signature: "signature".to_string(),
                        };
                        let _ = swarm.behaviour_mut().req_res.send_request(&peer_id, req);
                    }


                    SwarmEvent::ConnectionClosed { peer_id, .. } => info!("Disconnected from {peer_id}"),
                    SwarmEvent::Behaviour(Libp2pEvent::ReqRes(ev)) => match ev {
                        ReqResEvent::Message { peer, message, .. } => match message {
                            ReqResMessage::Request { request, channel, .. } => {
                                // Inbound request: spawn handler task to avoid blocking the swarm
                                info!("Received incoming message: {:?}", request);
                                match request.payload {
                                    Command { action: _, event } => {
                                        // Spawn handler in background task to prevent blocking the swarm event loop
                                        let handler = handler.clone();
                                        let ctrl_tx = ctrl_tx.clone();
                                        let peer_str = peer.to_string();
                                        tokio::spawn(async move {
                                            let response = match handler.handle_message(event).await {
                                                Ok(saved) => {
                                                    SnpMessage {
                                                        version: "1.0.0".to_string(),
                                                        id: Uuid::new_v4(),
                                                        correlation_id: Uuid::new_v4(),
                                                        destination: Synapse { id: peer_str },
                                                        agent_public_key: "agent_public_key".to_string(),
                                                        timestamp: OffsetDateTime::now_utc(),
                                                        payload: Reply {
                                                            ok: true,
                                                            events: saved,
                                                            error: None,
                                                        },
                                                        signature: "signature".to_string(),
                                                    }
                                                }
                                                Err(err) => {
                                                    tracing::warn!("ingest/handle failed: {err:?}");
                                                    SnpMessage {
                                                        version: "1.0.0".to_string(),
                                                        id: Uuid::new_v4(),
                                                        correlation_id: Uuid::new_v4(),
                                                        destination: Synapse { id: peer_str },
                                                        agent_public_key: "agent_public_key".to_string(),
                                                        timestamp: OffsetDateTime::now_utc(),
                                                        payload: Reply {
                                                            ok: false,
                                                            events: vec![],
                                                            error: Some(err.to_string()),
                                                        },
                                                        signature: "signature".to_string(),
                                                    }
                                                }
                                            };
                                            info!("Sending outbound response: {:?}", response);
                                            let _ = ctrl_tx.send(Control::SendResponse { channel, response }).await;
                                        });
                                    }
                                    _ => {
                                        let resp = SnpMessage {
                                            version: "1.0.0".to_string(),
                                            id: Uuid::new_v4(),
                                            correlation_id: Uuid::new_v4(),
                                            destination: Synapse { id: peer.to_string() },
                                            agent_public_key: "agent_public_key".to_string(),
                                            timestamp: OffsetDateTime::now_utc(),
                                            payload: Reply {
                                                ok: false,
                                                events: vec![],
                                                error: Some("unsupported payload".into()),
                                            },
                                            signature: "signature".to_string(),
                                        };
                                        let _ = ctrl_tx.send(Control::SendResponse { channel, response: resp }).await;
                                    }
                                }
                            }
                            ReqResMessage::Response { request_id, response } => {
                                if let Some(ch) = pending.remove(&request_id) {
                                    let _ = ch.send(Ok(response.clone()));
                                }
                                if let Reply { ok: true, events: events, .. } = &response.payload {
                                    for evt in events.iter() {
                                        match evt.event_type.as_str() {
                                            "synapse:return_public_key" => {
                                                if let Some(pk) = evt.content.clone() {
                                                    known_peers.insert(pk, peer.to_string());
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
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
