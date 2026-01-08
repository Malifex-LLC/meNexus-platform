// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::main_layout::MainLayout;
use crate::layouts::three_column_module_layout::ThreeColumnModuleLayout;
use crate::layouts::two_column_module_layout::TwoColumnModuleLayout;
use leptos::prelude::*;

/// Synapse Layout
/// 
/// This layout wraps synapse content in the `.synapse-themed` class, which applies
/// the synapse-specific theme. This allows each Synapse to have its own unique
/// look and feel, separate from the user's personal app theme.
/// 
/// The synapse theme is controlled by the Synapse host and affects:
/// - Module layouts (two_column, three_column)
/// - All modules (posts, chat, activity, members, etc.)
/// 
/// The synapse theme is loaded dynamically based on the current Synapse's configuration.
#[component]
pub fn SynapseLayout() -> impl IntoView {
    // TODO: In production, the synapse theme class would be dynamically set based on
    // the current Synapse's theme configuration fetched from the server.
    // For now, we use a static class that applies the default synapse theme.
    
    view! {
        <MainLayout>
            // The synapse-themed wrapper applies synapse-specific CSS variables
            // that override the user theme for content within this container
            <div class="synapse-themed h-full w-full">
                // <ThreeColumnModuleLayout/>
                <TwoColumnModuleLayout/>
            </div>
        </MainLayout>
    }
}
