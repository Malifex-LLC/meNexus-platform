// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{format_count, DiscoveryPost, DiscoverySynapse, DiscoveryUser, TagResult};
use leptos::prelude::*;
use leptos_router::components::A;

/// Featured synapses discovery section
#[component]
pub fn FeaturedSynapses() -> impl IntoView {
    let synapses = DiscoverySynapse::mock_featured();

    view! {
        <section class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/>
                </svg>
                <h3 class="font-semibold text-foreground">"Featured Synapses"</h3>
            </div>
            <div class="divide-y divide-border/30">
                {synapses.into_iter().map(|synapse| {
                    let first_char = synapse.name.chars().next().unwrap_or('S').to_uppercase().to_string();
                    let synapse_id = synapse.id.clone();
                    let synapse_id_2 = synapse.id.clone();
                    let synapse_name = synapse.name.clone();
                    
                    view! {
                        <div class="p-4 hover:bg-foreground/[0.02] transition-colors">
                            <div class="flex gap-3">
                                // Icon
                                <A href=format!("/synapses/{}", synapse_id) attr:class="flex-shrink-0">
                                    {if let Some(url) = synapse.icon_url.clone() {
                                        view! {
                                            <img src=url alt="" class="w-12 h-12 rounded-xl object-cover"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-violet-500/30 to-violet-500/10 flex items-center justify-center">
                                                <span class="text-violet-400 font-bold text-lg">{first_char}</span>
                                            </div>
                                        }.into_any()
                                    }}
                                </A>

                                <div class="flex-1 min-w-0">
                                    // Name and verification
                                    <div class="flex items-center gap-2">
                                        <A
                                            href=format!("/synapses/{}", synapse_id_2)
                                            attr:class="font-semibold text-foreground hover:text-brand transition-colors truncate"
                                        >
                                            {synapse_name}
                                        </A>
                                        {if synapse.is_verified {
                                            view! {
                                                <svg class="w-4 h-4 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>

                                    // Featured reason
                                    <p class="text-xs text-brand/60 font-medium">{synapse.featured_reason}</p>

                                    // Description
                                    <p class="mt-1 text-sm text-foreground/50 line-clamp-2">{synapse.description}</p>

                                    // Stats
                                    <div class="mt-2 flex items-center gap-3 text-xs text-foreground/40">
                                        <span class="flex items-center gap-1">
                                            <span class="font-medium text-foreground">{format_count(synapse.member_count)}</span>
                                            " members"
                                        </span>
                                        <span class="flex items-center gap-1 text-emerald-400">
                                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                                            {format_count(synapse.online_count)}
                                        </span>
                                        <span class="text-emerald-400 font-medium">
                                            "+"{synapse.growth_percentage}"%"
                                        </span>
                                    </div>
                                </div>

                                // Join button
                                <button class=move || format!(
                                    "self-start px-3 py-1.5 rounded-lg text-xs font-medium transition-all {}",
                                    if synapse.is_joined {
                                        "bg-foreground/10 text-foreground"
                                    } else {
                                        "bg-brand/15 text-brand hover:bg-brand/25"
                                    }
                                )>
                                    {if synapse.is_joined { "Joined" } else { "Join" }}
                                </button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="px-4 py-2 border-t border-border/30 bg-foreground/[0.01]">
                <A href="/synapses/browse" attr:class="text-xs text-brand hover:underline">"Explore all synapses →"</A>
            </div>
        </section>
    }
}

/// Suggested users discovery section
#[component]
pub fn SuggestedUsers() -> impl IntoView {
    let users = DiscoveryUser::mock_suggested();

    view! {
        <section class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                </svg>
                <h3 class="font-semibold text-foreground">"People to Follow"</h3>
            </div>
            <div class="divide-y divide-border/30">
                {users.into_iter().map(|user| {
                    let initials: String = user.display_name
                        .split_whitespace()
                        .take(2)
                        .filter_map(|s| s.chars().next())
                        .collect::<String>()
                        .to_uppercase();
                    
                    let handle = user.handle.clone();
                    let handle_2 = user.handle.clone();
                    let handle_3 = user.handle.clone();
                    let display_name = user.display_name.clone();
                    
                    view! {
                        <div class="p-4 hover:bg-foreground/[0.02] transition-colors">
                            <div class="flex gap-3">
                                // Avatar
                                <A href=format!("/profiles/{}", handle) attr:class="flex-shrink-0">
                                    {if let Some(url) = user.avatar_url.clone() {
                                        view! {
                                            <img src=url alt="" class="w-11 h-11 rounded-full object-cover"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="w-11 h-11 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                                <span class="text-brand font-bold">{initials}</span>
                                            </div>
                                        }.into_any()
                                    }}
                                </A>

                                <div class="flex-1 min-w-0">
                                    // Name and verification
                                    <div class="flex items-center gap-1.5">
                                        <A
                                            href=format!("/profiles/{}", handle_2)
                                            attr:class="font-semibold text-sm text-foreground hover:text-brand transition-colors truncate"
                                        >
                                            {display_name}
                                        </A>
                                        {if user.is_verified {
                                            view! {
                                                <svg class="w-3.5 h-3.5 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        <span class="px-1.5 py-0.5 bg-amber-500/15 text-amber-400 text-[10px] font-bold rounded">
                                            {format_count(user.reputation)}
                                        </span>
                                    </div>

                                    // Handle
                                    <A
                                        href=format!("/profiles/{}", handle_3)
                                        attr:class="text-xs text-brand/60 font-mono"
                                    >
                                        "@"{user.handle}
                                    </A>

                                    // Reason and mutual
                                    <div class="mt-1 flex items-center gap-2 text-xs text-foreground/40">
                                        <span>{user.reason}</span>
                                        {if user.mutual_count > 0 {
                                            view! {
                                                <span class="text-brand">"• "{user.mutual_count}" mutual"</span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                </div>

                                // Follow button
                                <button class="self-start px-3 py-1.5 rounded-lg text-xs font-medium bg-brand/15 text-brand hover:bg-brand/25 transition-all">
                                    "Follow"
                                </button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="px-4 py-2 border-t border-border/30 bg-foreground/[0.01]">
                <A href="/discover/people" attr:class="text-xs text-brand hover:underline">"Discover more people →"</A>
            </div>
        </section>
    }
}

/// Trending posts discovery section
#[component]
pub fn TrendingPosts() -> impl IntoView {
    let posts = DiscoveryPost::mock_trending();

    view! {
        <section class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z"/>
                </svg>
                <h3 class="font-semibold text-foreground">"Hot Right Now"</h3>
            </div>
            <div class="divide-y divide-border/30">
                {posts.into_iter().map(|post| {
                    let initials: String = post.author_name
                        .split_whitespace()
                        .take(2)
                        .filter_map(|s| s.chars().next())
                        .collect::<String>()
                        .to_uppercase();
                    
                    let handle = post.author_handle.clone();
                    let author_name = post.author_name.clone();
                    
                    view! {
                        <div class="p-4 hover:bg-foreground/[0.02] transition-colors">
                            // Trending reason badge
                            <div class="flex items-center gap-2 mb-2">
                                <span class="text-xs font-medium text-brand">{post.trending_reason}</span>
                                <span class="text-xs text-foreground/30">"in "{post.synapse_name}</span>
                            </div>

                            <div class="flex gap-3">
                                // Avatar
                                <A href=format!("/profiles/{}", handle) attr:class="flex-shrink-0">
                                    {if let Some(url) = post.author_avatar.clone() {
                                        view! {
                                            <img src=url alt="" class="w-9 h-9 rounded-full object-cover"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="w-9 h-9 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                                <span class="text-brand font-bold text-xs">{initials}</span>
                                            </div>
                                        }.into_any()
                                    }}
                                </A>

                                <div class="flex-1 min-w-0">
                                    // Author
                                    <div class="flex items-center gap-1.5 text-sm">
                                        <span class="font-semibold text-foreground">{author_name}</span>
                                        {if post.author_verified {
                                            view! {
                                                <svg class="w-3.5 h-3.5 text-brand" viewBox="0 0 24 24" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                        <span class="text-foreground/40">"·"</span>
                                        <span class="text-foreground/40">{post.timestamp}</span>
                                    </div>

                                    // Content preview
                                    <p class="mt-1 text-sm text-foreground/70 line-clamp-2">{post.content_preview}</p>

                                    // Stats
                                    <div class="mt-2 flex items-center gap-4 text-xs text-foreground/40">
                                        <span class="flex items-center gap-1">
                                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                                            </svg>
                                            {format_count(post.likes)}
                                        </span>
                                        <span class="flex items-center gap-1">
                                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                                            </svg>
                                            {format_count(post.comments)}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <div class="px-4 py-2 border-t border-border/30 bg-foreground/[0.01]">
                <A href="/trending" attr:class="text-xs text-brand hover:underline">"See all trending posts →"</A>
            </div>
        </section>
    }
}

/// Trending tags compact widget
#[component]
pub fn TrendingTags() -> impl IntoView {
    let tags = TagResult::mock_results("trending");

    view! {
        <section class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"/>
                </svg>
                <h3 class="font-semibold text-foreground">"Trending Tags"</h3>
            </div>
            <div class="p-4">
                <div class="flex flex-wrap gap-2">
                    {tags.into_iter().take(8).map(|tag| {
                        let tag_name = tag.tag.clone();
                        let tag_name_2 = tag.tag.clone();
                        view! {
                            <A
                                href=format!("/search?tag={}", tag_name)
                                attr:class="group flex items-center gap-1.5 px-3 py-1.5 bg-foreground/5 hover:bg-brand/15 rounded-xl transition-colors"
                            >
                                <span class="text-sm font-medium text-foreground/70 group-hover:text-brand transition-colors">
                                    "#"{tag_name_2}
                                </span>
                                {if let Some(rank) = tag.trending_rank {
                                    if rank <= 5 {
                                        view! {
                                            <span class="flex items-center text-emerald-400">
                                                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18"/>
                                                </svg>
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </A>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}

/// Browse by category section
#[component]
pub fn BrowseByCategory() -> impl IntoView {
    let categories = DiscoverySynapse::mock_by_category();

    view! {
        <section class="space-y-6">
            <div class="flex items-center justify-between">
                <h2 class="text-lg font-semibold text-foreground">"Browse by Category"</h2>
                <A href="/categories" attr:class="text-sm text-brand hover:underline">"View all"</A>
            </div>
            
            <div class="space-y-6">
                {categories.into_iter().map(|(category, synapses)| {
                    let cat_name = category.clone();
                    view! {
                        <div class="space-y-3">
                            <h3 class="text-sm font-medium text-foreground/60 flex items-center gap-2">
                                <span class="w-8 h-0.5 bg-brand/50 rounded-full"></span>
                                {cat_name}
                            </h3>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                                {synapses.into_iter().map(|synapse| {
                                    let first_char = synapse.name.chars().next().unwrap_or('S').to_uppercase().to_string();
                                    let synapse_id = synapse.id.clone();
                                    let synapse_name = synapse.name.clone();
                                    
                                    view! {
                                        <A
                                            href=format!("/synapses/{}", synapse_id)
                                            attr:class="group flex gap-3 p-3 bg-card border border-border/50 rounded-xl hover:border-border hover:shadow-md transition-all"
                                        >
                                            <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-violet-500/30 to-violet-500/10 flex items-center justify-center flex-shrink-0">
                                                <span class="text-violet-400 font-bold">{first_char}</span>
                                            </div>
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center gap-1.5">
                                                    <span class="font-medium text-foreground group-hover:text-brand transition-colors truncate">
                                                        {synapse_name}
                                                    </span>
                                                    {if synapse.is_verified {
                                                        view! {
                                                            <svg class="w-3.5 h-3.5 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                                                <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                            </svg>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
                                                </div>
                                                <p class="text-xs text-foreground/50 line-clamp-1">{synapse.description}</p>
                                                <div class="mt-1 flex items-center gap-2 text-xs text-foreground/40">
                                                    <span>{format_count(synapse.member_count)}" members"</span>
                                                    <span class="text-emerald-400">"+"{synapse.growth_percentage}"%"</span>
                                                </div>
                                            </div>
                                        </A>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}
