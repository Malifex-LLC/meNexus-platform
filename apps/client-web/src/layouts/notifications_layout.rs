// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::notifications::{
    AnalyticsPanel, BroadcastsPanel, ConnectionsPanel, ContentPanel, QuickActionsPanel,
    StatusOverview, SystemPanel,
};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn NotificationsLayout() -> impl IntoView {
    view! {
        <MainLayout>
            <div class="h-full w-full overflow-y-auto scrollbar-styled bg-background">
                // Compact header
                <header class="sticky top-0 z-10 bg-background/95 backdrop-blur-lg border-b border-border/50">
                    <div class="max-w-[1800px] mx-auto px-4 sm:px-6 lg:px-8 py-4">
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center">
                                    <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                                    </svg>
                                </div>
                                <div>
                                    <h1 class="text-xl font-bold text-foreground">"Notifications"</h1>
                                    <p class="text-xs text-foreground/50">"Your activity center"</p>
                                </div>
                            </div>

                            // Header actions
                            <div class="flex items-center gap-2">
                                <button class="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm font-medium bg-brand text-white hover:bg-brand/90 transition-colors">
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Mark All Read"
                                </button>
                                <button class="p-2 rounded-lg text-foreground/60 hover:text-foreground hover:bg-foreground/5 transition-colors" title="Settings">
                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>
                </header>

                // Dashboard content
                <main class="max-w-[1800px] mx-auto px-4 sm:px-6 lg:px-8 py-6 pb-12">
                    // Status overview banner
                    <div class="mb-6">
                        <StatusOverview />
                    </div>

                    // Main dashboard grid
                    <div class="grid grid-cols-1 xl:grid-cols-12 gap-6">
                        // Left column - Notification feeds (takes most space)
                        <div class="xl:col-span-8 space-y-6">
                            // Top row - Broadcasts (full width, shorter)
                            <div class="h-[280px]">
                                <BroadcastsPanel />
                            </div>

                            // Middle row - Content and Connections side by side
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                <div class="h-[400px]">
                                    <ContentPanel />
                                </div>
                                <div class="h-[400px]">
                                    <ConnectionsPanel />
                                </div>
                            </div>

                            // Bottom row - System notifications
                            <div class="h-[250px]">
                                <SystemPanel />
                            </div>
                        </div>

                        // Right column - Analytics and Actions
                        <div class="xl:col-span-4 space-y-6">
                            // Analytics panel
                            <AnalyticsPanel />

                            // Quick actions panel
                            <QuickActionsPanel />
                        </div>
                    </div>
                </main>
            </div>
        </MainLayout>
    }
}
