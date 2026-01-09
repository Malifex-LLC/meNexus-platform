// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{format_count, PostResult, SynapseResult, TagResult, UserResult};
use leptos::prelude::*;
use leptos_router::components::A;

/// User search result card
#[component]
pub fn UserResultCard(user: UserResult) -> impl IntoView {
    let initials: String = user
        .display_name
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
        <article class="group bg-card border border-border/50 rounded-2xl p-4 hover:border-border hover:shadow-lg transition-all duration-200">
            <div class="flex gap-4">
                // Avatar
                <A href=format!("/profiles/{}", handle) attr:class="flex-shrink-0">
                    {if let Some(url) = user.avatar_url.clone() {
                        view! {
                            <img src=url alt="" class="w-14 h-14 rounded-full object-cover ring-2 ring-border/30 group-hover:ring-brand/30 transition-all"/>
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-14 h-14 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30 group-hover:ring-brand/30 transition-all">
                                <span class="text-brand font-bold text-lg">{initials}</span>
                            </div>
                        }.into_any()
                    }}
                </A>

                // Info
                <div class="flex-1 min-w-0">
                    <div class="flex items-start justify-between gap-3">
                        <div class="min-w-0">
                            // Name and verification
                            <div class="flex items-center gap-1.5">
                                <A
                                    href=format!("/profiles/{}", handle_2)
                                    attr:class="font-semibold text-foreground hover:text-brand transition-colors truncate"
                                >
                                    {display_name}
                                </A>
                                {if user.is_verified {
                                    view! {
                                        <svg class="w-4 h-4 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                            <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                                // Reputation badge
                                <span class="px-1.5 py-0.5 bg-amber-500/15 text-amber-400 text-[10px] font-bold rounded">
                                    {format_count(user.reputation)}" REP"
                                </span>
                            </div>

                            // Handle
                            <A
                                href=format!("/profiles/{}", handle_3)
                                attr:class="text-sm text-brand/60 font-mono hover:text-brand transition-colors"
                            >
                                "@"{user.handle}
                            </A>
                        </div>

                        // Follow button
                        <button class=move || format!(
                            "px-4 py-1.5 rounded-xl text-sm font-medium transition-all {}",
                            if user.is_following {
                                "bg-foreground/10 text-foreground hover:bg-rose-500/15 hover:text-rose-400"
                            } else {
                                "bg-brand text-white hover:bg-brand/90 shadow-lg shadow-brand/20"
                            }
                        )>
                            {if user.is_following { "Following" } else { "Follow" }}
                        </button>
                    </div>

                    // Bio
                    {if let Some(bio) = user.bio {
                        view! {
                            <p class="mt-2 text-sm text-foreground/70 line-clamp-2">{bio}</p>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    // Stats
                    <div class="mt-3 flex items-center gap-4 text-xs text-foreground/50">
                        <span>
                            <span class="font-semibold text-foreground">{format_count(user.follower_count)}</span>
                            " followers"
                        </span>
                        <span>
                            <span class="font-semibold text-foreground">{format_count(user.following_count)}</span>
                            " following"
                        </span>
                        {if user.mutual_connections > 0 {
                            view! {
                                <span class="text-brand">
                                    <span class="font-semibold">{user.mutual_connections}</span>
                                    " mutual"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        <span class="ml-auto">"Joined "{user.joined_date}</span>
                    </div>
                </div>
            </div>
        </article>
    }
}

/// Synapse search result card
#[component]
pub fn SynapseResultCard(synapse: SynapseResult) -> impl IntoView {
    let first_char = synapse
        .name
        .chars()
        .next()
        .unwrap_or('S')
        .to_uppercase()
        .to_string();

    let synapse_id = synapse.id.clone();
    let synapse_id_2 = synapse.id.clone();
    let synapse_name = synapse.name.clone();

    view! {
        <article class="group bg-card border border-border/50 rounded-2xl overflow-hidden hover:border-border hover:shadow-lg transition-all duration-200">
            // Banner placeholder
            <div class="h-20 bg-gradient-to-br from-violet-500/20 via-brand/10 to-emerald-500/20 relative">
                <div class="absolute inset-0 bg-[url('data:image/svg+xml,...')] opacity-10"></div>
            </div>

            <div class="p-4 -mt-8 relative">
                // Icon
                <A href=format!("/synapses/{}", synapse_id) attr:class="block w-fit">
                    {if let Some(url) = synapse.icon_url.clone() {
                        view! {
                            <img src=url alt="" class="w-14 h-14 rounded-xl object-cover ring-4 ring-card"/>
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-14 h-14 rounded-xl bg-gradient-to-br from-violet-500/30 to-violet-500/10 flex items-center justify-center ring-4 ring-card">
                                <span class="text-violet-400 font-bold text-xl">{first_char}</span>
                            </div>
                        }.into_any()
                    }}
                </A>

                <div class="mt-3">
                    // Name and badges
                    <div class="flex items-center gap-2 flex-wrap">
                        <A
                            href=format!("/synapses/{}", synapse_id_2)
                            attr:class="font-semibold text-lg text-foreground hover:text-brand transition-colors"
                        >
                            {synapse_name}
                        </A>
                        {if synapse.is_verified {
                            view! {
                                <svg class="w-4 h-4 text-brand" viewBox="0 0 24 24" fill="currentColor">
                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        <span class="px-2 py-0.5 bg-foreground/5 text-foreground/50 text-xs rounded-lg">{synapse.category}</span>
                    </div>

                    // Description
                    <p class="mt-2 text-sm text-foreground/60 line-clamp-2">{synapse.description}</p>

                    // Tags
                    <div class="mt-3 flex flex-wrap gap-1.5">
                        {synapse.tags.iter().map(|tag| {
                            let t = tag.clone();
                            view! {
                                <span class="px-2 py-0.5 bg-brand/10 text-brand text-xs rounded-lg font-medium">
                                    "#"{t}
                                </span>
                            }
                        }).collect_view()}
                    </div>

                    // Stats and action
                    <div class="mt-4 flex items-center justify-between">
                        <div class="flex items-center gap-4 text-xs text-foreground/50">
                            <span class="flex items-center gap-1">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                                </svg>
                                <span class="font-semibold text-foreground">{format_count(synapse.member_count)}</span>
                                " members"
                            </span>
                            <span class="flex items-center gap-1">
                                <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                                <span class="text-emerald-400 font-medium">{format_count(synapse.online_count)}</span>
                                " online"
                            </span>
                        </div>

                        <button class=move || format!(
                            "px-4 py-1.5 rounded-xl text-sm font-medium transition-all {}",
                            if synapse.is_joined {
                                "bg-foreground/10 text-foreground hover:bg-rose-500/15 hover:text-rose-400"
                            } else {
                                "bg-brand text-white hover:bg-brand/90 shadow-lg shadow-brand/20"
                            }
                        )>
                            {if synapse.is_joined { "Joined" } else { "Join" }}
                        </button>
                    </div>
                </div>
            </div>
        </article>
    }
}

/// Post search result card
#[component]
pub fn PostResultCard(post: PostResult) -> impl IntoView {
    let initials: String = post
        .author_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    let handle = post.author_handle.clone();
    let handle_2 = post.author_handle.clone();
    let author_name = post.author_name.clone();

    view! {
        <article class="group bg-card border border-border/50 rounded-2xl p-4 hover:border-border hover:shadow-lg transition-all duration-200">
            // Source info
            <div class="flex items-center gap-2 text-xs text-foreground/40 mb-3">
                <A href=format!("/synapses/{}", post.synapse_id) attr:class="flex items-center gap-1.5 hover:text-foreground transition-colors">
                    <div class="w-4 h-4 rounded bg-violet-500/20 flex items-center justify-center">
                        <span class="text-violet-400 font-bold text-[8px]">
                            {post.synapse_name.chars().next().unwrap_or('S').to_uppercase().to_string()}
                        </span>
                    </div>
                    <span class="font-medium">{post.synapse_name}</span>
                </A>
                <span>"·"</span>
                <span>{post.timestamp}</span>
            </div>

            <div class="flex gap-3">
                // Avatar
                <A href=format!("/profiles/{}", handle) attr:class="flex-shrink-0">
                    {if let Some(url) = post.author_avatar.clone() {
                        view! {
                            <img src=url alt="" class="w-10 h-10 rounded-full object-cover"/>
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                <span class="text-brand font-bold text-sm">{initials}</span>
                            </div>
                        }.into_any()
                    }}
                </A>

                <div class="flex-1 min-w-0">
                    // Author info
                    <div class="flex items-center gap-1.5 flex-wrap">
                        <A
                            href=format!("/profiles/{}", handle_2)
                            attr:class="font-semibold text-sm text-foreground hover:text-brand transition-colors"
                        >
                            {author_name}
                        </A>
                        {if post.author_verified {
                            view! {
                                <svg class="w-3.5 h-3.5 text-brand" viewBox="0 0 24 24" fill="currentColor">
                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        <span class="text-foreground/40 text-sm font-mono">"@"{post.author_handle}</span>
                    </div>

                    // Content with highlighted snippet
                    <div class="mt-2 text-sm text-foreground/80 line-clamp-3" inner_html=post.highlight_snippet></div>

                    // Media indicator
                    {if post.has_media {
                        view! {
                            <div class="mt-2 flex items-center gap-2 text-xs text-foreground/40">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                                <span>"Contains media"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    // Actions
                    <div class="mt-3 flex items-center gap-4 text-xs text-foreground/40">
                        <button class="flex items-center gap-1.5 hover:text-rose-400 transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                            </svg>
                            <span>{format_count(post.likes)}</span>
                        </button>
                        <button class="flex items-center gap-1.5 hover:text-brand transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                            </svg>
                            <span>{format_count(post.comments)}</span>
                        </button>
                        <button class="flex items-center gap-1.5 hover:text-emerald-400 transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/>
                            </svg>
                            <span>{format_count(post.shares)}</span>
                        </button>
                        <button class="ml-auto hover:text-brand transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </article>
    }
}

/// Tag search result card
#[component]
pub fn TagResultCard(tag: TagResult) -> impl IntoView {
    let tag_name = tag.tag.clone();
    let tag_name_2 = tag.tag.clone();

    view! {
        <article class="group bg-card border border-border/50 rounded-2xl p-4 hover:border-border hover:shadow-lg transition-all duration-200">
            <div class="flex items-start justify-between gap-4">
                <div class="flex-1 min-w-0">
                    // Tag name
                    <div class="flex items-center gap-2">
                        <A
                            href=format!("/search?tag={}", tag_name)
                            attr:class="text-xl font-bold text-brand hover:text-brand/80 transition-colors"
                        >
                            "#"{tag_name_2}
                        </A>
                        {if let Some(rank) = tag.trending_rank {
                            view! {
                                <span class="flex items-center gap-1 px-2 py-0.5 bg-amber-500/15 text-amber-400 text-xs font-bold rounded-lg">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                                    </svg>
                                    "#{rank}"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Stats
                    <div class="mt-2 flex items-center gap-4 text-sm">
                        <span class="text-foreground/60">
                            <span class="font-semibold text-foreground">{format_count(tag.post_count)}</span>
                            " posts"
                        </span>
                        <span class="text-foreground/60">
                            <span class="font-semibold text-foreground">{format_count(tag.follower_count)}</span>
                            " followers"
                        </span>
                        <span class=format!(
                            "font-medium {}",
                            if tag.growth_percentage >= 0 { "text-emerald-400" } else { "text-rose-400" }
                        )>
                            {if tag.growth_percentage >= 0 { "+" } else { "" }}{tag.growth_percentage}"%"
                        </span>
                    </div>

                    // Top synapses
                    {if !tag.top_synapses.is_empty() {
                        view! {
                            <div class="mt-3 flex items-center gap-2 text-xs text-foreground/40">
                                <span>"Popular in:"</span>
                                <div class="flex flex-wrap gap-1.5">
                                    {tag.top_synapses.iter().map(|syn| {
                                        let s = syn.clone();
                                        view! {
                                            <span class="px-2 py-0.5 bg-violet-500/10 text-violet-400 rounded-lg">{s}</span>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Follow button
                <button class=move || format!(
                    "px-4 py-1.5 rounded-xl text-sm font-medium transition-all {}",
                    if tag.is_following {
                        "bg-foreground/10 text-foreground hover:bg-rose-500/15 hover:text-rose-400"
                    } else {
                        "bg-brand/15 text-brand hover:bg-brand/25"
                    }
                )>
                    {if tag.is_following { "Following" } else { "Follow" }}
                </button>
            </div>
        </article>
    }
}
