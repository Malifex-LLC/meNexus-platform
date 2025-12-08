// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::{
    server_fns::list_posts_for_channel_server, types::ListPostsForChannelRequest,
    ui::components::compose_bar::ComposeBar, ui::components::post_card::PostCard,
};
use leptos::prelude::*;
use synapse_core::domain::profiles::Profile;

/// Main posts feed component - responsive with collapsible channel selector
#[component]
pub fn PostsFeed(session_user_profile: Profile) -> impl IntoView {
    provide_context(session_user_profile);

    let channels = vec![
        "general".to_string(),
        "memes".to_string(),
        "support".to_string(),
    ];
    let channels_for_dropdown = channels.clone();
    let channels_for_sidebar = channels.clone();

    let (active_channel, set_active_channel) = signal(channels[0].clone());
    let (show_channel_dropdown, set_show_channel_dropdown) = signal(false);
    let (show_sidebar, set_show_sidebar) = signal(false);
    let (refresh, set_refresh) = signal(0usize);

    let posts = Resource::new(
        move || (active_channel.get(), refresh.get()),
        |(channel, refresh_counter): (String, usize)| async move {
            list_posts_for_channel_server(ListPostsForChannelRequest {
                event_type: "posts:create_post".to_string(),
                channel,
            })
            .await
            .unwrap_or_default()
        },
    );

    let on_post_created = {
        let set_refresh = set_refresh.clone();
        Callback::new(move |_: ()| {
            set_refresh.update(|n| *n += 1);
        })
    };

    view! {
        <div class="flex h-full w-full bg-panel/70 relative">
            // Mobile sidebar overlay
            {move || {
                if show_sidebar.get() {
                    view! {
                        <div
                            class="fixed inset-0 bg-black/50 z-40 lg:hidden"
                            on:click=move |_| set_show_sidebar.set(false)
                        ></div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Sidebar - Channel Navigation (hidden on small screens)
            <aside class=move || format!(
                "fixed lg:relative inset-y-0 left-0 z-50 w-56 flex-shrink-0 border-r border-border/50 bg-panel flex flex-col transform transition-transform duration-300 lg:translate-x-0 {}",
                if show_sidebar.get() { "translate-x-0" } else { "-translate-x-full" }
            )>
                <div class="p-3">
                    // Header
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-2">
                            <div class="w-7 h-7 rounded-lg bg-gradient-to-br from-brand to-brand/60 flex items-center justify-center">
                                <svg class="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"></path>
                                </svg>
                            </div>
                            <h2 class="text-foreground font-semibold text-sm">"Channels"</h2>
                        </div>
                        <button
                            class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors lg:hidden"
                            on:click=move |_| set_show_sidebar.set(false)
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    </div>

                    // Channel List
                    <nav class="space-y-0.5">
                        {channels_for_sidebar.iter().map(|channel| {
                            let channel_for_button = channel.clone();
                            let channel_for_click = channel.clone();
                            let channel_for_indicator = channel.clone();
                            let channel_display = channel.clone();

                            view! {
                                <button
                                    class=move || {
                                        let is_active = active_channel.get() == channel_for_button;
                                        format!(
                                            "w-full group flex items-center gap-2 px-2 py-1.5 rounded-lg text-left transition-all text-sm {}",
                                            if is_active {
                                                "bg-brand/15 text-brand"
                                            } else {
                                                "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                                            }
                                        )
                                    }
                                    on:click=move |_| {
                                        set_active_channel.set(channel_for_click.clone());
                                        set_show_sidebar.set(false);
                                    }
                                >
                                    <span class="text-foreground/40 font-mono text-xs">"#"</span>
                                    <span class="flex-1 font-medium truncate">{channel_display}</span>
                                    {move || {
                                        if active_channel.get() == channel_for_indicator {
                                            view! {
                                                <span class="w-1.5 h-1.5 rounded-full bg-brand animate-pulse"></span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }}
                                </button>
                            }
                        }).collect_view()}
                    </nav>

                    // Network status
                    <div class="mt-4 pt-4 border-t border-border/30">
                        <div class="flex items-center justify-between text-[10px]">
                            <span class="text-foreground/40 uppercase tracking-wider">"Status"</span>
                            <span class="flex items-center gap-1 text-emerald-400">
                                <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                                "Online"
                            </span>
                        </div>
                    </div>
                </div>
            </aside>

            // Main Content - Posts Feed
            <main class="flex-1 overflow-hidden flex flex-col min-w-0">
                // Compact header with channel selector
                <header class="flex-shrink-0 flex items-center gap-2 px-2 py-1.5 border-b border-border/50 bg-panel/30">
                    // Hamburger menu (mobile)
                    <button
                        class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors lg:hidden"
                        on:click=move |_| set_show_sidebar.set(true)
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                    </button>

                    // Channel selector dropdown
                    <div class="relative flex-1 min-w-0">
                        <button
                            class="flex items-center gap-2 px-2 py-1 rounded-lg hover:bg-foreground/5 transition-colors"
                            on:click=move |_| set_show_channel_dropdown.update(|v| *v = !*v)
                        >
                            <span class="font-semibold text-sm text-brand">
                                "# "{move || active_channel.get()}
                            </span>
                            <svg class=move || format!(
                                "w-4 h-4 text-foreground/40 transition-transform {}",
                                if show_channel_dropdown.get() { "rotate-180" } else { "" }
                            ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>

                        // Dropdown
                        {move || {
                            if show_channel_dropdown.get() {
                                view! {
                                    <div class="absolute top-full left-0 mt-1 w-48 bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-1">
                                        {channels_for_dropdown.iter().map(|channel| {
                                            let channel_id_for_class = channel.clone();
                                            let channel_id_for_click = channel.clone();
                                            let channel_name = channel.clone();
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "w-full flex items-center gap-2 px-3 py-2 text-sm transition-colors {}",
                                                        if active_channel.get() == channel_id_for_class {
                                                            "bg-brand/10 text-brand"
                                                        } else {
                                                            "text-foreground/70 hover:bg-foreground/5"
                                                        }
                                                    )
                                                    on:click=move |_| {
                                                        set_active_channel.set(channel_id_for_click.clone());
                                                        set_show_channel_dropdown.set(false);
                                                    }
                                                >
                                                    <span class="text-foreground/40">"#"</span>
                                                    <span class="flex-1 text-left">{channel_name}</span>
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </div>

                    // Post count
                    <span class="hidden sm:block px-2 py-0.5 rounded-full bg-foreground/5 text-foreground/50 text-xs">
                        {move || posts.get().map(|p| p.len()).unwrap_or(0)}" posts"
                    </span>

                    // Actions
                    <div class="flex items-center gap-1">
                        <button class="p-1.5 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"></path>
                            </svg>
                        </button>
                        <button class="p-1.5 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors">
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                            </svg>
                        </button>
                    </div>
                </header>

                // Posts List
                <div class="flex-1 overflow-y-auto scrollbar-styled">
                    <Suspense fallback=move || view! {
                        <div class="p-3 space-y-3">
                            {(0..3).map(|_| view! {
                                <div class="animate-pulse bg-panel rounded-xl p-4 border border-border/30">
                                    <div class="flex items-start gap-3">
                                        <div class="w-10 h-10 rounded-full bg-foreground/10"></div>
                                        <div class="flex-1 space-y-2">
                                            <div class="h-3 bg-foreground/10 rounded w-1/4"></div>
                                            <div class="h-2 bg-foreground/5 rounded w-1/3"></div>
                                        </div>
                                    </div>
                                    <div class="mt-3 space-y-2">
                                        <div class="h-3 bg-foreground/10 rounded w-full"></div>
                                        <div class="h-3 bg-foreground/5 rounded w-2/3"></div>
                                    </div>
                                </div>
                            }).collect_view()}
                        </div>
                    }>
                        {move || posts.get().map(|posts_list| {
                            if posts_list.is_empty() {
                                view! {
                                    <div class="flex flex-col items-center justify-center h-48 text-center p-4">
                                        <div class="w-12 h-12 rounded-full bg-foreground/5 flex items-center justify-center mb-3">
                                            <svg class="w-6 h-6 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"></path>
                                            </svg>
                                        </div>
                                        <h3 class="text-foreground/70 font-medium text-sm mb-1">"No posts yet"</h3>
                                        <p class="text-foreground/40 text-xs">"Be the first to post!"</p>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="p-2 sm:p-3 space-y-2 sm:space-y-3">
                                        {posts_list.iter().rev().map(|post| view! {
                                            <PostCard
                                                id=post.id.clone()
                                                author_public_key=post.author_public_key.clone()
                                                author_name=post.author_name.clone()
                                                author_handle=post.author_handle.clone()
                                                author_avatar=post.author_avatar.clone()
                                                timestamp=post.timestamp.clone()
                                                content=post.content.clone()
                                                posted_in=post.posted_in.clone()
                                                likes=post.likes
                                                comments=post.comments
                                                liked=post.liked
                                            />
                                        }).collect_view()}
                                    </div>
                                }.into_any()
                            }
                        })}
                    </Suspense>
                </div>

                // Compose bar
                <ComposeBar
                    channel=active_channel
                    on_post_created=on_post_created
                />
            </main>
        </div>
    }
}
