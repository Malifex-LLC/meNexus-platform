// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{AccessLevel, ContentPost, ContentType};
use leptos::prelude::*;
use leptos_router::components::A;

/// Individual content card for displaying posts
#[component]
pub fn ContentCard(
    /// The content post data
    post: ContentPost,
    /// Whether the content is locked (user doesn't have access)
    #[prop(default = false)]
    is_locked: bool,
    /// Callback when unlock/subscribe button is clicked
    on_unlock: Callback<()>,
) -> impl IntoView {
    let (is_liked, set_is_liked) = signal(false);
    let (likes_count, set_likes_count) = signal(post.likes);
    let (show_comments, set_show_comments) = signal(false);
    let (show_share_menu, set_show_share_menu) = signal(false);

    // Extract post fields
    let post_id = post.id.clone();
    let post_title = post.title.clone();
    let post_content = post.content.clone();
    let post_preview = post.preview_text.clone();
    let post_type = post.content_type.clone();
    let access_level = post.access_level.clone();
    let media = post.media.clone();
    let poll_options = post.poll_options.clone();
    let created_at = post.created_at;
    let comments_count = post.comments;
    let is_pinned = post.is_pinned;
    let tags = post.tags.clone();
    let preview_blur = post.preview_blur;

    // Format relative time
    let time_ago = format_relative_time(created_at);

    // Handle like toggle
    let toggle_like = move |_| {
        if is_liked.get() {
            set_likes_count.update(|c| *c = c.saturating_sub(1));
        } else {
            set_likes_count.update(|c| *c += 1);
        }
        set_is_liked.update(|v| *v = !*v);
    };

    view! {
        <article class="bg-panel rounded-2xl border border-border/30 overflow-hidden transition-all hover:border-border/50 group">
            // Pinned indicator
            {if is_pinned {
                view! {
                    <div class="flex items-center gap-2 px-4 py-2 bg-brand/5 border-b border-brand/10 text-brand text-sm">
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"></path>
                        </svg>
                        "Pinned post"
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Media Section (if any)
            {if !media.is_empty() && !matches!(post_type, ContentType::Text | ContentType::Poll) {
                let first_media = media[0].clone();
                let media_count = media.len();
                let thumbnail_url = first_media.thumbnail_url.clone().unwrap_or(first_media.url.clone());

                view! {
                    <div class="relative aspect-video bg-background/50 overflow-hidden">
                        // Media preview
                        <img
                            src=thumbnail_url
                            alt="Content preview"
                            class=format!(
                                "w-full h-full object-cover transition-transform duration-500 group-hover:scale-105 {}",
                                if is_locked && preview_blur > 0 { format!("blur-[{}px]", preview_blur) } else { "".to_string() }
                            )
                        />

                        // Content type badge
                        <div class="absolute top-3 left-3">
                            <ContentTypeBadge content_type=post_type.clone() />
                        </div>

                        // Access level badge
                        <div class="absolute top-3 right-3">
                            <AccessBadge level=access_level.clone() />
                        </div>

                        // Gallery count indicator
                        {if matches!(post_type, ContentType::Gallery) && media_count > 1 {
                            view! {
                                <div class="absolute bottom-3 right-3 flex items-center gap-1 px-2 py-1 bg-black/60 backdrop-blur-sm rounded-full text-white text-xs">
                                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                    </svg>
                                    {media_count}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Video duration indicator
                        {if matches!(post_type, ContentType::Video) {
                            if let Some(duration) = first_media.duration_seconds {
                                view! {
                                    <div class="absolute bottom-3 left-3 flex items-center gap-1 px-2 py-1 bg-black/60 backdrop-blur-sm rounded-full text-white text-xs">
                                        <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M8 5v14l11-7z"></path>
                                        </svg>
                                        {format_duration(duration)}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Locked overlay
                        {if is_locked {
                            view! {
                                <div class="absolute inset-0 bg-background/80 backdrop-blur-sm flex flex-col items-center justify-center text-center p-6">
                                    <div class="w-16 h-16 rounded-full bg-foreground/10 flex items-center justify-center mb-4">
                                        <svg class="w-8 h-8 text-foreground/50" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-foreground font-semibold mb-2">"Unlock this content"</h3>
                                    <p class="text-foreground/60 text-sm mb-4">
                                        "Subscribe to "{access_level.display_name()}" to access"
                                    </p>
                                    <button
                                        class="px-6 py-2.5 bg-brand hover:bg-brand/90 text-white font-semibold rounded-xl shadow-lg shadow-brand/20 transition-all active:scale-95"
                                        on:click=move |_| on_unlock.run(())
                                    >
                                        "Subscribe to Unlock"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Content Section
            <div class="p-4 sm:p-5">
                // Header row (access badge for text posts, timestamp)
                <div class="flex items-center justify-between gap-2 mb-3">
                    <div class="flex items-center gap-2">
                        {if matches!(post_type, ContentType::Text | ContentType::Poll) {
                            view! {
                                <>
                                    <ContentTypeBadge content_type=post_type.clone() />
                                    <AccessBadge level=access_level.clone() />
                                </>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    <span class="text-foreground/40 text-xs">{time_ago}</span>
                </div>

                // Title
                {if let Some(title) = post_title {
                    view! {
                        <h2 class="text-lg sm:text-xl font-bold text-foreground mb-2 leading-tight">
                            {title}
                        </h2>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Content text
                <div class=format!(
                    "text-foreground/80 text-sm sm:text-base leading-relaxed {}",
                    if is_locked { "line-clamp-3" } else { "" }
                )>
                    {if is_locked {
                        post_preview.clone().unwrap_or_else(|| truncate_text(&post_content, 150))
                    } else {
                        post_content.clone()
                    }}
                </div>

                // Poll options (if poll type and not locked)
                {if matches!(post_type, ContentType::Poll) && !is_locked {
                    if let Some(options) = poll_options {
                        view! {
                            <div class="mt-4 space-y-2">
                                {options.iter().map(|opt| {
                                    let percentage = opt.percentage;
                                    let votes = opt.votes;
                                    let text = opt.text.clone();
                                    view! {
                                        <button class="w-full group/poll">
                                            <div class="relative p-3 bg-foreground/5 rounded-xl border border-border/30 hover:border-brand/30 transition-colors overflow-hidden">
                                                // Progress bar
                                                <div
                                                    class="absolute inset-y-0 left-0 bg-brand/10 transition-all duration-500"
                                                    style=format!("width: {}%", percentage)
                                                ></div>
                                                // Content
                                                <div class="relative flex items-center justify-between">
                                                    <span class="text-foreground text-sm font-medium">{text}</span>
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-foreground/50 text-xs">{votes}" votes"</span>
                                                        <span class="text-brand font-semibold text-sm">{format!("{:.1}%", percentage)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else if matches!(post_type, ContentType::Poll) && is_locked {
                    view! {
                        <div class="mt-4 p-4 bg-foreground/5 rounded-xl border border-border/30 text-center">
                            <svg class="w-8 h-8 text-foreground/30 mx-auto mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                            </svg>
                            <p class="text-foreground/50 text-sm">"Subscribe to participate in this poll"</p>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // File download info (if file type and not locked)
                {if matches!(post_type, ContentType::File) && !is_locked {
                    if let Some(file_media) = media.first() {
                        let file_name = file_media.file_name.clone().unwrap_or_else(|| "Download".to_string());
                        let file_size = file_media.file_size.clone().unwrap_or_default();
                        view! {
                            <div class="mt-4">
                                <a
                                    href=file_media.url.clone()
                                    class="flex items-center gap-3 p-4 bg-foreground/5 rounded-xl border border-border/30 hover:border-brand/30 hover:bg-brand/5 transition-all group/file"
                                >
                                    <div class="w-12 h-12 rounded-lg bg-brand/10 flex items-center justify-center flex-shrink-0">
                                        <svg class="w-6 h-6 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                                        </svg>
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <p class="text-foreground font-medium truncate">{file_name}</p>
                                        <p class="text-foreground/50 text-sm">{file_size}</p>
                                    </div>
                                    <div class="flex-shrink-0 px-4 py-2 bg-brand text-white font-medium rounded-lg text-sm group-hover/file:shadow-lg group-hover/file:shadow-brand/20 transition-all">
                                        "Download"
                                    </div>
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Audio player (if audio type and not locked)
                {if matches!(post_type, ContentType::Audio) && !is_locked {
                    if let Some(audio_media) = media.first() {
                        let duration = audio_media.duration_seconds.map(format_duration).unwrap_or_default();
                        view! {
                            <div class="mt-4">
                                <div class="flex items-center gap-3 p-4 bg-foreground/5 rounded-xl border border-border/30">
                                    <button class="w-12 h-12 rounded-full bg-brand flex items-center justify-center flex-shrink-0 hover:bg-brand/90 transition-colors shadow-lg shadow-brand/20">
                                        <svg class="w-5 h-5 text-white ml-0.5" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M8 5v14l11-7z"></path>
                                        </svg>
                                    </button>
                                    <div class="flex-1 min-w-0">
                                        // Waveform placeholder
                                        <div class="h-8 flex items-center gap-0.5">
                                            {(0..40).map(|i| {
                                                let height = 20 + (i * 7 % 30);
                                                view! {
                                                    <div
                                                        class="flex-1 bg-brand/30 rounded-full"
                                                        style=format!("height: {}%", height)
                                                    ></div>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="flex items-center justify-between text-xs text-foreground/50 mt-1">
                                            <span>"0:00"</span>
                                            <span>{duration}</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Tags
                {if !tags.is_empty() && !is_locked {
                    view! {
                        <div class="flex flex-wrap gap-2 mt-4">
                            {tags.iter().map(|tag| {
                                let tag = tag.clone();
                                view! {
                                    <span class="px-2 py-1 rounded-md bg-foreground/5 text-foreground/50 text-xs hover:bg-foreground/10 hover:text-foreground/70 transition-colors cursor-pointer">
                                        "#"{tag}
                                    </span>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Actions bar
                <div class="flex items-center justify-between mt-4 pt-4 border-t border-border/30">
                    <div class="flex items-center gap-1">
                        // Like button
                        <button
                            class=move || format!(
                                "flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-all {}",
                                if is_liked.get() {
                                    "bg-rose-500/10 text-rose-500"
                                } else {
                                    "text-foreground/50 hover:bg-foreground/5 hover:text-foreground"
                                }
                            )
                            on:click=toggle_like
                        >
                            <svg class="w-5 h-5" fill=move || if is_liked.get() { "currentColor" } else { "none" } viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                            </svg>
                            <span class="text-sm font-medium">{move || likes_count.get()}</span>
                        </button>

                        // Comments button
                        <button
                            class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-foreground/50 hover:bg-foreground/5 hover:text-foreground transition-all"
                            on:click=move |_| set_show_comments.update(|v| *v = !*v)
                        >
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                            </svg>
                            <span class="text-sm font-medium">{comments_count}</span>
                        </button>

                        // Share button
                        <div class="relative">
                            <button
                                class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-foreground/50 hover:bg-foreground/5 hover:text-foreground transition-all"
                                on:click=move |_| set_show_share_menu.update(|v| *v = !*v)
                            >
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                                </svg>
                            </button>

                            // Share dropdown
                            {move || {
                                if show_share_menu.get() {
                                    view! {
                                        <div class="absolute bottom-full left-0 mb-2 w-48 bg-panel border border-border/50 rounded-xl shadow-xl p-2 z-10">
                                            <button class="w-full flex items-center gap-2 px-3 py-2 text-sm text-foreground/70 hover:bg-foreground/5 rounded-lg transition-colors">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                                </svg>
                                                "Copy link"
                                            </button>
                                            <button class="w-full flex items-center gap-2 px-3 py-2 text-sm text-foreground/70 hover:bg-foreground/5 rounded-lg transition-colors">
                                                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                                    <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path>
                                                </svg>
                                                "Share on X"
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    </div>

                    // Bookmark button
                    <button class="p-2 rounded-lg text-foreground/50 hover:bg-foreground/5 hover:text-foreground transition-all">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"></path>
                        </svg>
                    </button>
                </div>
            </div>
        </article>
    }
}

/// Content type badge component
#[component]
fn ContentTypeBadge(content_type: ContentType) -> impl IntoView {
    let (icon, label, color) = match content_type {
        ContentType::Image => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                </svg>
            }.into_any(),
            "Image",
            "text-emerald-400 bg-emerald-500/10",
        ),
        ContentType::Gallery => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path>
                </svg>
            }.into_any(),
            "Gallery",
            "text-violet-400 bg-violet-500/10",
        ),
        ContentType::Video => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                </svg>
            }.into_any(),
            "Video",
            "text-rose-400 bg-rose-500/10",
        ),
        ContentType::Audio => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3"></path>
                </svg>
            }.into_any(),
            "Audio",
            "text-amber-400 bg-amber-500/10",
        ),
        ContentType::Text => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
            }.into_any(),
            "Article",
            "text-sky-400 bg-sky-500/10",
        ),
        ContentType::File => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
            }.into_any(),
            "Download",
            "text-cyan-400 bg-cyan-500/10",
        ),
        ContentType::Poll => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                </svg>
            }.into_any(),
            "Poll",
            "text-indigo-400 bg-indigo-500/10",
        ),
        ContentType::Livestream => (
            view! {
                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M5.636 18.364a9 9 0 010-12.728m12.728 0a9 9 0 010 12.728m-9.9-2.829a5 5 0 010-7.07m7.072 0a5 5 0 010 7.07M13 12a1 1 0 11-2 0 1 1 0 012 0z"></path>
                </svg>
            }.into_any(),
            "Live",
            "text-red-400 bg-red-500/10",
        ),
    };

    view! {
        <span class=format!("flex items-center gap-1 px-2 py-1 rounded-md text-xs font-medium {}", color)>
            {icon}
            {label}
        </span>
    }
}

/// Access level badge component
#[component]
fn AccessBadge(level: AccessLevel) -> impl IntoView {
    match level {
        AccessLevel::Free => view! {
            <span class="flex items-center gap-1 px-2 py-1 rounded-md bg-emerald-500/20 text-emerald-400 text-xs font-medium backdrop-blur-sm">
                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"></path>
                </svg>
                "Free"
            </span>
        }.into_any(),
        AccessLevel::Subscribers => view! {
            <span class="flex items-center gap-1 px-2 py-1 rounded-md bg-brand/20 text-brand text-xs font-medium backdrop-blur-sm">
                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                </svg>
                "Subscribers"
            </span>
        }.into_any(),
        AccessLevel::Tier(name) => view! {
            <span class="flex items-center gap-1 px-2 py-1 rounded-md bg-amber-500/20 text-amber-400 text-xs font-medium backdrop-blur-sm">
                <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M12 2L15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2z"></path>
                </svg>
                {name}
            </span>
        }.into_any(),
    }
}

/// Format duration in seconds to MM:SS or HH:MM:SS
fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

/// Format relative time (e.g., "2 hours ago")
fn format_relative_time(timestamp: time::OffsetDateTime) -> String {
    let now = time::OffsetDateTime::now_utc();
    let diff = now - timestamp;

    if diff.whole_seconds() < 60 {
        "Just now".to_string()
    } else if diff.whole_minutes() < 60 {
        let mins = diff.whole_minutes();
        format!("{}m ago", mins)
    } else if diff.whole_hours() < 24 {
        let hours = diff.whole_hours();
        format!("{}h ago", hours)
    } else if diff.whole_days() < 7 {
        let days = diff.whole_days();
        format!("{}d ago", days)
    } else if diff.whole_weeks() < 4 {
        let weeks = diff.whole_weeks();
        format!("{}w ago", weeks)
    } else {
        timestamp.date().to_string()
    }
}

/// Truncate text to a maximum length
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    }
}

