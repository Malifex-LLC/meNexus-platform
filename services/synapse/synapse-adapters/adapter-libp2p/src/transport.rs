// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;

use crate::control::Control;
use crate::{
    errors::Libp2pAdapterError,
    swarm::{create_swarm, run_swarm},
};
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use dashmap::DashMap;
use libp2p::identity::PublicKey;
use libp2p::{Multiaddr, PeerId, identity::Keypair};
use protocol_snp::{
    Destination::{Local, Multicast, Synapse},
    SnpMessage,
    SnpPayload::{Command, Reply},
};
use synapse_core::domain::events::Event;
use synapse_core::ports::federation::MessageHandler;
use synapse_core::{TransportError, ports::federation::FederationTransport};
use time::OffsetDateTime;
use tokio::sync::{mpsc, oneshot};
use tracing::warn;
use uuid::Uuid;

pub struct Libp2pTransport {
    config: TransportConfig,
    tx: mpsc::Sender<Control>,
    rx: Option<mpsc::Receiver<Control>>,
    inbound_handler: Arc<dyn MessageHandler>,
    known_peers: Arc<DashMap<String, String>>,
}

#[derive(Clone, Debug)]
pub struct TransportConfig {
    pub keypair: Keypair,
    pub bootstrap_addrs: Vec<Multiaddr>,
    pub listen_addr: Multiaddr,
    pub announce_addrs: Vec<Multiaddr>,
}

impl Libp2pTransport {
    pub fn new(
        config: TransportConfig,
        handler: Arc<dyn MessageHandler + Send + Sync>,
        known_peers: Arc<DashMap<String, String>>,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Control>(64);
        Self {
            config,
            tx,
            rx: Some(rx),
            inbound_handler: handler,
            known_peers,
        }
    }

    pub async fn start(&mut self) -> Result<(), Libp2pAdapterError> {
        let swarm = create_swarm(self.config.clone())?;
        let rx = self.rx.take().expect("transport already started");
        let handler = self.inbound_handler.clone();
        let known_peers = self.known_peers.clone();

        tokio::spawn(async move {
            if let Err(e) = run_swarm(swarm, rx, handler, known_peers).await {
                warn!("libp2p swarm exited with error: {e:?}");
            }
        });
        Ok(())
    }

    pub fn public_key_for_peer(&self, peer_id: &str) -> Option<String> {
        self.known_peers
            .iter()
            .find(|entry| entry.value() == peer_id)
            .map(|entry| entry.key().clone())
    }

    pub async fn announce_profile(&self, profile_pk: &str) -> Result<(), TransportError> {
        let key = profile_pk.as_bytes().to_vec();
        self.tx.send(Control::Provide { key }).await.unwrap();
        Ok(())
    }

    pub async fn lookup_profile_providers(
        &self,
        profile_pk: &str,
    ) -> Result<Vec<String>, TransportError> {
        let key = profile_pk.as_bytes().to_vec();
        let (ret_tx, ret_rx) = oneshot::channel();

        self.tx
            .send(Control::QueryProviders { key, ret: ret_tx })
            .await
            .map_err(|_| TransportError::Other("swarm control channel closed".into()))?;

        let peers = ret_rx
            .await
            .map_err(|_| TransportError::Other("provider query dropped".into()))?;

        let mut providers = Vec::new();
        for peer in peers {
            let peer_str = peer.to_string();
            if let Some(public_key) = self.public_key_for_peer(&peer_str) {
                providers.push(public_key);
            } else {
                warn!("Can resolve public_key for provided PeerId")
            }
        }

        Ok(providers)
    }
}

#[async_trait]
impl FederationTransport for Libp2pTransport {
    async fn send_message(
        &self,
        synapse_public_key: String,
        event: Event,
    ) -> Result<Vec<Event>, TransportError> {
        let peer_id = peer_id_from_urlsafe_b64_pk(&synapse_public_key)
            .map_err(|e| TransportError::Other(e.to_string()))?;

        // Extract signature from event for the SNP message envelope
        let event_signature = event.agent_signature.clone().unwrap_or_default();
        
        let req = SnpMessage {
            version: "1.0.0".to_string(),
            id: Uuid::new_v4(),
            correlation_id: Uuid::new_v4(),
            destination: Synapse {
                id: peer_id.to_string(),
            },
            // Use the actual agent public key from the event
            agent_public_key: event.agent.clone(),
            timestamp: OffsetDateTime::now_utc(),
            payload: Command {
                action: event.event_type.clone(),
                event: event.clone(),
            },
            // Use the event's signature for the message envelope
            signature: event_signature,
        };

        let (ret_tx, ret_rx) = oneshot::channel();
        self.tx
            .send(Control::SendSnp {
                peer: peer_id,
                request: req,
                ret: ret_tx,
            })
            .await
            .map_err(|_| TransportError::Other("swarm control channel closed".into()))?;

        let resp = ret_rx
            .await
            .map_err(|_| TransportError::Other("swarm dropped response".into()))?
            .map_err(|e| TransportError::Other(e.to_string()))?;

        let _ = resp;
        match resp.payload {
            Reply {
                ok: true, events, ..
            } => Ok(events),
            Reply {
                ok: false,
                error: Some(error),
                ..
            } => Err(TransportError::Other(error)),
            _ => Err(TransportError::Other("unexpected response payload".into())),
        }
    }
}

fn peer_id_from_urlsafe_b64_pk(s: &str) -> Result<PeerId, Libp2pAdapterError> {
    let bytes = URL_SAFE_NO_PAD.decode(s)?;
    let pk = PublicKey::try_decode_protobuf(&bytes)?;
    Ok(pk.to_peer_id())
}
