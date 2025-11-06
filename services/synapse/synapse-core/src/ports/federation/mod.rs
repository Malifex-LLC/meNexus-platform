// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;

use crate::domain::federation::PeerInfo;
use crate::errors::CoreError;

// Outbound Port
#[async_trait]
pub trait SnpTransport: Send + Sync {}

// Inbound port
#[async_trait]
pub trait MessageHandler: Send + Sync {}
