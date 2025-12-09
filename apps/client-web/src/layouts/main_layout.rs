// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::control_panel::ControlPanel;
use leptos::prelude::*;

/// Main responsive layout with collapsible sidebar
#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    // Mobile sidebar open state
    let (sidebar_open, set_sidebar_open) = signal(false);
    // Desktop sidebar collapsed state
    let (sidebar_collapsed, set_sidebar_collapsed) = signal(false);

    view! {
        <div class="flex h-screen w-screen overflow-hidden bg-background">
            // Mobile sidebar overlay
            {move || {
                if sidebar_open.get() {
                    view! {
                        <div 
                            class="fixed inset-0 bg-black/50 z-40 lg:hidden"
                            on:click=move |_| set_sidebar_open.set(false)
                        ></div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Sidebar container - this div controls the space allocation
            // When collapsed on desktop, this has w-0 so content takes full width
            <div class=move || format!(
                "flex-shrink-0 transition-all duration-300 ease-in-out {} {}",
                // Normal width on desktop when expanded
                if sidebar_collapsed.get() { "lg:w-0" } else { "lg:w-56 xl:w-64" },
                // Always w-0 on mobile (sidebar is fixed/overlay)
                "w-0"
            )>
                // The actual sidebar - always fixed position, slides in/out
                <aside class=move || format!(
                    "fixed inset-y-0 left-0 z-50 w-64 lg:w-56 xl:w-64 transform transition-transform duration-300 ease-in-out {}",
                    // Mobile: controlled by sidebar_open
                    // Desktop: controlled by sidebar_collapsed
                    if sidebar_open.get() { 
                        "translate-x-0" 
                    } else if sidebar_collapsed.get() { 
                        "-translate-x-full" 
                    } else { 
                        "-translate-x-full lg:translate-x-0" 
                    }
                )>
                    <ControlPanel 
                        on_close=move || set_sidebar_open.set(false) 
                        on_collapse=move || set_sidebar_collapsed.set(true)
                    />
                </aside>
            </div>

            // Floating expand button (visible when sidebar is collapsed on desktop)
            {move || {
                if sidebar_collapsed.get() {
                    view! {
                        <button
                            class="hidden lg:flex fixed left-3 top-3 z-40 items-center gap-2 px-3 py-2 bg-panel/95 backdrop-blur-sm border border-border/50 rounded-xl shadow-lg hover:bg-card hover:border-brand/30 transition-all duration-200 group"
                            on:click=move |_| set_sidebar_collapsed.set(false)
                            title="Expand Control Panel"
                        >
                            // Logo
                            <div class="w-7 h-7 rounded-lg bg-gradient-to-br from-brand to-brand/70 flex items-center justify-center shadow-md shadow-brand/20">
                                <svg class="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                </svg>
                            </div>
                            // Expand icon
                            <svg class="w-4 h-4 text-foreground/50 group-hover:text-brand transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 5l7 7-7 7M5 5l7 7-7 7"></path>
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Main content area - takes remaining space
            <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
                // Mobile header with hamburger menu
                <header class="flex-shrink-0 flex items-center gap-3 px-3 py-2 border-b border-border/50 bg-panel/50 lg:hidden">
                    <button 
                        class="p-2 rounded-lg text-foreground/60 hover:text-foreground hover:bg-foreground/5 transition-colors"
                        on:click=move |_| set_sidebar_open.set(true)
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </button>
                    <div class="flex items-center gap-2">
                        <div class="w-7 h-7 rounded-lg bg-gradient-to-br from-brand to-brand/70 flex items-center justify-center">
                            <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        </div>
                        <span class="font-bold text-foreground text-sm">"meNexus"</span>
                    </div>
                </header>

                // Main content - each page handles its own scrolling
                <main class="flex-1 w-full min-h-0 overflow-hidden">
                    {children()}
                </main>
            </div>
        </div>
    }
}
