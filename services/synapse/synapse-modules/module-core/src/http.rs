// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Core Module HTTP Layer
//!
//! Axum HTTP handlers and routes for core Synapse operations.

use async_trait::async_trait;
use axum::{
    Json,
    extract::Path,
    http::StatusCode,
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;
use synapse_config::{get_synapse_config, get_synapse_manifest};
use synapse_core::{
    CoreError,
    domain::events::Event,
    ports::{
        events::event_repository::{EventFilter, EventRepository},
        modules::Module,
    },
};
use tracing::debug;

use crate::service;
use crate::types::{ClientManifest, CoreDeps};

// =============================================================================
// ROUTES
// =============================================================================

pub fn routes<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
    CoreDeps: axum::extract::FromRef<S>,
{
    axum::Router::new()
        .route("/core/manifest", get(get_local_manifest_http))
        .route(
            "/synapses/{synapse_public_key}/core/manifest",
            get(get_remote_manifest_http),
        )
}

// =============================================================================
// HTTP HANDLERS
// =============================================================================

#[derive(Serialize)]
struct ManifestResponse {
    manifest: ClientManifest,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// GET /api/core/manifest - Get the local Synapse manifest
async fn get_local_manifest_http() -> Result<(StatusCode, Json<ManifestResponse>), (StatusCode, Json<ErrorResponse>)> {
    match service::get_local_manifest() {
        Ok(manifest) => Ok((StatusCode::OK, Json(ManifestResponse { manifest }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() }),
        )),
    }
}

/// GET /api/synapses/{synapse_public_key}/core/manifest - Get a remote Synapse's manifest
async fn get_remote_manifest_http(
    axum::extract::State(deps): axum::extract::State<CoreDeps>,
    Path(synapse_public_key): Path<String>,
) -> Result<(StatusCode, Json<ManifestResponse>), (StatusCode, Json<ErrorResponse>)> {
    match service::get_remote_manifest(&deps, synapse_public_key).await {
        Ok(manifest) => Ok((StatusCode::OK, Json(ManifestResponse { manifest }))),
        Err(e) => Err((
            StatusCode::BAD_GATEWAY,
            Json(ErrorResponse { error: e.to_string() }),
        )),
    }
}

// =============================================================================
// MODULE IMPLEMENTATION (Event Handling)
// =============================================================================

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
    async fn handle_event(&self, event: &Event) -> Result<Vec<Event>, CoreError> {
        match event.event_type.as_str() {
            // Return this Synapse's public key
            "synapse:get_public_key" => {
                let config = get_synapse_config().unwrap();
                let public_key = config.identity.public_key;
                let res_event = Event::new()
                    .with_event_type("synapse:return_public_key")
                    .with_module_kind("core")
                    .with_agent(public_key.clone())
                    .with_content(public_key.clone())
                    .build();
                Ok(vec![res_event])
            }

            // Return this Synapse's manifest for rendering
            "synapse:get_manifest" => {
                debug!("Handling synapse:get_manifest event");
                let manifest = get_synapse_manifest()
                    .map_err(|e| CoreError::config(format!("Failed to get manifest: {}", e)))?;
                
                // Serialize manifest to JSON and return in content field
                let manifest_json = serde_json::to_string(&manifest)
                    .map_err(|e| CoreError::config(format!("Failed to serialize manifest: {}", e)))?;
                
                let res_event = Event::new()
                    .with_event_type("synapse:manifest")
                    .with_module_kind("core")
                    .with_agent(manifest.identity.public_key.clone())
                    .with_content(manifest_json)
                    .build();
                
                debug!("Returning manifest for Synapse: {}", manifest.identity.name);
                Ok(vec![res_event])
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
                Ok(vec![res_event])
            }

            "synapse:list_all_events" => {
                let events = self
                    .repo
                    .retrieve(EventFilter {
                        event_type: None,
                        module_kind: None,
                        module_slug: None,
                    })
                    .await
                    .unwrap();
                Ok(events)
            }

            _ => Ok(vec![])
        }
    }
}
