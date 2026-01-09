// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Rendering Components
//!
//! This module contains components for rendering a Synapse based on its manifest.
//! The `SynapseRenderer` is the main entry point that fetches the manifest and
//! renders the appropriate layout with configured modules.
//!
//! For remote Synapses, use `RemoteSynapseRenderer` which fetches the manifest
//! via the `synapse:get_manifest` event.

mod synapse_renderer;
mod remote_synapse_renderer;
mod synapse_header;
mod dynamic_layout;
mod module_slot;

pub use synapse_renderer::SynapseRenderer;
pub use remote_synapse_renderer::{RemoteSynapseRenderer, RemoteSynapseContext};
pub use synapse_header::SynapseHeader;
pub use dynamic_layout::DynamicLayout;
pub use module_slot::ModuleSlot;
