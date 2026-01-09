// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Module Core
//!
//! Core functionality shared across all Synapse modules.
//!
//! ## HTTP Routes
//!
//! - `GET /api/core/manifest` - Get the local Synapse manifest
//! - `GET /api/synapses/{synapse_public_key}/core/manifest` - Get a remote Synapse's manifest

#[cfg(feature = "ssr")]
pub mod http;

#[cfg(feature = "ssr")]
pub mod service;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub mod server_fns;

pub mod types;

// Re-export types
pub use types::{
    ClientManifest,
    ClientIdentity,
    ClientTheme,
    ClientLayout,
    ClientCapabilities,
    LayoutTemplate,
    SlotConfig,
    known_modules,
};

#[cfg(feature = "ssr")]
pub use types::CoreDeps;

#[cfg(feature = "ssr")]
pub use http::routes;

#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub use server_fns::{get_local_manifest, get_remote_manifest};
