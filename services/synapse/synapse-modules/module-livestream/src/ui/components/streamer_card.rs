// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::Streamer;
use leptos::prelude::*;
use leptos_router::components::A;

/// Card displaying streamer/host information
#[component]
pub fn StreamerCard(
    #[prop(into)] streamer: Streamer,
    /// Whether the stream is currently live
    #[prop(into, optional)] is_live: Option<bool>,
) -> impl IntoView {
    let is_live = is_live.unwrap_or(false);
    let (is_following, set_is_following) = signal(true);
    let (show_more, set_show_more) = signal(false);

    // Generate initials for avatar fallback
    let initials = streamer.display_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    let handle = streamer.handle.clone();
    let handle_for_link = streamer.handle.clone();
    let display_name = streamer.display_name.clone();
    let display_name_for_alt = streamer.display_name.clone();
    let follower_count = format_count(streamer.follower_count);
    let is_verified = streamer.is_verified;
    let is_host = streamer.is_synapse_host;
    let avatar_url = streamer.avatar_url.clone();

    view! {
        <div class="bg-panel/50 border border-border/50 rounded-xl p-4">
            <div class="flex items-start gap-4">
                // Avatar
                <A href=format!("/profiles/{}", handle_for_link) attr:class="relative flex-shrink-0 group">
                    {if let Some(avatar) = avatar_url {
                        view! {
                            <img 
                                src=avatar
                                alt=format!("{}'s avatar", display_name_for_alt)
                                class="w-16 h-16 rounded-xl object-cover ring-2 ring-border/50 group-hover:ring-brand/50 transition-all"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-16 h-16 rounded-xl bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center ring-2 ring-border/50 group-hover:ring-brand/50 transition-all">
                                <span class="text-brand font-bold text-xl">{initials}</span>
                            </div>
                        }.into_any()
                    }}
                    
                    // Live indicator on avatar
                    {if is_live {
                        view! {
                            <div class="absolute -bottom-1 -right-1 w-5 h-5 bg-rose-500 rounded-full border-2 border-panel flex items-center justify-center">
                                <span class="w-2 h-2 bg-white rounded-full animate-pulse"></span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </A>

                // Info
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 flex-wrap">
                        <A href=format!("/profiles/{}", handle) attr:class="font-bold text-foreground hover:text-brand transition-colors text-lg">
                            {display_name.clone()}
                        </A>
                        
                        // Verified badge
                        {if is_verified {
                            view! {
                                <span class="flex items-center gap-1 px-1.5 py-0.5 bg-sky-500/20 text-sky-400 rounded text-xs font-medium" title="Verified">
                                    <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
                                    </svg>
                                    "Verified"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        
                        // Host badge
                        {if is_host {
                            view! {
                                <span class="flex items-center gap-1 px-1.5 py-0.5 bg-amber-500/20 text-amber-400 rounded text-xs font-medium" title="Synapse Host">
                                    <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>
                                    </svg>
                                    "Host"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    
                    <div class="flex items-center gap-2 mt-1">
                        <span class="text-brand/80 font-mono text-sm">{"@"}{handle.clone()}</span>
                        <span class="text-foreground/30">"·"</span>
                        <span class="text-foreground/50 text-sm">{follower_count}" followers"</span>
                    </div>

                    // Action buttons
                    <div class="flex items-center gap-2 mt-3">
                        <button 
                            class=move || format!(
                                "flex items-center gap-2 px-4 py-2 rounded-lg font-medium text-sm transition-all {}",
                                if is_following.get() {
                                    "bg-foreground/10 text-foreground hover:bg-foreground/15"
                                } else {
                                    "bg-brand text-white hover:bg-brand/90"
                                }
                            )
                            on:click=move |_| set_is_following.update(|f| *f = !*f)
                        >
                            {move || {
                                if is_following.get() {
                                    view! {
                                        <>
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"></path>
                                            </svg>
                                            "Following"
                                        </>
                                    }.into_any()
                                } else {
                                    view! {
                                        <>
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"></path>
                                            </svg>
                                            "Follow"
                                        </>
                                    }.into_any()
                                }
                            }}
                        </button>

                        <button 
                            class="flex items-center gap-2 px-4 py-2 rounded-lg bg-foreground/5 text-foreground/70 hover:bg-foreground/10 hover:text-foreground font-medium text-sm transition-all"
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                            </svg>
                            "Subscribe"
                        </button>

                        <div class="relative ml-auto">
                            <button 
                                class="p-2 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors"
                                on:click=move |_| set_show_more.update(|s| *s = !*s)
                            >
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path>
                                </svg>
                            </button>

                            // More options dropdown
                            {move || {
                                if show_more.get() {
                                    view! {
                                        <div class="absolute top-full right-0 mt-1 w-48 bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-2">
                                            <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                                                </svg>
                                                "Share Profile"
                                            </button>
                                            <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                </svg>
                                                "About"
                                            </button>
                                            <div class="my-1 border-t border-border/30"></div>
                                            <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-rose-400 hover:bg-rose-500/10 transition-colors">
                                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"></path>
                                                </svg>
                                                "Block"
                                            </button>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

