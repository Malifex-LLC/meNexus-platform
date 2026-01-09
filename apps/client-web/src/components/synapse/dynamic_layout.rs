// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Dynamic Layout
//!
//! Renders the Synapse layout based on manifest configuration.

use leptos::prelude::*;
use module_core::{ClientManifest, LayoutTemplate, SlotConfig};

use super::module_slot::ModuleSlot;
use crate::components::{ModuleTab, TabbedModules, ModulePanel};

/// Dynamic layout renderer
#[component]
pub fn DynamicLayout(manifest: ClientManifest) -> impl IntoView {
    let slots = manifest.layout.get_slot_configs();
    let template = manifest.layout.template.clone();

    match template {
        LayoutTemplate::SingleColumn => {
            view! { <SingleColumnLayout slots=slots manifest=manifest/> }.into_any()
        }
        LayoutTemplate::TwoColumn => {
            view! { <TwoColumnLayout slots=slots manifest=manifest/> }.into_any()
        }
        LayoutTemplate::ThreeColumn => {
            view! { <ThreeColumnLayout slots=slots manifest=manifest/> }.into_any()
        }
        LayoutTemplate::Fullscreen => {
            view! { <FullscreenLayout slots=slots manifest=manifest/> }.into_any()
        }
    }
}

// =============================================================================
// SINGLE COLUMN
// =============================================================================

#[component]
fn SingleColumnLayout(slots: Vec<SlotConfig>, manifest: ClientManifest) -> impl IntoView {
    let slot = slots.into_iter().next().unwrap_or(SlotConfig {
        name: "main".to_string(),
        span: 12,
        modules: vec![],
    });

    view! {
        <div class="h-full w-full overflow-hidden">
            <SlotContent slot_config=slot manifest=manifest/>
        </div>
    }
}

// =============================================================================
// TWO COLUMN
// =============================================================================

#[component]
fn TwoColumnLayout(slots: Vec<SlotConfig>, manifest: ClientManifest) -> impl IntoView {
    let left = slots.iter().find(|s| s.name == "left").cloned().unwrap_or(SlotConfig {
        name: "left".to_string(), span: 8, modules: vec![]
    });
    let right = slots.iter().find(|s| s.name == "right").cloned().unwrap_or(SlotConfig {
        name: "right".to_string(), span: 4, modules: vec![]
    });

    let (active_column, set_active_column) = signal(0usize);

    // TwoColumn layout: 8 cols left, 4 cols right
    view! {
        <div class="flex flex-col h-full w-full overflow-hidden">
            <div class="flex-1 min-h-0 overflow-hidden relative">
                <div class="h-full w-full lg:grid lg:grid-cols-12">
                    // Left column (8/12)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-8 lg:block {}",
                        if active_column.get() == 0 { "block" } else { "hidden" }
                    )>
                        <SlotContent slot_config=left.clone() manifest=manifest.clone()/>
                    </div>

                    // Right column (4/12)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-4 lg:border-l lg:border-border/50 lg:block {}",
                        if active_column.get() == 1 { "block" } else { "hidden" }
                    )>
                        <SlotContent slot_config=right.clone() manifest=manifest.clone()/>
                    </div>
                </div>
            </div>

            <MobileColumnSwitcher
                slot_names=vec!["Feed".to_string(), "Social".to_string()]
                active=active_column
                set_active=set_active_column
            />
        </div>
    }
}

// =============================================================================
// THREE COLUMN
// =============================================================================

#[component]
fn ThreeColumnLayout(slots: Vec<SlotConfig>, manifest: ClientManifest) -> impl IntoView {
    let left = slots.iter().find(|s| s.name == "left").cloned().unwrap_or(SlotConfig {
        name: "left".to_string(), span: 3, modules: vec![]
    });
    let center = slots.iter().find(|s| s.name == "center").cloned().unwrap_or(SlotConfig {
        name: "center".to_string(), span: 6, modules: vec![]
    });
    let right = slots.iter().find(|s| s.name == "right").cloned().unwrap_or(SlotConfig {
        name: "right".to_string(), span: 3, modules: vec![]
    });

    let (active_column, set_active_column) = signal(1usize); // Center by default

    // ThreeColumn layout: 3 cols left, 6 cols center, 3 cols right
    view! {
        <div class="flex flex-col h-full w-full overflow-hidden">
            <div class="flex-1 min-h-0 overflow-hidden relative">
                <div class="h-full w-full lg:grid lg:grid-cols-12">
                    // Left column (3/12)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-3 lg:border-r lg:border-border/50 lg:block {}",
                        if active_column.get() == 0 { "block" } else { "hidden" }
                    )>
                        <SlotContent slot_config=left.clone() manifest=manifest.clone()/>
                    </div>

                    // Center column (6/12)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-6 lg:block {}",
                        if active_column.get() == 1 { "block" } else { "hidden" }
                    )>
                        <SlotContent slot_config=center.clone() manifest=manifest.clone()/>
                    </div>

                    // Right column (3/12)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-3 lg:border-l lg:border-border/50 lg:block {}",
                        if active_column.get() == 2 { "block" } else { "hidden" }
                    )>
                        <SlotContent slot_config=right.clone() manifest=manifest.clone()/>
                    </div>
                </div>
            </div>

            <MobileColumnSwitcher
                slot_names=vec!["Left".to_string(), "Main".to_string(), "Right".to_string()]
                active=active_column
                set_active=set_active_column
            />
        </div>
    }
}

