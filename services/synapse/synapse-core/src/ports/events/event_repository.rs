// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::PersistenceError;
use crate::domain::events::Event;

#[async_trait::async_trait]
pub trait EventRepository: Send + Sync {
    async fn record(&self, event: Event) -> Result<Event, PersistenceError>;
    async fn retrieve(&self, filter: EventFilter) -> Result<Vec<Event>, PersistenceError>;
}

#[derive(Clone, Debug, Default)]
pub struct EventFilter {
    pub event_type: Option<String>,
    pub module_kind: Option<String>,
    pub module_slug: Option<String>,
}
