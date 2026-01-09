// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Layout
//!
//! This layout renders the Synapse based on its manifest configuration.
//! The manifest is fetched from the server and used to dynamically render:
//! - Theme and visual appearance
//! - Layout structure (columns, templates)
//! - Module placement and configuration
//!
//! The synapse-themed wrapper applies synapse-specific CSS variables
//! that can be customized per-Synapse via the manifest.

use crate::components::synapse::SynapseRenderer;
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

/// Synapse Layout Component
///
/// Wraps the SynapseRenderer in the MainLayout to provide:
/// - Navigation (control panel)
/// - App-level context
/// - Synapse-specific theming
#[component]
pub fn SynapseLayout() -> impl IntoView {
    view! {
        <MainLayout>
            <SynapseRenderer/>
        </MainLayout>
    }
}
