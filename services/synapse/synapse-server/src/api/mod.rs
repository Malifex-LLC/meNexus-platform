// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use axum::Router;

use crate::state::AppState;

pub mod agents;
pub mod artifacts;
pub mod auth;
pub mod channels;
pub mod comments;
pub mod events;
pub mod federation;
pub mod health;
pub mod modules;
pub mod peers;
pub mod settings;
pub mod synapses;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(events::routes())
}
