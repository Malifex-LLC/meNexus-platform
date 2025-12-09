// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::dashboard::{GlobalFeed, NetworkStatsWidget, TrendingSidebar};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn DashboardLayout() -> impl IntoView {
    view! {
        <MainLayout>
            <div class="h-full w-full flex overflow-hidden">
                // Main content area with feed
                <main class="flex-1 overflow-y-auto scrollbar-styled bg-panel/50">
                    <div class="max-w-2xl mx-auto p-4 lg:p-6">
                        // Dashboard header
                        <header class="mb-6">
                            <div class="flex items-center justify-between mb-2">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-brand to-brand/60 flex items-center justify-center shadow-lg shadow-brand/20">
                                        <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h1 class="text-xl font-bold text-foreground">"Dashboard"</h1>
                                        <p class="text-sm text-foreground/50">"Your global network feed"</p>
                                    </div>
                                </div>

                                // Quick stats
                                <div class="hidden md:flex items-center gap-4 text-sm">
                                    <div class="flex items-center gap-1.5 text-status-online">
                                        <span class="w-2 h-2 rounded-full bg-status-online animate-pulse"></span>
                                        <span class="font-medium">"491"</span>
                                        <span class="text-foreground/40">"online"</span>
                                    </div>
                                    <div class="w-px h-4 bg-border/50"></div>
                                    <div class="flex items-center gap-1.5">
                                        <svg class="w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                        </svg>
                                        <span class="font-medium text-foreground">"4"</span>
                                        <span class="text-foreground/40">"synapses"</span>
                                    </div>
                                </div>
                            </div>

                            // Breadcrumb / context bar
                            <div class="flex items-center gap-2 text-xs text-foreground/40 mt-3">
                                <span class="flex items-center gap-1.5">
                                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    "Aggregating from all joined Synapses"
                                </span>
                                <span>"•"</span>
                                <span class="text-brand font-mono">"P2P Connected"</span>
                            </div>
                        </header>

                        // Global Feed
                        <GlobalFeed />
                    </div>
                </main>

                // Right sidebar - trending, network stats
                <aside class="hidden xl:block w-80 flex-shrink-0 border-l border-border/30 overflow-y-auto scrollbar-thin bg-panel/50">
                    <div class="p-4 space-y-4">
                        // Network stats widget
                        <NetworkStatsWidget />

                        // Trending sidebar
                        <TrendingSidebar />
                    </div>
                </aside>
            </div>
        </MainLayout>
    }
}
