// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use std::sync::Arc;
use synapse_config::get_synapse_config;
use synapse_core::{
    CoreError,
    domain::events::Event,
    ports::{events::event_repository::EventRepository, modules::Module},
};

pub struct CoreModule {
    kind: String,
    version: String,
    repo: Arc<dyn EventRepository>,
}

impl CoreModule {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self {
            kind: "core".to_string(),
            version: "1.0.0".to_string(),
            repo,
        }
    }
}

#[async_trait]
impl Module for CoreModule {
    fn kind(&self) -> Result<String, CoreError> {
        Ok(self.kind.clone())
    }
    fn version(&self) -> Result<String, CoreError> {
        Ok(self.version.clone())
    }
    async fn handle_event(&self, event: &Event) -> Result<Option<Vec<Event>>, CoreError> {
        match event.event_type.as_str() {
            "synapse:get_public_key" => {
                let config = get_synapse_config().unwrap();
                let public_key = config.identity.public_key;
                let res_event = Event::new()
                    .with_event_type("synapse:return_public_key")
                    .with_module_kind("core")
                    .with_agent(public_key.clone())
                    .with_content(public_key.clone())
                    .build();
                let events = vec![res_event];
                Ok(Some(events))
            }
            "synapse:create_event" => {
                let config = get_synapse_config().unwrap();
                let public_key = config.identity.public_key;
                let res_event = Event::new()
                    .with_event_type("synapse:event_created")
                    .with_module_kind("core")
                    .with_agent(public_key.clone())
                    .with_content(public_key.clone())
                    .build();
                let events = vec![res_event];
                Ok(Some(events))
            }
            "synapse:list_all_events" => {
                let events = self.repo.retrieve("all".to_string()).await.unwrap();
                Ok(events)
            }

            _ => Ok(None),
        }
    }
}
