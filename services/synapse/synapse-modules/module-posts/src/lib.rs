// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use synapse_core::{CoreError, domain::events::Event, ports::modules::Module};

// implement module port
pub struct PostsModule {
    kind: String,
    version: String,
}

impl PostsModule {
    pub fn new() -> Self {
        Self {
            kind: "module:posts".to_string(),
            version: "1.0.0".to_string(),
        }
    }
}

#[async_trait]
impl Module for PostsModule {
    fn kind(&self) -> Result<String, CoreError> {
        Ok(self.kind.clone())
    }
    fn version(&self) -> Result<String, CoreError> {
        Ok(self.version.clone())
    }
    async fn handle_event(&self, event: &Event) -> Result<Option<Vec<Event>>, CoreError> {
        match event.event_type.as_str() {
            "posts:create_post" => Ok(None),
            _ => Ok(None),
        }
    }
}
// register module function
