// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::control_panel::ControlPanel;
use leptos::prelude::*;

/// Main responsive layout with collapsible sidebar
#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    let (sidebar_open, set_sidebar_open) = signal(false);

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

            // Sidebar - fixed position on mobile, relative on desktop
            <aside class=move || format!(
                "fixed lg:relative inset-y-0 left-0 z-50 w-64 lg:w-56 xl:w-64 flex-shrink-0 transform transition-transform duration-300 lg:translate-x-0 {}",
                if sidebar_open.get() { "translate-x-0" } else { "-translate-x-full" }
            )>
                <ControlPanel on_close=move || set_sidebar_open.set(false) />
            </aside>

            // Main content area
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
