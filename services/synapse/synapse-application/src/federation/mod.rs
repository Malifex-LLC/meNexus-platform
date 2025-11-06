// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod federation_service;

use async_trait::async_trait;
use std::sync::Arc;
use synapse_core::ports::federation::{MessageHandler, SnpTransport};

// Application service
pub struct SnpMessenger<T: SnpTransport> {
    transport: Arc<T>,
}

impl<T: SnpTransport> SnpMessenger<T> {}
