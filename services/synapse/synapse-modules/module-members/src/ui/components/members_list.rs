// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::Member;
use crate::ui::components::member_section::OnlineStatusSection;
use leptos::prelude::*;

/// Main members list component - displays all Synapse members organized by status and role
#[component]
pub fn MembersList() -> impl IntoView {
    // Get mock data
    let members = Member::mock_members();
    let members_for_stats = members.clone();
    
    let online_by_role = Member::get_online_by_role(&members);
    let offline_by_role = Member::get_offline_by_role(&members);
    
    let total_count = members.len();
    let online_count = members_for_stats.iter().filter(|m| m.status.is_online()).count();
    let streaming_count = members_for_stats.iter().filter(|m| m.is_streaming).count();

    // Search state
    let (search_query, set_search_query) = signal(String::new());
    let (show_search, set_show_search) = signal(false);

    // View mode state
    let (compact_view, set_compact_view) = signal(false);

    view! {
        <div class="flex flex-col h-full bg-panel border border-border/30 rounded-xl overflow-hidden">
            // Header
            <header class="flex-shrink-0 px-4 py-3 border-b border-border/30 bg-panel/50 backdrop-blur-sm">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        // Icon
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-brand to-brand/60 flex items-center justify-center">
                            <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"/>
                            </svg>
                        </div>
                        <div>
                            <h2 class="text-foreground font-semibold tracking-tight">"Members"</h2>
                            <p class="text-foreground/40 text-xs">"People in this Synapse"</p>
                        </div>
                    </div>

                    // Stats badges
                    <div class="flex items-center gap-2">
                        // Streaming badge
                        {if streaming_count > 0 {
                            view! {
                                <span class="flex items-center gap-1.5 px-2 py-1 rounded-full bg-rose-500/15 text-rose-400 text-xs font-medium">
                                    <span class="w-1.5 h-1.5 rounded-full bg-rose-400 animate-pulse"></span>
                                    {streaming_count}" live"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        
                        // Online count badge
                        <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-emerald-500/15 text-emerald-400 text-xs font-medium">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            {online_count}" online"
                        </span>
                    </div>
                </div>
            </header>

            // Toolbar
            <div class="flex-shrink-0 px-4 py-2 border-b border-border/20 bg-background/30">
                <div class="flex items-center gap-2">
                    // Search toggle & input
                    <div class="flex-1 relative">
                        <div class=move || format!(
                            "flex items-center transition-all duration-200 {}",
                            if show_search.get() { "w-full" } else { "w-auto" }
                        )>
                            <button
                                class=move || format!(
                                    "p-1.5 rounded-lg transition-colors {}",
                                    if show_search.get() { 
                                        "text-brand bg-brand/10" 
                                    } else { 
                                        "text-foreground/50 hover:text-foreground hover:bg-foreground/5" 
                                    }
                                )
                                on:click=move |_| {
                                    set_show_search.update(|v| *v = !*v);
                                    if !show_search.get() {
                                        set_search_query.set(String::new());
                                    }
                                }
                            >
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"/>
                                </svg>
                            </button>
                            
                            {move || {
                                if show_search.get() {
                                    view! {
                                        <input
                                            type="text"
                                            placeholder="Search members..."
                                            class="flex-1 ml-2 px-2 py-1 bg-background/50 border border-border/50 rounded-lg text-sm text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 transition-all"
                                            prop:value=search_query
                                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                        />
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    </div>

                    // View mode toggle
                    <div class="flex items-center gap-0.5 p-0.5 rounded-lg bg-foreground/5 border border-border/30">
                        <button
                            class=move || format!(
                                "p-1.5 rounded transition-colors {}",
                                if !compact_view.get() { 
                                    "bg-brand/15 text-brand" 
                                } else { 
                                    "text-foreground/40 hover:text-foreground/60" 
                                }
                            )
                            on:click=move |_| set_compact_view.set(false)
                            title="Detailed view"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                            </svg>
                        </button>
                        <button
                            class=move || format!(
                                "p-1.5 rounded transition-colors {}",
                                if compact_view.get() { 
                                    "bg-brand/15 text-brand" 
                                } else { 
                                    "text-foreground/40 hover:text-foreground/60" 
                                }
                            )
                            on:click=move |_| set_compact_view.set(true)
                            title="Compact view"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
                            </svg>
                        </button>
                    </div>

                    // Sort/Filter dropdown trigger (placeholder)
                    <button class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"/>
                        </svg>
                    </button>
                </div>
            </div>

            // Members list
            <div class="flex-1 overflow-y-auto scrollbar-styled">
                <div class="p-4">
                    {move || {
                        let is_compact = compact_view.get();
                        let online = online_by_role.clone();
                        let offline = offline_by_role.clone();
                        view! {
                            // Online members section
                            <OnlineStatusSection
                                title="Online".to_string()
                                members_by_role=online
                                is_online=true
                                compact=is_compact
                            />

                            // Offline members section
                            <OnlineStatusSection
                                title="Offline".to_string()
                                members_by_role=offline
                                is_online=false
                                compact=is_compact
                                default_collapsed=true
                            />
                        }
                    }}
                </div>
            </div>

            // Footer with stats
            <footer class="flex-shrink-0 px-4 py-2 border-t border-border/20 bg-panel/30">
                <div class="flex items-center justify-between text-xs">
                    <div class="flex items-center gap-3 text-foreground/40">
                        <span class="flex items-center gap-1.5 font-mono">
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"/>
                            </svg>
                            {total_count}" total"
                        </span>
                        <span class="text-foreground/20">"|"</span>
                        <span class="flex items-center gap-1.5">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                            {online_count}" active"
                        </span>
                    </div>
                    
                    <button class="text-foreground/40 hover:text-brand transition-colors font-medium">
                        "Invite members"
                    </button>
                </div>
            </footer>
        </div>
    }
}

/// Compact sidebar version of the members list
#[component]
pub fn MembersSidebar() -> impl IntoView {
    let members = Member::mock_members();
    let online_by_role = Member::get_online_by_role(&members);
    let offline_by_role = Member::get_offline_by_role(&members);
    
    let online_count = members.iter().filter(|m| m.status.is_online()).count();
    let total_count = members.len();

    view! {
        <div class="flex flex-col h-full bg-panel border-l border-border/30 overflow-hidden">
            // Header
            <header class="flex-shrink-0 px-3 py-2 border-b border-border/30 bg-panel/50">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-2">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"/>
                        </svg>
                        <span class="font-semibold text-sm text-foreground">"Members"</span>
                    </div>
                    <span class="text-xs text-foreground/40 font-mono">{online_count}"/"{ total_count }</span>
                </div>
            </header>

            // Search
            <div class="flex-shrink-0 px-2 py-2 border-b border-border/20">
                <div class="relative">
                    <svg class="absolute left-2 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"/>
                    </svg>
                    <input
                        type="text"
                        placeholder="Search..."
                        class="w-full pl-7 pr-2 py-1.5 bg-background/50 border border-border/50 rounded-lg text-xs text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 transition-all"
                    />
                </div>
            </div>

            // Members list
            <div class="flex-1 overflow-y-auto scrollbar-thin">
                <div class="p-2">
                    // Online section
                    <OnlineStatusSection
                        title="Online".to_string()
                        members_by_role=online_by_role
                        is_online=true
                        compact=true
                    />

                    // Offline section
                    <OnlineStatusSection
                        title="Offline".to_string()
                        members_by_role=offline_by_role
                        is_online=false
                        compact=true
                        default_collapsed=true
                    />
                </div>
            </div>
        </div>
    }
}
