// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Core Module Service
//!
//! Business logic for core Synapse operations.

use crate::types::{ClientManifest, CoreDeps};
use synapse_application::events::{CreateEventCommand, CreateRemoteEventCommand};
use synapse_config::get_synapse_manifest;
use synapse_core::CoreError;
use tracing::debug;

/// Get the local Synapse manifest.
/// This returns the manifest for the current Synapse.
pub fn get_local_manifest() -> Result<ClientManifest, CoreError> {
    debug!("Fetching local Synapse manifest");

    let manifest = get_synapse_manifest()
        .map_err(|e| CoreError::config(format!("Failed to load manifest: {}", e)))?;

    debug!(
        "Loaded manifest for: {} with layout {:?}",
        manifest.identity.name, manifest.layout.template
    );
    debug!("Slots: {:?}", manifest.layout.slots);

    Ok(ClientManifest::from(manifest))
}

/// Get a remote Synapse's manifest by dispatching a synapse:get_manifest event.
pub async fn get_remote_manifest(
    deps: &CoreDeps,
    synapse_public_key: String,
) -> Result<ClientManifest, CoreError> {
    debug!("Fetching remote manifest for Synapse: {}", synapse_public_key);

    // Create the event command
    let inner = CreateEventCommand {
        event_type: "synapse:get_manifest".to_string(),
        module_kind: Some("core".to_string()),
        agent: "local-agent".to_string(),
        ..Default::default()
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key: synapse_public_key.clone(),
        event: inner,
    };

    // Dispatch the event to the remote Synapse
    let events = deps
        .create_remote_event
        .execute(cmd)
        .await
        .map_err(|e| CoreError::transport(format!("Failed to contact remote Synapse: {}", e)))?;

    debug!("Received {} events from remote Synapse", events.len());

    // Find the synapse:manifest event in the response
    let manifest_event = events
        .iter()
        .find(|e| e.event_type == "synapse:manifest")
        .ok_or_else(|| CoreError::transport("Remote Synapse did not return a manifest"))?;

    // Parse the manifest JSON from the event content
    let manifest_json = manifest_event
        .content
        .as_ref()
        .ok_or_else(|| CoreError::transport("Manifest event has no content"))?;

    let manifest: synapse_config::SynapseManifest = serde_json::from_str(manifest_json)
        .map_err(|e| CoreError::config(format!("Failed to parse manifest: {}", e)))?;

    debug!("Successfully fetched manifest for: {}", manifest.identity.name);

    Ok(ClientManifest::from(manifest))
}
