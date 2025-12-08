// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::feed_filters::FeedFilters;
use super::global_compose::GlobalCompose;
use super::global_post_card::GlobalPostCard;
use super::types::{GlobalPost, SortOrder};
use leptos::prelude::*;

/// Main global feed component
#[component]
pub fn GlobalFeed() -> impl IntoView {
    let posts = GlobalPost::mock_posts();
    let (sort_order, set_sort_order) = signal(SortOrder::Latest);
    
    let on_sort_change = Callback::new(move |order: SortOrder| {
        set_sort_order.set(order);
    });

    view! {
        <div class="space-y-4">
            // Compose bar
            <GlobalCompose />

            // Filters
            <FeedFilters
                sort_order=Signal::derive(move || sort_order.get())
                on_sort_change=on_sort_change
            />

            // Feed header
            <div class="flex items-center justify-between px-1">
                <div class="flex items-center gap-2">
                    <h2 class="text-sm font-semibold text-foreground">"Global Feed"</h2>
                    <span class="text-xs text-foreground/40 font-mono">"({} posts)", posts.len()</span>
                </div>
                <div class="flex items-center gap-1.5 text-xs text-foreground/40">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                    "Live"
                </div>
            </div>

            // Posts list
            <div class="space-y-4">
                {posts.into_iter().map(|post| {
                    view! { <GlobalPostCard post=post /> }
                }).collect_view()}
            </div>

            // Load more
            <div class="pt-4">
                <button class="w-full py-3 px-4 rounded-xl border border-border/30 text-foreground/50 text-sm font-medium hover:bg-foreground/5 hover:text-foreground/70 hover:border-border/50 transition-all">
                    "Load more posts"
                </button>
            </div>
        </div>
    }
}
