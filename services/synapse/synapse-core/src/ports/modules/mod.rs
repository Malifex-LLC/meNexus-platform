// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::{CoreError, domain::events::Event};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Module: Send + Sync {
    fn kind(&self) -> Result<String, CoreError>;
    fn version(&self) -> Result<String, CoreError>;
    async fn handle_event(&self, event: &Event) -> Result<Vec<Event>, CoreError>;
}

#[async_trait]
pub trait ModuleRegistry: Send + Sync {
    fn register(&self, module: Arc<dyn Module>) -> Result<(), CoreError>;
    fn get(&self, kind: &str) -> Option<Arc<dyn Module>>;
    fn installed_kinds(&self) -> Vec<String>;
}
