// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod following_list;
pub mod identity_panel;
pub mod nav_bar;
pub mod notifications_dropdown;
pub mod synapses_list;
pub mod user_profile_card;

use crate::components::control_panel::following_list::FollowingList;
use crate::components::control_panel::identity_panel::IdentityPanel;
use crate::components::control_panel::nav_bar::NavBar;
use crate::components::control_panel::notifications_dropdown::NotificationsDropdown;
use crate::components::control_panel::synapses_list::SynapsesList;
use crate::components::control_panel::user_profile_card::UserProfileCard;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ControlPanel(
    /// Callback when close button is clicked (for mobile)
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col h-full bg-panel border-r border-border/50">
            // Logo/Brand header
            <div class="flex-shrink-0 p-3 border-b border-border/30">
                <div class="flex items-center justify-between">
                    <A href="/" attr:class="flex items-center gap-2 group">
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-brand to-brand/70 flex items-center justify-center shadow-lg shadow-brand/20">
                            <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        </div>
                        <div>
                            <h1 class="font-bold text-foreground text-sm tracking-tight">"meNexus"</h1>
                            <p class="text-[10px] text-foreground/40 font-mono">"v0.3.0-alpha"</p>
                        </div>
                    </A>
                    // Close button (mobile only)
                    {if let Some(close) = on_close {
                        view! {
                            <button
                                class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors lg:hidden"
                                on:click=move |_| close.run(())
                            >
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>

            // Scrollable content area
            <div class="flex-1 overflow-y-auto scrollbar-thin p-2 space-y-3">
                // User Profile Card
                // <UserProfileCard />
                <IdentityPanel />

                // Quick Actions Bar
                <div class="flex items-center gap-1.5">
                    <A href="/compose" attr:class="flex-1 flex items-center justify-center gap-1.5 py-2 bg-brand hover:bg-brand/90 text-white font-semibold text-sm rounded-lg transition-all active:scale-[0.98]">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"></path>
                        </svg>
                        <span class="hidden xl:inline">"New Post"</span>
                        <span class="xl:hidden">"Post"</span>
                    </A>
                    <NotificationsDropdown />
                </div>

                // Main Navigation
                <div>
                    <div class="flex items-center gap-2 px-1 mb-1">
                        <span class="text-[10px] font-medium text-foreground/40 uppercase tracking-wider">"Navigation"</span>
                        <span class="flex-1 h-px bg-border/30"></span>
                    </div>
                    <NavBar />
                </div>

                // Synapses List
                <SynapsesList />

                // Following List
                <FollowingList />
            </div>

            // Footer with network status
            <div class="flex-shrink-0 p-2 border-t border-border/30 bg-background/30">
                <div class="flex items-center justify-between text-[10px]">
                    <span class="flex items-center gap-1.5 text-emerald-400">
                        <span class="relative flex h-1.5 w-1.5">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-1.5 w-1.5 bg-emerald-500"></span>
                        </span>
                        "Online"
                    </span>
                    <div class="flex items-center gap-2 text-foreground/40">
                        <span class="flex items-center gap-1">
                            <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            </svg>
                            "12"
                        </span>
                        <span class="flex items-center gap-1">
                            <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                            "24ms"
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}
