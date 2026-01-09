// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use synapse_core::TransportError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Libp2pAdapterError {
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),

    #[error(transparent)]
    Libp2pBehaviourBuilder(#[from] libp2p::BehaviourBuilderError),

    #[error(transparent)]
    Libp2pDecodingError(#[from] libp2p::identity::DecodingError),

    #[error(transparent)]
    Libp2pTransportError(#[from] libp2p::core::transport::TransportError<std::io::Error>),

    #[error(transparent)]
    Libp2pNoiseError(#[from] libp2p::noise::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl From<Libp2pAdapterError> for TransportError {
    fn from(error: Libp2pAdapterError) -> Self {
        match error {
            Libp2pAdapterError::Base64(e) => TransportError::Io(e.to_string()),
            Libp2pAdapterError::Io(e) => TransportError::Io(e.to_string()),
            Libp2pAdapterError::Libp2pBehaviourBuilder(e) => {
                TransportError::Protocol(e.to_string())
            }
            Libp2pAdapterError::Libp2pDecodingError(e) => TransportError::Io(e.to_string()),
            Libp2pAdapterError::Libp2pTransportError(e) => TransportError::Protocol(e.to_string()),
            Libp2pAdapterError::Libp2pNoiseError(e) => TransportError::Protocol(e.to_string()),
        }
    }
}
