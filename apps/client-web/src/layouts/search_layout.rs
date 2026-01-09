// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::search::{
    BrowseByCategory, FeaturedSynapses, SearchBar, SearchCategory, SearchFilters, SearchResults,
    SearchScope, SearchSortBy, SearchTimeRange, SuggestedUsers, TrendingPosts, TrendingTags,
};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn SearchLayout() -> impl IntoView {
    // Search state
    let (query, set_query) = signal(String::new());
    let (has_searched, set_has_searched) = signal(false);
    let (scope, set_scope) = signal(SearchScope::Global);
    let (category, set_category) = signal(SearchCategory::All);
    let (sort_by, set_sort_by) = signal(SearchSortBy::Relevance);
    let (time_range, set_time_range) = signal(SearchTimeRange::AnyTime);
    let (only_verified, set_only_verified) = signal(false);

    // Create RwSignals for components that need them
    let query_signal = RwSignal::new(query.get_untracked());
    let scope_signal = RwSignal::new(scope.get_untracked());
    let category_signal = RwSignal::new(category.get_untracked());
    let sort_by_signal = RwSignal::new(sort_by.get_untracked());
    let time_range_signal = RwSignal::new(time_range.get_untracked());
    let only_verified_signal = RwSignal::new(only_verified.get_untracked());

    // Sync signals
    Effect::new(move |_| {
        set_query.set(query_signal.get());
    });
    Effect::new(move |_| {
        set_scope.set(scope_signal.get());
    });
    Effect::new(move |_| {
        set_category.set(category_signal.get());
    });
    Effect::new(move |_| {
        set_sort_by.set(sort_by_signal.get());
    });
    Effect::new(move |_| {
        set_time_range.set(time_range_signal.get());
    });
    Effect::new(move |_| {
        set_only_verified.set(only_verified_signal.get());
    });

    let on_search = Callback::new(move |q: String| {
        query_signal.set(q);
        set_has_searched.set(true);
    });

    view! {
        <MainLayout>
            <div class="h-full w-full overflow-y-auto scrollbar-styled bg-background">
                // Header section with search bar
                <header class="sticky top-0 z-40 bg-background/95 backdrop-blur-lg border-b border-border/50">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        // Page title and description
                        <div class="pt-6 pb-4">
                            <div class="flex items-center gap-3 mb-2">
                                <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center">
                                    <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h1 class="text-2xl font-bold text-foreground">"Discover"</h1>
                                    <p class="text-sm text-foreground/50">"Find users, synapses, posts, and trending content"</p>
                                </div>
                            </div>
                        </div>

                        // Search bar
                        <div class="pb-4">
                            <SearchBar
                                query=query_signal
                                scope=scope_signal
                                on_search=on_search
                            />
                        </div>

                        // Filters (only show when searching)
                        {move || {
                            if has_searched.get() && !query.get().is_empty() {
                                view! {
                                    <div class="pb-4">
                                        <SearchFilters
                                            active_category=category_signal
                                            sort_by=sort_by_signal
                                            time_range=time_range_signal
                                            only_verified=only_verified_signal
                                            result_count=15
                                        />
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </div>
                </header>

                // Main content
                <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6 pb-12">
                    {move || {
                        if has_searched.get() && !query.get().is_empty() {
                            // Show search results
                            let current_query = query.get();
                            let current_category = category.get();
                            view! {
                                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                                    // Main results
                                    <div class="lg:col-span-2">
                                        <SearchResults
                                            query=current_query
                                            category=current_category
                                        />
                                    </div>

                                    // Sidebar
                                    <aside class="space-y-4">
                                        <TrendingTags/>
                                        <SuggestedUsers/>
                                    </aside>
                                </div>
                            }.into_any()
                        } else {
                            // Show discovery content
                            view! {
                                <DiscoveryView/>
                            }.into_any()
                        }
                    }}
                </main>
            </div>
        </MainLayout>
    }
}

/// Discovery view when no search is active
#[component]
fn DiscoveryView() -> impl IntoView {
    view! {
        <div class="space-y-8">
            // Quick stats banner
            <div class="bg-brand/30 border border-border/50 rounded-2xl p-6">
                <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                    <div>
                        <h2 class="text-lg font-semibold text-foreground mb-1">"Explore the Network"</h2>
                        <p class="text-sm text-foreground/50">"Discover new synapses, connect with users, and find trending content."</p>
                    </div>
                    <div class="flex items-center gap-6 text-sm">
                        <div class="text-center">
                            <p class="text-2xl font-bold text-foreground">"12.4K"</p>
                            <p class="text-foreground/40">"Users"</p>
                        </div>
                        <div class="w-px h-10 bg-border/50"></div>
                        <div class="text-center">
                            <p class="text-2xl font-bold text-foreground">"847"</p>
                            <p class="text-foreground/40">"Synapses"</p>
                        </div>
                        <div class="w-px h-10 bg-border/50"></div>
                        <div class="text-center">
                            <p class="text-2xl font-bold text-status-online">"2.3K"</p>
                            <p class="text-foreground/40">"Online"</p>
                        </div>
                    </div>
                </div>
            </div>

            // Main discovery grid
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Left column - Featured content
                <div class="lg:col-span-2 space-y-6">
                    // Featured Synapses
                    <FeaturedSynapses/>

                    // Trending Posts
                    <TrendingPosts/>

                    // Browse by Category
                    <BrowseByCategory/>
                </div>

                // Right sidebar
                <aside class="space-y-4">
                    // Trending Tags
                    <TrendingTags/>

                    // Suggested Users
                    <SuggestedUsers/>

                    // Quick links
                    <div class="bg-card border border-border/50 rounded-2xl p-4">
                        <h3 class="font-semibold text-foreground mb-3">"Quick Links"</h3>
                        <div class="space-y-2">
                            <a href="/synapses/create" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 transition-colors text-sm">
                                <div class="w-8 h-8 rounded-lg bg-violet-500/15 flex items-center justify-center">
                                    <svg class="w-4 h-4 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="font-medium text-foreground">"Create a Synapse"</p>
                                    <p class="text-xs text-foreground/40">"Start your own community"</p>
                                </div>
                            </a>
                            <a href="/invite" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 transition-colors text-sm">
                                <div class="w-8 h-8 rounded-lg bg-emerald-500/15 flex items-center justify-center">
                                    <svg class="w-4 h-4 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="font-medium text-foreground">"Invite Friends"</p>
                                    <p class="text-xs text-foreground/40">"Grow the network"</p>
                                </div>
                            </a>
                            <a href="/settings/profile" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 transition-colors text-sm">
                                <div class="w-8 h-8 rounded-lg bg-brand/15 flex items-center justify-center">
                                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="font-medium text-foreground">"Complete Profile"</p>
                                    <p class="text-xs text-foreground/40">"Get discovered"</p>
                                </div>
                            </a>
                        </div>
                    </div>

                    // Network health indicator
                    <div class="bg-card border border-border/50 rounded-2xl p-4">
                        <div class="flex items-center gap-2 mb-3">
                            <span class="relative flex h-2.5 w-2.5">
                                <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                                <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-emerald-400"></span>
                            </span>
                            <span class="text-sm font-medium text-foreground">"Network Status"</span>
                        </div>
                        <div class="space-y-2 text-xs">
                            <div class="flex justify-between">
                                <span class="text-foreground/50">"Connected Peers"</span>
                                <span class="text-foreground font-mono">"24"</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-foreground/50">"Sync Latency"</span>
                                <span class="text-emerald-400 font-mono">"18ms"</span>
                            </div>
                            <div class="flex justify-between">
                                <span class="text-foreground/50">"Last Sync"</span>
                                <span class="text-foreground/70 font-mono">"2s ago"</span>
                            </div>
                        </div>
                    </div>
                </aside>
            </div>
        </div>
    }
}
