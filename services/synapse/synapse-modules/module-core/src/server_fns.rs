// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Core Module Server Functions
//!
//! Leptos server functions for core Synapse operations.

use crate::types::ClientManifest;
use leptos::prelude::*;

/// Fetch the local Synapse manifest.
/// This returns the manifest for the current Synapse (not a remote one).
/// The manifest is regenerated from environment variables on each call to ensure
/// configuration changes are reflected immediately.
#[server(GetLocalManifest, "/api/synapse")]
pub async fn get_local_manifest() -> Result<ClientManifest, ServerFnError> {
    use crate::service;

    service::get_local_manifest()
        .map_err(|e| ServerFnError::new(format!("Failed to load manifest: {}", e)))
}

/// Fetch a remote Synapse's manifest by dispatching a synapse:get_manifest event.
/// This is used for Synapse-to-Synapse rendering.
#[server(GetRemoteManifest, "/api/synapse")]
pub async fn get_remote_manifest(synapse_public_key: String) -> Result<ClientManifest, ServerFnError> {
    use crate::service;
    use crate::types::CoreDeps;

    let deps: CoreDeps = expect_context();

    service::get_remote_manifest(&deps, synapse_public_key)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch remote manifest: {}", e)))
}
