// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Renderer
//!
//! The main component that renders a Synapse based on its manifest.
//! It fetches the manifest, applies the theme, and renders the layout with modules.

use leptos::prelude::*;
use module_core::{ClientManifest, get_local_manifest};

use super::dynamic_layout::DynamicLayout;
use super::synapse_header::SynapseHeader;

/// Context type for accessing the manifest throughout the Synapse
pub type ManifestContext = Resource<Result<ClientManifest, ServerFnError>>;

/// The main Synapse renderer component.
///
/// This component:
/// 1. Fetches the Synapse manifest from the server
/// 2. Applies theme CSS variables based on manifest settings
/// 3. Renders the appropriate layout with configured modules
#[component]
pub fn SynapseRenderer() -> impl IntoView {
    // Fetch the manifest
    let manifest_resource = Resource::new(
        || (),
        |_| get_local_manifest(),
    );

    // Provide manifest as context for child components
    provide_context(manifest_resource);

    view! {
        <Suspense fallback=move || view! { <SynapseLoadingState/> }>
            {move || {
                manifest_resource.get().map(|result| {
                    match result {
                        Ok(manifest) => view! { <SynapseContent manifest=manifest/> }.into_any(),
                        Err(e) => view! { <SynapseErrorState error=e.to_string()/> }.into_any(),
                    }
                })
            }}
        </Suspense>
    }
}

/// The actual Synapse content once manifest is loaded
#[component]
fn SynapseContent(manifest: ClientManifest) -> impl IntoView {
    // Generate theme CSS variables from manifest
    let theme_style = generate_theme_style(&manifest);
    let theme_class = get_theme_class(&manifest);

    view! {
        // Apply synapse theme via inline styles for CSS variable overrides
        // and the theme class for preset themes
        <div
            class=format!("synapse-themed {} h-full w-full flex flex-col", theme_class)
            style=theme_style
        >
            // Optional header based on manifest
            <SynapseHeader manifest=manifest.clone()/>

            // Main content area with dynamic layout
            <div class="flex-1 min-h-0 overflow-hidden">
                <DynamicLayout manifest=manifest/>
            </div>
        </div>
    }
}

/// Generate inline CSS for theme variable overrides
fn generate_theme_style(manifest: &ClientManifest) -> String {
    let mut styles = Vec::new();

    // Apply accent color if specified
    if let Some(ref accent) = manifest.theme.accent_color {
        styles.push(format!("--synapse-accent: {}", accent));
        // Also set a hover variant (slightly lighter)
        styles.push(format!("--synapse-accent-hover: {}", lighten_color(accent)));
    }

    // Apply any custom CSS variables from manifest
    for (key, value) in &manifest.theme.variables {
        // Ensure the key has the proper prefix
        let var_name = if key.starts_with("--synapse-") {
            key.clone()
        } else if key.starts_with("--") {
            key.replace("--", "--synapse-")
        } else {
            format!("--synapse-{}", key)
        };
        styles.push(format!("{}: {}", var_name, value));
    }

    styles.join("; ")
}

/// Get the theme preset class name
fn get_theme_class(manifest: &ClientManifest) -> String {
    match manifest.theme.preset.as_deref() {
        Some("malifex") => "synapse-theme-malifex".to_string(),
        Some("meNexus_dark") | Some("menexus_dark") | Some("menexus-dark") => "synapse-theme-menexus-dark".to_string(),
        Some("midnight") => "synapse-theme-midnight".to_string(),
        Some("cyberpunk") => "synapse-theme-cyberpunk".to_string(),
        Some("nature") => "synapse-theme-nature".to_string(),
        Some("ocean") => "synapse-theme-ocean".to_string(),
        Some("light") => "synapse-theme-light".to_string(),
        Some("default") | None => "synapse-theme-default".to_string(),
        Some(custom) => format!("synapse-theme-{}", custom.to_lowercase().replace('_', "-")),
    }
}

/// Simple color lightening for hover states
fn lighten_color(hex: &str) -> String {
    // Very basic implementation - just return a slightly modified color
    // In production, you'd want proper color manipulation
    if hex.starts_with('#') && hex.len() == 7 {
        // Parse and lighten
        if let Ok(r) = u8::from_str_radix(&hex[1..3], 16) {
            if let Ok(g) = u8::from_str_radix(&hex[3..5], 16) {
                if let Ok(b) = u8::from_str_radix(&hex[5..7], 16) {
                    let lighten = |c: u8| -> u8 {
                        c.saturating_add(((255 - c) as f32 * 0.2) as u8)
                    };
                    return format!("#{:02x}{:02x}{:02x}", lighten(r), lighten(g), lighten(b));
                }
            }
        }
    }
    hex.to_string()
}

/// Loading state while fetching manifest
#[component]
fn SynapseLoadingState() -> impl IntoView {
    view! {
        <div class="h-full w-full flex items-center justify-center bg-panel">
            <div class="flex flex-col items-center gap-4">
                // Animated loading spinner
                <div class="w-12 h-12 border-4 border-foreground/20 border-t-brand rounded-full animate-spin"></div>
                <p class="text-foreground/60 text-sm">"Loading Synapse..."</p>
            </div>
        </div>
    }
}

/// Error state when manifest loading fails
#[component]
fn SynapseErrorState(error: String) -> impl IntoView {
    view! {
        <div class="h-full w-full flex items-center justify-center bg-panel">
            <div class="flex flex-col items-center gap-4 max-w-md text-center p-8">
                // Error icon
                <div class="w-16 h-16 rounded-full bg-error/20 flex items-center justify-center">
                    <svg class="w-8 h-8 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                </div>
                <h2 class="text-xl font-bold text-foreground">"Failed to Load Synapse"</h2>
                <p class="text-foreground/60 text-sm">{error}</p>
                <button
                    class="px-4 py-2 bg-brand text-white rounded-lg hover:bg-brand/80 transition-colors"
                    on:click=|_| {
                        // Reload the page to retry
                        let _ = window().location().reload();
                    }
                >
                    "Try Again"
                </button>
            </div>
        </div>
    }
}
