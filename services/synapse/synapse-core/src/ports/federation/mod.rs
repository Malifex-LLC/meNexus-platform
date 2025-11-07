// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod errors;

use async_trait::async_trait;

// Outbound Port
#[async_trait]
pub trait SnpTransport: Send + Sync {}

// Inbound port
#[async_trait]
pub trait MessageHandler: Send + Sync {}
