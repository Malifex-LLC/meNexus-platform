// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use libp2p::PeerId;
use libp2p::request_response::ResponseChannel;
use protocol_snp::SnpMessage;
use synapse_core::TransportError;
use tokio::sync::oneshot;

pub enum Control {
    SendSnp {
        peer: PeerId,
        request: SnpMessage,
        ret: oneshot::Sender<Result<SnpMessage, TransportError>>,
    },
    SendResponse {
        channel: ResponseChannel<SnpMessage>,
        response: SnpMessage,
    },
    Provide {
        key: Vec<u8>,
    },
    QueryProviders {
        key: Vec<u8>,
        ret: oneshot::Sender<Vec<PeerId>>,
    },
}
