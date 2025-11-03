// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;
use synapse_application::events::CreateEventUseCase;

#[derive(Clone)]
pub struct AppState {
    pub create_event: Arc<dyn CreateEventUseCase>,
}
