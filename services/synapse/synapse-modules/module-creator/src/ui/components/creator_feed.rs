// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{
    ContentFilter, ContentPost, ContentSort, Creator, CreatorStats, SubscriptionStatus,
    SubscriptionTier, Supporter,
};
use crate::ui::components::{
    ContentCard, ContentFilterBar, CreatorHeader, SubscriptionSidebar, SupportersList,
};
use leptos::prelude::*;

/// Main creator feed component - the primary view for a creator's page
#[component]
pub fn CreatorFeed() -> impl IntoView {
    // Mock data
    let creator = Creator::mock_creator();
    let stats = CreatorStats::mock_public_stats();
    let tiers = SubscriptionTier::mock_tiers();
    let posts = ContentPost::mock_posts();
    let supporters = Supporter::mock_supporters();

    // User's subscription status (mock - would come from context/server)
    let (subscription_status, _set_subscription_status) =
        signal(SubscriptionStatus::NotSubscribed);

    // Content filter and sort state
    let (filter, set_filter) = signal(ContentFilter::default());
    let (sort, set_sort) = signal(ContentSort::default());
    let (view_mode, set_view_mode) = signal(ViewMode::Feed);
    let (show_sidebar, set_show_sidebar) = signal(false);

    // Get all unique tags from posts (before moving posts into closure)
    let all_tags: Vec<String> = posts
        .iter()
        .flat_map(|p| p.tags.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    // Filtered posts
    let filtered_posts = Memo::new(move |_| {
        let f = filter.get();
        let s = sort.get();
        let mut filtered: Vec<_> = posts
            .iter()
            .filter(|post| {
                // Filter by content type
                if let Some(ref ct) = f.content_type {
                    if &post.content_type != ct {
                        return false;
                    }
                }
                // Filter by access level
                if let Some(ref al) = f.access_level {
                    if &post.access_level != al {
                        return false;
                    }
                }
                // Filter by tag
                if let Some(ref tag) = f.tag {
                    if !post.tags.contains(tag) {
                        return false;
                    }
                }
                // Filter by search query
                if let Some(ref query) = f.search_query {
                    let q = query.to_lowercase();
                    let in_title = post
                        .title
                        .as_ref()
                        .map(|t| t.to_lowercase().contains(&q))
                        .unwrap_or(false);
                    let in_content = post.content.to_lowercase().contains(&q);
                    if !in_title && !in_content {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        // Sort posts (pinned always first)
        filtered.sort_by(|a, b| {
            if a.is_pinned && !b.is_pinned {
                return std::cmp::Ordering::Less;
            }
            if !a.is_pinned && b.is_pinned {
                return std::cmp::Ordering::Greater;
            }
            match s {
                ContentSort::Newest => b.created_at.cmp(&a.created_at),
                ContentSort::Oldest => a.created_at.cmp(&b.created_at),
                ContentSort::MostLiked => b.likes.cmp(&a.likes),
                ContentSort::MostCommented => b.comments.cmp(&a.comments),
            }
        });

        filtered
    });

    view! {
        <div class="flex h-full w-full bg-background relative">
            // Mobile sidebar overlay
            {move || {
                if show_sidebar.get() {
                    view! {
                        <div
                            class="fixed inset-0 bg-black/50 z-40 xl:hidden"
                            on:click=move |_| set_show_sidebar.set(false)
                        ></div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Main Content Area
            <main class="flex-1 flex flex-col overflow-hidden">
                // Scrollable content
                <div class="flex-1 overflow-y-auto scrollbar-styled">
                    // Creator Header / Banner
                    <CreatorHeader
                        creator=creator.clone()
                        stats=stats.clone()
                        subscription_status=subscription_status
                        on_subscribe=Callback::new(move |_| {
                            set_show_sidebar.set(true);
                        })
                    />

                    // Content Area with Filter Bar
                    <div class="max-w-4xl mx-auto px-4 py-6">
                        // Filter/Sort Bar
                        <ContentFilterBar
                            filter=filter
                            set_filter=set_filter
                            sort=sort
                            set_sort=set_sort
                            view_mode=view_mode
                            set_view_mode=set_view_mode
                            tags=all_tags
                        />

                        // Posts Feed
                        <div class="mt-6 space-y-6">
                            {move || {
                                let posts = filtered_posts.get();
                                if posts.is_empty() {
                                    view! {
                                        <div class="flex flex-col items-center justify-center py-16 text-center">
                                            <div class="w-16 h-16 rounded-full bg-foreground/5 flex items-center justify-center mb-4">
                                                <svg class="w-8 h-8 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m6.75 12l-3-3m0 0l-3 3m3-3v6m-1.5-15H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"></path>
                                                </svg>
                                            </div>
                                            <h3 class="text-foreground/70 font-medium mb-1">"No posts found"</h3>
                                            <p class="text-foreground/40 text-sm">"Try adjusting your filters"</p>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="space-y-6">
                                            {posts.iter().map(|post| {
                                                let post = post.clone();
                                                let is_locked = !can_view_content(&subscription_status.get(), &post.access_level);
                                                view! {
                                                    <ContentCard
                                                        post=post
                                                        is_locked=is_locked
                                                        on_unlock=Callback::new(move |_| {
                                                            set_show_sidebar.set(true);
                                                        })
                                                    />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
            </main>

            // Subscription Sidebar (slide-in on mobile, fixed on desktop)
            <aside class=move || format!(
                "fixed xl:relative inset-y-0 right-0 z-50 w-80 flex-shrink-0 border-l border-border/50 bg-panel flex flex-col transform transition-transform duration-300 xl:translate-x-0 {}",
                if show_sidebar.get() { "translate-x-0" } else { "translate-x-full" }
            )>
                <SubscriptionSidebar
                    tiers=tiers.clone()
                    supporters=supporters.clone()
                    subscription_status=subscription_status
                    on_close=Callback::new(move |_| set_show_sidebar.set(false))
                />
            </aside>

            // Mobile Subscribe Button (floating)
            <div class="fixed bottom-6 right-6 xl:hidden z-30">
                <button
                    class="flex items-center gap-2 px-5 py-3 bg-brand hover:bg-brand/90 text-white font-semibold rounded-full shadow-lg shadow-brand/30 transition-all active:scale-95"
                    on:click=move |_| set_show_sidebar.set(true)
                >
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                    </svg>
                    "Subscribe"
                </button>
            </div>
        </div>
    }
}

/// View mode for content display
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ViewMode {
    #[default]
    Feed,
    Grid,
}

/// Helper to check if user can view content based on subscription
fn can_view_content(status: &SubscriptionStatus, access: &crate::types::AccessLevel) -> bool {
    use crate::types::AccessLevel;
    match access {
        AccessLevel::Free => true,
        AccessLevel::Subscribers => matches!(status, SubscriptionStatus::Subscribed { .. }),
        AccessLevel::Tier(required_tier) => {
            if let SubscriptionStatus::Subscribed { tier_id } = status {
                // In real implementation, check tier hierarchy
                // For mock, just check if subscribed
                !tier_id.is_empty() && tier_id == required_tier
            } else {
                false
            }
        }
    }
}

