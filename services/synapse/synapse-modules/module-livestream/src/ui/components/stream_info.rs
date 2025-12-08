// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{StreamInfo, StreamStats, StreamStatus};
use leptos::prelude::*;

/// Stream information panel with title, description, tags, and stats
#[component]
pub fn StreamInfoPanel(
    #[prop(into)] info: StreamInfo,
    #[prop(into)] stats: StreamStats,
    #[prop(into)] status: StreamStatus,
) -> impl IntoView {
    let (show_full_description, set_show_full_description) = signal(false);
    let (copied_link, set_copied_link) = signal(false);

    let is_live = status == StreamStatus::Live;
    let title = info.title.clone();
    let description = info.description.clone();
    let description_for_toggle = info.description.clone();
    let category = info.category.clone();
    let tags = info.tags.clone();
    let started_at = info.started_at.clone();

    // Truncate description
    let truncated_desc = if description.len() > 200 {
        format!("{}...", &description[..200])
    } else {
        description.clone()
    };
    let has_long_desc = description.len() > 200;

    view! {
        <div class="bg-panel/50 border border-border/50 rounded-xl overflow-hidden">
            // Header with title
            <div class="p-4 border-b border-border/30">
                <div class="flex items-start justify-between gap-4">
                    <div class="flex-1 min-w-0">
                        <h1 class="text-xl font-bold text-foreground leading-tight">{title}</h1>
                        
                        {if is_live {
                            view! {
                                <div class="flex items-center gap-3 mt-2 text-sm">
                                    // Category
                                    {if !category.is_empty() {
                                        view! {
                                            <a href="#" class="flex items-center gap-1.5 text-brand hover:underline">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                                </svg>
                                                {category.clone()}
                                            </a>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                    
                                    // Started time
                                    {if let Some(started) = started_at {
                                        view! {
                                            <span class="text-foreground/50 flex items-center gap-1.5">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                </svg>
                                                "Started "{started}
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Actions
                    <div class="flex items-center gap-2 flex-shrink-0">
                        // Share button
                        <button 
                            class=move || format!(
                                "flex items-center gap-2 px-3 py-2 rounded-lg text-sm font-medium transition-all {}",
                                if copied_link.get() {
                                    "bg-emerald-500/20 text-emerald-400"
                                } else {
                                    "bg-foreground/5 text-foreground/70 hover:bg-foreground/10 hover:text-foreground"
                                }
                            )
                            on:click=move |_| {
                                set_copied_link.set(true);
                                // Reset after 2 seconds
                                set_timeout(move || set_copied_link.set(false), std::time::Duration::from_secs(2));
                            }
                        >
                            {move || {
                                if copied_link.get() {
                                    view! {
                                        <>
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"></path>
                                            </svg>
                                            "Copied!"
                                        </>
                                    }.into_any()
                                } else {
                                    view! {
                                        <>
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                                            </svg>
                                            "Share"
                                        </>
                                    }.into_any()
                                }
                            }}
                        </button>

                        // Clip button (if live)
                        {if is_live {
                            view! {
                                <button class="flex items-center gap-2 px-3 py-2 rounded-lg bg-foreground/5 text-foreground/70 hover:bg-foreground/10 hover:text-foreground text-sm font-medium transition-all">
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                    "Clip"
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>
            </div>

            // Description
            {if !description.is_empty() {
                view! {
                    <div class="p-4 border-b border-border/30">
                        <p class="text-foreground/70 text-sm leading-relaxed whitespace-pre-wrap">
                            {move || {
                                if show_full_description.get() || !has_long_desc {
                                    description_for_toggle.clone()
                                } else {
                                    truncated_desc.clone()
                                }
                            }}
                        </p>
                        {if has_long_desc {
                            view! {
                                <button 
                                    class="mt-2 text-sm text-brand hover:underline"
                                    on:click=move |_| set_show_full_description.update(|s| *s = !*s)
                                >
                                    {move || if show_full_description.get() { "Show less" } else { "Show more" }}
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Tags
            {if !tags.is_empty() && is_live {
                view! {
                    <div class="p-4 border-b border-border/30">
                        <div class="flex items-center gap-2 flex-wrap">
                            {tags.iter().map(|tag| {
                                let tag_for_href = tag.clone();
                                let tag_for_display = tag.clone();
                                view! {
                                    <a 
                                        href=format!("/tags/{}", tag_for_href.to_lowercase().replace(' ', "-"))
                                        class="px-3 py-1 bg-foreground/5 hover:bg-foreground/10 text-foreground/70 hover:text-foreground rounded-full text-sm transition-colors"
                                    >
                                        {tag_for_display}
                                    </a>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Stream stats (only when live)
            {if is_live {
                view! {
                    <div class="p-4 bg-background/30">
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            // Viewers
                            <div class="flex flex-col items-center p-3 rounded-lg bg-foreground/5">
                                <div class="flex items-center gap-1.5 text-foreground/50 text-xs uppercase tracking-wider mb-1">
                                    <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"></path>
                                    </svg>
                                    "Viewers"
                                </div>
                                <span class="text-xl font-bold text-foreground">{stats.viewer_count.to_string()}</span>
                            </div>

                            // Peak viewers
                            <div class="flex flex-col items-center p-3 rounded-lg bg-foreground/5">
                                <div class="flex items-center gap-1.5 text-foreground/50 text-xs uppercase tracking-wider mb-1">
                                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
                                    </svg>
                                    "Peak"
                                </div>
                                <span class="text-xl font-bold text-foreground">{stats.peak_viewers.to_string()}</span>
                            </div>

                            // Resolution/Quality
                            <div class="flex flex-col items-center p-3 rounded-lg bg-foreground/5">
                                <div class="flex items-center gap-1.5 text-foreground/50 text-xs uppercase tracking-wider mb-1">
                                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                                    </svg>
                                    "Quality"
                                </div>
                                <span class="text-xl font-bold text-foreground font-mono">{format!("{}fps", stats.fps)}</span>
                            </div>

                            // Bitrate
                            <div class="flex flex-col items-center p-3 rounded-lg bg-foreground/5">
                                <div class="flex items-center gap-1.5 text-foreground/50 text-xs uppercase tracking-wider mb-1">
                                    <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                    </svg>
                                    "Bitrate"
                                </div>
                                <span class="text-xl font-bold text-foreground font-mono">{format!("{}k", stats.bitrate_kbps / 1000)}</span>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

