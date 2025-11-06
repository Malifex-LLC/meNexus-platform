// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod snp;

use crate::errors::CoreError;
use async_trait::async_trait;

// Add domain types
#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addresses: Vec<String>,
    pub is_connected: bool,
}

#[derive(Debug, Clone)]
pub enum TransportEvent {
    PeerConnected(PeerInfo),
    PeerDisconnected(String), // peer_id
    ListeningOn(String),      // address
    BootstrapCompleted,
}
