// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::result_cards::{PostResultCard, SynapseResultCard, TagResultCard, UserResultCard};
use super::types::{PostResult, SearchCategory, SynapseResult, TagResult, UserResult};
use leptos::prelude::*;

/// Search results display component
#[component]
pub fn SearchResults(
    query: String,
    category: SearchCategory,
) -> impl IntoView {
    let query_for_users = query.clone();
    let query_for_posts = query.clone();
    let query_for_tags = query.clone();

    // Generate mock results based on category
    let users = UserResult::mock_results(&query_for_users);
    let synapses = SynapseResult::mock_results(&query);
    let posts = PostResult::mock_results(&query_for_posts);
    let tags = TagResult::mock_results(&query_for_tags);

    view! {
        <div class="space-y-4">
            {match category {
                SearchCategory::All => {
                    view! {
                        <AllResultsView
                            users=users.clone()
                            synapses=synapses.clone()
                            posts=posts.clone()
                            tags=tags.clone()
                        />
                    }.into_any()
                }
                SearchCategory::Users => {
                    view! {
                        <div class="space-y-3">
                            {users.into_iter().map(|user| {
                                view! { <UserResultCard user=user/> }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
                SearchCategory::Synapses => {
                    view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            {synapses.into_iter().map(|synapse| {
                                view! { <SynapseResultCard synapse=synapse/> }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
                SearchCategory::Posts => {
                    view! {
                        <div class="space-y-3">
                            {posts.into_iter().map(|post| {
                                view! { <PostResultCard post=post/> }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
                SearchCategory::Tags => {
                    view! {
                        <div class="space-y-3">
                            {tags.into_iter().map(|tag| {
                                view! { <TagResultCard tag=tag/> }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Combined view showing top results from each category
#[component]
fn AllResultsView(
    users: Vec<UserResult>,
    synapses: Vec<SynapseResult>,
    posts: Vec<PostResult>,
    tags: Vec<TagResult>,
) -> impl IntoView {
    view! {
        <div class="space-y-8">
            // Top Users
            {if !users.is_empty() {
                let display_users: Vec<_> = users.into_iter().take(3).collect();
                view! {
                    <section>
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="flex items-center gap-2 text-sm font-semibold text-foreground">
                                <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                                </svg>
                                "Users"
                            </h3>
                            <button class="text-xs text-brand hover:underline">"See all users"</button>
                        </div>
                        <div class="space-y-3">
                            {display_users.into_iter().map(|user| {
                                view! { <UserResultCard user=user/> }
                            }).collect_view()}
                        </div>
                    </section>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Top Synapses
            {if !synapses.is_empty() {
                let display_synapses: Vec<_> = synapses.into_iter().take(2).collect();
                view! {
                    <section>
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="flex items-center gap-2 text-sm font-semibold text-foreground">
                                <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                </svg>
                                "Synapses"
                            </h3>
                            <button class="text-xs text-brand hover:underline">"See all synapses"</button>
                        </div>
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                            {display_synapses.into_iter().map(|synapse| {
                                view! { <SynapseResultCard synapse=synapse/> }
                            }).collect_view()}
                        </div>
                    </section>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Top Posts
            {if !posts.is_empty() {
                let display_posts: Vec<_> = posts.into_iter().take(3).collect();
                view! {
                    <section>
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="flex items-center gap-2 text-sm font-semibold text-foreground">
                                <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/>
                                </svg>
                                "Posts"
                            </h3>
                            <button class="text-xs text-brand hover:underline">"See all posts"</button>
                        </div>
                        <div class="space-y-3">
                            {display_posts.into_iter().map(|post| {
                                view! { <PostResultCard post=post/> }
                            }).collect_view()}
                        </div>
                    </section>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Related Tags
            {if !tags.is_empty() {
                let display_tags: Vec<_> = tags.into_iter().take(3).collect();
                view! {
                    <section>
                        <div class="flex items-center justify-between mb-3">
                            <h3 class="flex items-center gap-2 text-sm font-semibold text-foreground">
                                <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"/>
                                </svg>
                                "Related Tags"
                            </h3>
                            <button class="text-xs text-brand hover:underline">"See all tags"</button>
                        </div>
                        <div class="space-y-3">
                            {display_tags.into_iter().map(|tag| {
                                view! { <TagResultCard tag=tag/> }
                            }).collect_view()}
                        </div>
                    </section>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

/// Empty state when no search has been performed
#[component]
pub fn EmptySearchState() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 text-center">
            <div class="w-20 h-20 rounded-full bg-brand/10 flex items-center justify-center mb-6">
                <svg class="w-10 h-10 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
            </div>
            <h3 class="text-xl font-semibold text-foreground mb-2">"Discover the Network"</h3>
            <p class="text-foreground/50 max-w-md mb-6">
                "Search for users, synapses, posts, and tags. Or explore trending content below."
            </p>
            <div class="flex flex-wrap justify-center gap-2">
                {["#LocalFirst", "#RustLang", "#P2P", "@neo", "@oracle", "Zion HQ"].into_iter().map(|suggestion| {
                    view! {
                        <button class="px-4 py-2 bg-foreground/5 hover:bg-foreground/10 text-foreground/70 hover:text-foreground rounded-xl text-sm font-medium transition-colors">
                            {suggestion}
                        </button>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

/// No results found state
#[component]
pub fn NoResultsState(query: String) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center py-16 text-center">
            <div class="w-20 h-20 rounded-full bg-foreground/5 flex items-center justify-center mb-6">
                <svg class="w-10 h-10 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                </svg>
            </div>
            <h3 class="text-xl font-semibold text-foreground mb-2">"No results found"</h3>
            <p class="text-foreground/50 max-w-md mb-6">
                "We couldn't find anything matching \""<span class="text-foreground font-medium">{query}</span>"\". Try different keywords or browse trending content."
            </p>
            <div class="flex gap-3">
                <button class="px-4 py-2 bg-foreground/5 hover:bg-foreground/10 text-foreground rounded-xl text-sm font-medium transition-colors">
                    "Clear search"
                </button>
                <button class="px-4 py-2 bg-brand hover:bg-brand/90 text-white rounded-xl text-sm font-medium shadow-lg shadow-brand/20 transition-colors">
                    "Browse trending"
                </button>
            </div>
        </div>
    }
}
