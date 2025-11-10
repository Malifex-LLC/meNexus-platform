// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::{RpcRequest, RpcResponse};
use libp2p::PeerId;
use synapse_core::TransportError;
use tokio::sync::oneshot;

pub enum Control {
    SendRpc {
        peer: PeerId,
        request: RpcRequest,
        ret: oneshot::Sender<Result<RpcResponse, TransportError>>,
    },
}