// =============================================================================
// FULLSCREEN
// =============================================================================

#[component]
fn FullscreenLayout(slots: Vec<SlotConfig>, manifest: ClientManifest) -> impl IntoView {
    let slot = slots.into_iter().next().unwrap_or(SlotConfig {
        name: "main".to_string(),
        span: 12,
        modules: vec![],
    });

    // Fullscreen shows first module only, no tabs
    let first_module = slot.modules.first().cloned();

    view! {
        <div class="h-full w-full overflow-hidden">
            {if let Some(module_id) = first_module {
                view! {
                    <ModuleSlot module_id=module_id manifest=manifest/>
                }.into_any()
            } else {
                view! {
                    <div class="h-full w-full flex items-center justify-center text-foreground/50">
                        "No modules configured"
                    </div>
                }.into_any()
            }}
        </div>
    }
}

// =============================================================================
// SLOT CONTENT
// =============================================================================

/// Renders content for a single slot (with tabs if multiple modules)
#[component]
fn SlotContent(slot_config: SlotConfig, manifest: ClientManifest) -> impl IntoView {
    let modules = slot_config.modules.clone();

    // No modules
    if modules.is_empty() {
        return view! {
            <div class="h-full w-full flex items-center justify-center bg-panel/30">
                <p class="text-foreground/40 text-sm">"No modules assigned"</p>
            </div>
        }.into_any();
    }

    // Single module - render directly
    if modules.len() == 1 {
        let module_id = modules.first().unwrap().clone();
        return view! {
            <div class="h-full w-full overflow-hidden">
                <ModuleSlot module_id=module_id manifest=manifest/>
            </div>
        }.into_any();
    }

    // Multiple modules - use tabbed interface
    let tabs: Vec<ModuleTab> = modules
        .iter()
        .map(|id| ModuleTab::new(id.clone(), get_module_name(id), get_module_icon(id)))
        .collect();

    let default_active = modules.first().cloned().unwrap_or_default();

    view! {
        <TabbedModules tabs=tabs default_active=default_active>
            {modules.into_iter().map(|module_id| {
                let id = module_id.clone();
                let manifest_clone = manifest.clone();
                view! {
                    <ModulePanel id=id.clone()>
                        <ModuleSlot module_id=id manifest=manifest_clone/>
                    </ModulePanel>
                }
            }).collect_view()}
        </TabbedModules>
    }.into_any()
}

// =============================================================================
// MOBILE COLUMN SWITCHER
// =============================================================================

#[component]
fn MobileColumnSwitcher(
    slot_names: Vec<String>,
    active: ReadSignal<usize>,
    set_active: WriteSignal<usize>,
) -> impl IntoView {
    if slot_names.len() <= 1 {
        return view! { <div></div> }.into_any();
    }

    view! {
        <div class="flex-shrink-0 lg:hidden border-t border-border/50 bg-panel/80 backdrop-blur-sm safe-area-bottom">
            <div class="flex items-center justify-center gap-1 p-2">
                {slot_names.into_iter().enumerate().map(|(i, name)| {
                    view! {
                        <button
                            class=move || format!(
                                "flex-1 flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl font-medium text-sm transition-all {}",
                                if active.get() == i {
                                    "bg-brand text-white shadow-lg shadow-brand/20"
                                } else {
                                    "bg-foreground/5 text-foreground/60 hover:text-foreground hover:bg-foreground/10"
                                }
                            )
                            on:click=move |_| set_active.set(i)
                        >
                            {name.clone()}
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }.into_any()
}

// =============================================================================
// HELPERS
// =============================================================================

fn get_module_name(module_id: &str) -> &'static str {
    match module_id {
        "posts" => "Posts",
        "chat" => "Chat",
        "activity" => "Activity",
        "members" => "Members",
        "livestream" => "Live",
        "creator" => "Creator",
        "files" => "Files",
        "media" => "Media",
        "calendar" => "Calendar",
        _ => "Module",
    }
}

fn get_module_icon(module_id: &str) -> &'static str {
    match module_id {
        "posts" => "M7 20l4-16m2 16l4-16M6 9h14M4 15h14",
        "chat" => "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
        "activity" => "M13 10V3L4 14h7v7l9-11h-7z",
        "members" => "M17 20a4 4 0 00-8 0m10-7a3 3 0 11-6 0 3 3 0 016 0zm-8 0a3 3 0 11-6 0 3 3 0 016 0zm-4 7a4 4 0 118 0m6 0a6 6 0 00-12 0",
        "livestream" => "M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z",
        "creator" => "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z",
        "files" => "M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z",
        "media" => "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
        "calendar" => "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
        _ => "M4 6h16M4 12h16M4 18h16",
    }
}
