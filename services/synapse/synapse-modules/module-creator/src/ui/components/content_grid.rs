// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{AccessLevel, ContentPost, ContentType};
use leptos::prelude::*;

/// Grid view for content posts (alternative to feed view)
#[component]
pub fn ContentGrid(
    /// List of posts to display
    posts: Vec<ContentPost>,
    /// Callback when a locked post is clicked
    on_unlock: Callback<()>,
    /// Check if content is locked for user
    #[prop(into)]
    is_locked_fn: Callback<AccessLevel, bool>,
) -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3 sm:gap-4">
            {posts.into_iter().map(|post| {
                let is_locked = is_locked_fn.run(post.access_level.clone());
                view! {
                    <ContentGridItem
                        post=post
                        is_locked=is_locked
                        on_click=on_unlock
                    />
                }
            }).collect_view()}
        </div>
    }
}

/// Helper function to generate content type icon
fn content_type_icon(content_type: &ContentType) -> impl IntoView {
    match content_type {
        ContentType::Image => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
            </svg>
        }.into_any(),
        ContentType::Gallery => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path>
            </svg>
        }.into_any(),
        ContentType::Video => view! {
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                <path d="M8 5v14l11-7z"></path>
            </svg>
        }.into_any(),
        ContentType::Audio => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path>
            </svg>
        }.into_any(),
        ContentType::Text => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
            </svg>
        }.into_any(),
        ContentType::File => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
            </svg>
        }.into_any(),
        ContentType::Poll => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
            </svg>
        }.into_any(),
        ContentType::Livestream => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5.636 18.364a9 9 0 010-12.728m12.728 0a9 9 0 010 12.728m-9.9-2.829a5 5 0 010-7.07m7.072 0a5 5 0 010 7.07M13 12a1 1 0 11-2 0 1 1 0 012 0z"></path>
            </svg>
        }.into_any(),
    }
}

/// Individual grid item
#[component]
fn ContentGridItem(
    post: ContentPost,
    is_locked: bool,
    on_click: Callback<()>,
) -> impl IntoView {
    let post_title = post.title.clone();
    let post_type = post.content_type.clone();
    let post_type_for_badge = post.content_type.clone();
    let post_type_for_placeholder = post.content_type.clone();
    let access_level = post.access_level.clone();
    let media = post.media.clone();
    let likes = post.likes;
    let comments = post.comments;
    let preview_blur = post.preview_blur;

    // Get thumbnail URL
    let thumbnail_url = media.first().and_then(|m| {
        m.thumbnail_url.clone().or(Some(m.url.clone()))
    });

    // Access badge color
    let (access_bg, access_text) = match &access_level {
        AccessLevel::Free => ("bg-emerald-500/20", "text-emerald-400"),
        AccessLevel::Subscribers => ("bg-brand/20", "text-brand"),
        AccessLevel::Tier(_) => ("bg-amber-500/20", "text-amber-400"),
    };

    view! {
        <button
            class="group relative aspect-square rounded-xl overflow-hidden border border-border/30 hover:border-brand/30 transition-all bg-panel"
            on:click=move |_| {
                if is_locked {
                    on_click.run(());
                }
                // Otherwise, navigate to post detail
            }
        >
            // Thumbnail or placeholder
            {if let Some(url) = thumbnail_url {
                view! {
                    <img
                        src=url
                        alt=post_title.clone().unwrap_or_default()
                        class=format!(
                            "w-full h-full object-cover transition-transform duration-500 group-hover:scale-105 {}",
                            if is_locked && preview_blur > 0 { format!("blur-[{}px]", preview_blur) } else { "".to_string() }
                        )
                    />
                }.into_any()
            } else {
                // Placeholder for text/poll content
                view! {
                    <div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-foreground/5 to-foreground/10">
                        <div class="text-foreground/30">
                            {content_type_icon(&post_type_for_placeholder)}
                        </div>
                    </div>
                }.into_any()
            }}

            // Top overlay - badges
            <div class="absolute top-2 left-2 right-2 flex items-center justify-between pointer-events-none">
                // Content type badge
                <span class="flex items-center gap-1 px-1.5 py-0.5 rounded bg-black/50 backdrop-blur-sm text-white text-xs">
                    {content_type_icon(&post_type_for_badge)}
                </span>

                // Access badge
                <span class=format!(
                    "px-1.5 py-0.5 rounded text-xs font-medium backdrop-blur-sm {} {}",
                    access_bg, access_text
                )>
                    {match &access_level {
                        AccessLevel::Free => "Free",
                        AccessLevel::Subscribers => "Sub",
                        AccessLevel::Tier(t) => t.as_str(),
                    }}
                </span>
            </div>

            // Bottom overlay - title and stats
            <div class="absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/80 via-black/50 to-transparent p-3 pt-8">
                {if let Some(title) = post_title {
                    view! {
                        <h3 class="text-white text-sm font-medium line-clamp-2 leading-tight mb-1">
                            {title}
                        </h3>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                <div class="flex items-center gap-3 text-white/70 text-xs">
                    <span class="flex items-center gap-1">
                        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                        </svg>
                        {format_count(likes)}
                    </span>
                    <span class="flex items-center gap-1">
                        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                        </svg>
                        {format_count(comments)}
                    </span>
                </div>
            </div>

            // Locked overlay
            {if is_locked {
                view! {
                    <div class="absolute inset-0 bg-background/60 backdrop-blur-[2px] flex flex-col items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                        <div class="w-12 h-12 rounded-full bg-foreground/10 flex items-center justify-center mb-2">
                            <svg class="w-6 h-6 text-foreground/70" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                            </svg>
                        </div>
                        <span class="text-foreground text-sm font-medium">"Unlock"</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Gallery count
            {if matches!(post_type, ContentType::Gallery) && media.len() > 1 {
                view! {
                    <div class="absolute bottom-2 right-2 flex items-center gap-1 px-1.5 py-0.5 bg-black/50 backdrop-blur-sm rounded text-white text-xs">
                        {media.len()}
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </button>
    }
}

/// Format large numbers (e.g., 1.2K)
fn format_count(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}
