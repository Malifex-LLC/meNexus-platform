// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Header
//!
//! Renders the Synapse header with banner, title, and description based on manifest.

use leptos::prelude::*;
use module_core::ClientManifest;

/// Synapse header component showing banner, name, and description
#[component]
pub fn SynapseHeader(manifest: ClientManifest) -> impl IntoView {
    // If header is not configured to show, render nothing
    if !manifest.layout.show_header {
        return view! { <div></div> }.into_any();
    }

    let identity = manifest.identity;
    let has_banner = identity.banner_url.is_some();

    view! {
        <header class="flex-shrink-0 relative">
            // Banner image (if configured)
            {if has_banner {
                let banner_url = identity.banner_url.clone().unwrap_or_default();
                view! {
                    <div class="relative h-32 sm:h-48 w-full overflow-hidden">
                        <img
                            src=banner_url
                            alt="Synapse banner"
                            class="w-full h-full object-cover"
                        />
                        // Gradient overlay for text readability
                        <div class="absolute inset-0 bg-gradient-to-t from-background/90 to-transparent"></div>
                    </div>
                }.into_any()
            } else {
                view! { <div></div> }.into_any()
            }}

            // Title bar
            <div class=move || format!(
                "px-4 py-3 flex items-center gap-4 {}",
                if has_banner { "absolute bottom-0 left-0 right-0" } else { "bg-panel border-b border-border/50" }
            )>
                // Avatar
                {if let Some(avatar_url) = identity.avatar_url.clone() {
                    view! {
                        <img
                            src=avatar_url
                            alt="Synapse avatar"
                            class="w-12 h-12 rounded-xl object-cover border-2 border-background shadow-lg"
                        />
                    }.into_any()
                } else {
                    // Default avatar with first letter
                    let first_char = identity.name.chars().next().unwrap_or('S').to_uppercase().to_string();
                    view! {
                        <div class="w-12 h-12 rounded-xl bg-brand flex items-center justify-center text-white font-bold text-xl border-2 border-background shadow-lg">
                            {first_char}
                        </div>
                    }.into_any()
                }}

                // Name and description
                <div class="flex-1 min-w-0">
                    <h1 class="text-xl font-bold text-foreground truncate">
                        {identity.name.clone()}
                    </h1>

                    {if let Some(desc) = identity.description.clone() {
                        view! {
                            <p class="text-sm text-foreground/60 truncate">{desc}</p>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Tags (if any)
                {if !identity.tags.is_empty() {
                    view! {
                        <div class="hidden sm:flex items-center gap-2">
                            {identity.tags.iter().take(3).map(|tag| {
                                view! {
                                    <span class="px-2 py-1 text-xs rounded-full bg-brand/10 text-brand">
                                        {format!("#{}", tag)}
                                    </span>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
        </header>
    }.into_any()
}
