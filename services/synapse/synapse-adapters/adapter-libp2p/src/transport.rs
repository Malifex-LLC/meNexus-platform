// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::RpcRequest;
use crate::control::Control;
use crate::{
    errors::Libp2pAdapterError,
    swarm::{create_swarm, run_swarm},
};
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use libp2p::identity::PublicKey;
use libp2p::{Multiaddr, PeerId, identity::Keypair};
use synapse_core::{TransportError, ports::federation::FederationTransport};
use tokio::sync::{mpsc, oneshot};
use tracing::warn;

pub struct Libp2pTransport {
    config: TransportConfig,
    tx: mpsc::Sender<Control>,
    rx: Option<mpsc::Receiver<Control>>,
}

#[derive(Clone, Debug)]
pub struct TransportConfig {
    pub keypair: Keypair,
    pub bootstrap_addrs: Vec<Multiaddr>,
    pub listen_addr: Multiaddr,
}

impl Libp2pTransport {
    pub fn new(config: TransportConfig) -> Self {
        let (tx, rx) = mpsc::channel::<Control>(64);
        Self {
            config,
            tx,
            rx: Some(rx),
        }
    }

    pub async fn start(&mut self) -> Result<(), Libp2pAdapterError> {
        let swarm = create_swarm(self.config.clone())?;
        let rx = self.rx.take().expect("transport already started");
        tokio::spawn(async move {
            if let Err(e) = run_swarm(swarm, rx).await {
                warn!("libp2p swarm exited with error: {e:?}");
            }
        });
        Ok(())
    }
}

#[async_trait]
impl FederationTransport for Libp2pTransport {
    async fn send_message(
        &self,
        synapse_public_key: String,
        event: synapse_core::domain::events::Event,
    ) -> Result<synapse_core::domain::events::Event, TransportError> {
        let peer_id = peer_id_from_urlsafe_b64_pk(&synapse_public_key)
            .map_err(|e| TransportError::Other(e.to_string()))?;

        let req = RpcRequest {
            action: "create_event".to_string(),
            payload: serde_json::to_value(&event)
                .map_err(|e| TransportError::Other(e.to_string()))?,
        };

        let (ret_tx, ret_rx) = oneshot::channel();
        self.tx
            .send(Control::SendRpc {
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
        Ok(event)
    }
}

fn peer_id_from_urlsafe_b64_pk(s: &str) -> Result<PeerId, Libp2pAdapterError> {
    let bytes = URL_SAFE_NO_PAD.decode(s)?;
    let pk = PublicKey::try_decode_protobuf(&bytes)?;
    Ok(pk.to_peer_id())
}
