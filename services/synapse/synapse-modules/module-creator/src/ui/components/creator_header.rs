// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{Creator, CreatorStats, SubscriptionStatus};
use leptos::prelude::*;
use leptos_router::components::A;

/// Creator profile header with banner, avatar, bio, and stats
#[component]
pub fn CreatorHeader(
    /// The creator's profile data
    creator: Creator,
    /// The creator's stats
    stats: CreatorStats,
    /// User's subscription status
    subscription_status: ReadSignal<SubscriptionStatus>,
    /// Callback when subscribe button is clicked
    on_subscribe: Callback<()>,
) -> impl IntoView {
    let (show_full_bio, set_show_full_bio) = signal(false);
    let (is_following, set_is_following) = signal(false);

    // Format large numbers
    let format_number = |n: u32| -> String {
        if n >= 1_000_000 {
            format!("{:.1}M", n as f64 / 1_000_000.0)
        } else if n >= 1_000 {
            format!("{:.1}K", n as f64 / 1_000.0)
        } else {
            n.to_string()
        }
    };

    let subscribers_formatted = format_number(stats.total_subscribers);
    let posts_formatted = format_number(stats.total_posts);
    let likes_formatted = format_number(stats.total_likes);

    // Clone values needed in closures
    let creator_handle = creator.handle.clone();
    let creator_name = creator.display_name.clone();
    let creator_bio = creator.bio.clone();
    let creator_bio_len = creator.bio.len();
    let creator_bio_truncated = if creator_bio_len > 200 {
        format!("{}...", &creator.bio[..200])
    } else {
        creator.bio.clone()
    };
    let creator_avatar = creator.avatar_url.clone();
    let creator_banner = creator.banner_url.clone();
    let creator_location = creator.location.clone();
    let creator_website = creator.website.clone();
    let creator_category = creator.category.clone();
    let creator_verified = creator.is_verified;
    let creator_tags = creator.tags.clone();
    let social_links = creator.social_links.clone();

    view! {
        <div class="relative">
            // Banner Image
            <div class="relative h-48 sm:h-64 lg:h-80 overflow-hidden">
                {if let Some(banner_url) = creator_banner {
                    view! {
                        <img
                            src=banner_url
                            alt="Creator banner"
                            class="w-full h-full object-cover"
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="w-full h-full bg-gradient-to-br from-brand/30 via-background to-brand/10"></div>
                    }.into_any()
                }}

                // Gradient overlay
                <div class="absolute inset-0 bg-gradient-to-t from-background via-background/20 to-transparent"></div>

                // Top bar with back button and actions
                <div class="absolute top-0 left-0 right-0 p-4 flex items-center justify-between">
                    <button class="p-2 rounded-full bg-background/50 backdrop-blur-sm text-foreground hover:bg-background/70 transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                        </svg>
                    </button>

                    <div class="flex items-center gap-2">
                        <button class="p-2 rounded-full bg-background/50 backdrop-blur-sm text-foreground hover:bg-background/70 transition-colors" title="Share">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                            </svg>
                        </button>
                        <button class="p-2 rounded-full bg-background/50 backdrop-blur-sm text-foreground hover:bg-background/70 transition-colors" title="More options">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>

                // Category badge
                <div class="absolute top-4 left-1/2 -translate-x-1/2">
                    <span class="px-3 py-1 rounded-full bg-brand/80 backdrop-blur-sm text-white text-xs font-medium">
                        {creator_category}
                    </span>
                </div>
            </div>

            // Profile Info Section
            <div class="relative px-4 sm:px-6 lg:px-8 -mt-16 sm:-mt-20 pb-6">
                <div class="flex flex-col sm:flex-row items-start gap-4 sm:gap-6">
                    // Avatar
                    <div class="relative flex-shrink-0">
                        {if let Some(avatar_url) = creator_avatar {
                            view! {
                                <img
                                    src=avatar_url
                                    alt=format!("{}'s avatar", creator_name)
                                    class="w-28 h-28 sm:w-36 sm:h-36 rounded-2xl object-cover border-4 border-background shadow-xl"
                                />
                            }.into_any()
                        } else {
                            let initials: String = creator_name
                                .split_whitespace()
                                .take(2)
                                .filter_map(|s| s.chars().next())
                                .collect();
                            view! {
                                <div class="w-28 h-28 sm:w-36 sm:h-36 rounded-2xl bg-gradient-to-br from-brand to-brand/60 border-4 border-background shadow-xl flex items-center justify-center">
                                    <span class="text-white font-bold text-4xl">{initials}</span>
                                </div>
                            }.into_any()
                        }}

                        // Online indicator
                        <div class="absolute -bottom-1 -right-1 w-6 h-6 bg-emerald-500 rounded-full border-4 border-background"></div>
                    </div>

                    // Info Column
                    <div class="flex-1 min-w-0 pt-4 sm:pt-8">
                        // Name and verification
                        <div class="flex flex-wrap items-center gap-2 mb-1">
                            <h1 class="text-2xl sm:text-3xl font-bold text-foreground">{creator_name.clone()}</h1>
                            {if creator_verified {
                                view! {
                                    <span class="flex items-center gap-1 px-2 py-0.5 rounded-full bg-brand/10 text-brand text-xs font-medium" title="Verified Creator">
                                        <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                            <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
                                        </svg>
                                        "Verified"
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>

                        // Handle
                        <p class="text-foreground/60 text-sm sm:text-base mb-3">"@"{creator_handle}</p>

                        // Location and Website
                        <div class="flex flex-wrap items-center gap-4 text-sm text-foreground/50 mb-4">
                            {if let Some(location) = creator_location {
                                view! {
                                    <span class="flex items-center gap-1">
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                        </svg>
                                        {location}
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                            {if let Some(website) = creator_website {
                                view! {
                                    <a href=website.clone() target="_blank" class="flex items-center gap-1 text-brand hover:underline">
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path>
                                        </svg>
                                        {website.replace("https://", "").replace("http://", "")}
                                    </a>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>

                        // Social Links
                        {if !social_links.is_empty() {
                            view! {
                                <div class="flex flex-wrap items-center gap-2 mb-4">
                                    {social_links.iter().map(|link| {
                                        let url = link.url.clone();
                                        let platform = link.platform.clone();
                                        view! {
                                            <a
                                                href=url
                                                target="_blank"
                                                class="p-2 rounded-lg bg-foreground/5 hover:bg-foreground/10 text-foreground/60 hover:text-foreground transition-colors"
                                                title=platform.clone()
                                            >
                                                {get_social_icon(&platform)}
                                            </a>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Bio
                        <div class="mb-4">
                            <p class="text-foreground/80 text-sm sm:text-base whitespace-pre-line leading-relaxed">
                                {move || if show_full_bio.get() || creator_bio_len <= 200 {
                                    creator_bio.clone()
                                } else {
                                    creator_bio_truncated.clone()
                                }}
                            </p>
                            {if creator_bio_len > 200 {
                                view! {
                                    <button
                                        class="text-brand text-sm hover:underline mt-1"
                                        on:click=move |_| set_show_full_bio.update(|v| *v = !*v)
                                    >
                                        {move || if show_full_bio.get() { "Show less" } else { "Show more" }}
                                    </button>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>

                        // Tags
                        {if !creator_tags.is_empty() {
                            view! {
                                <div class="flex flex-wrap gap-2 mb-4">
                                    {creator_tags.iter().map(|tag| {
                                        let tag = tag.clone();
                                        view! {
                                            <span class="px-2 py-1 rounded-md bg-foreground/5 text-foreground/60 text-xs font-medium">
                                                "#"{tag}
                                            </span>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    // Action Buttons (desktop)
                    <div class="hidden sm:flex flex-col gap-2 pt-8">
                        {move || {
                            match subscription_status.get() {
                                SubscriptionStatus::NotSubscribed => view! {
                                    <button
                                        class="px-6 py-2.5 bg-brand hover:bg-brand/90 text-white font-semibold rounded-xl shadow-lg shadow-brand/20 transition-all active:scale-95"
                                        on:click=move |_| on_subscribe.run(())
                                    >
                                        "Subscribe"
                                    </button>
                                }.into_any(),
                                SubscriptionStatus::Subscribed { .. } => view! {
                                    <button class="px-6 py-2.5 bg-brand/20 text-brand font-semibold rounded-xl border border-brand/30 transition-all hover:bg-brand/30">
                                        "✓ Subscribed"
                                    </button>
                                }.into_any(),
                                SubscriptionStatus::Expired { .. } => view! {
                                    <button
                                        class="px-6 py-2.5 bg-amber-500 hover:bg-amber-500/90 text-white font-semibold rounded-xl shadow-lg shadow-amber-500/20 transition-all active:scale-95"
                                        on:click=move |_| on_subscribe.run(())
                                    >
                                        "Renew"
                                    </button>
                                }.into_any(),
                            }
                        }}
                        <button
                            class=move || format!(
                                "px-6 py-2.5 font-semibold rounded-xl border transition-all {}",
                                if is_following.get() {
                                    "bg-foreground/5 text-foreground border-border/50 hover:bg-foreground/10"
                                } else {
                                    "bg-transparent text-foreground/70 border-border/50 hover:bg-foreground/5"
                                }
                            )
                            on:click=move |_| set_is_following.update(|v| *v = !*v)
                        >
                            {move || if is_following.get() { "Following" } else { "Follow" }}
                        </button>
                    </div>
                </div>

                // Stats Bar
                <div class="mt-6 grid grid-cols-3 sm:grid-cols-4 gap-4">
                    <StatCard label="Subscribers" value=subscribers_formatted.clone() icon="users" highlight=true />
                    <StatCard label="Posts" value=posts_formatted.clone() icon="posts" highlight=false />
                    <StatCard label="Likes" value=likes_formatted.clone() icon="heart" highlight=false />
                    <div class="hidden sm:block">
                        <StatCard label="Engagement" value=format!("{:.1}%", stats.engagement_rate) icon="chart" highlight=false />
                    </div>
                </div>

                // Mobile Action Buttons
                <div class="sm:hidden flex gap-2 mt-4">
                    {move || {
                        match subscription_status.get() {
                            SubscriptionStatus::NotSubscribed => view! {
                                <button
                                    class="flex-1 px-4 py-2.5 bg-brand hover:bg-brand/90 text-white font-semibold rounded-xl shadow-lg shadow-brand/20 transition-all active:scale-95"
                                    on:click=move |_| on_subscribe.run(())
                                >
                                    "Subscribe"
                                </button>
                            }.into_any(),
                            SubscriptionStatus::Subscribed { .. } => view! {
                                <button class="flex-1 px-4 py-2.5 bg-brand/20 text-brand font-semibold rounded-xl border border-brand/30">
                                    "✓ Subscribed"
                                </button>
                            }.into_any(),
                            SubscriptionStatus::Expired { .. } => view! {
                                <button
                                    class="flex-1 px-4 py-2.5 bg-amber-500 text-white font-semibold rounded-xl shadow-lg shadow-amber-500/20 transition-all active:scale-95"
                                    on:click=move |_| on_subscribe.run(())
                                >
                                    "Renew"
                                </button>
                            }.into_any(),
                        }
                    }}
                    <button
                        class=move || format!(
                            "px-4 py-2.5 font-semibold rounded-xl border transition-all {}",
                            if is_following.get() {
                                "bg-foreground/5 text-foreground border-border/50"
                            } else {
                                "text-foreground/70 border-border/50"
                            }
                        )
                        on:click=move |_| set_is_following.update(|v| *v = !*v)
                    >
                        {move || if is_following.get() { "Following" } else { "Follow" }}
                    </button>
                </div>
            </div>

            // Decorative elements
            <div class="absolute top-1/2 left-0 w-32 h-32 bg-brand/5 rounded-full blur-3xl pointer-events-none"></div>
            <div class="absolute top-1/3 right-0 w-48 h-48 bg-brand/3 rounded-full blur-3xl pointer-events-none"></div>
        </div>
    }
}

/// Individual stat card
#[component]
fn StatCard(
    label: &'static str,
    value: String,
    icon: &'static str,
    highlight: bool,
) -> impl IntoView {
    view! {
        <div class=format!(
            "p-3 sm:p-4 rounded-xl border transition-colors {}",
            if highlight {
                "bg-brand/5 border-brand/20"
            } else {
                "bg-foreground/5 border-border/30"
            }
        )>
            <div class="flex items-center gap-2 mb-1">
                {get_stat_icon(icon, highlight)}
                <span class="text-foreground/50 text-xs uppercase tracking-wider">{label}</span>
            </div>
            <p class=format!(
                "text-lg sm:text-xl font-bold {}",
                if highlight { "text-brand" } else { "text-foreground" }
            )>
                {value}
            </p>
        </div>
    }
}

/// Get stat icon SVG
fn get_stat_icon(icon: &str, highlight: bool) -> impl IntoView {
    let color_class = if highlight { "text-brand" } else { "text-foreground/40" };

    match icon {
        "users" => view! {
            <svg class=format!("w-4 h-4 {}", color_class) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
            </svg>
        }.into_any(),
        "posts" => view! {
            <svg class=format!("w-4 h-4 {}", color_class) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
            </svg>
        }.into_any(),
        "heart" => view! {
            <svg class=format!("w-4 h-4 {}", color_class) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
            </svg>
        }.into_any(),
        "chart" => view! {
            <svg class=format!("w-4 h-4 {}", color_class) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"></path>
            </svg>
        }.into_any(),
        _ => view! { <span></span> }.into_any(),
    }
}

/// Get social media icon
fn get_social_icon(platform: &str) -> impl IntoView {
    match platform.to_lowercase().as_str() {
        "twitter" | "x" => view! {
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path>
            </svg>
        }.into_any(),
        "instagram" => view! {
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M12 2.163c3.204 0 3.584.012 4.85.07 3.252.148 4.771 1.691 4.919 4.919.058 1.265.069 1.645.069 4.849 0 3.205-.012 3.584-.069 4.849-.149 3.225-1.664 4.771-4.919 4.919-1.266.058-1.644.07-4.85.07-3.204 0-3.584-.012-4.849-.07-3.26-.149-4.771-1.699-4.919-4.92-.058-1.265-.07-1.644-.07-4.849 0-3.204.013-3.583.07-4.849.149-3.227 1.664-4.771 4.919-4.919 1.266-.057 1.645-.069 4.849-.069zm0-2.163c-3.259 0-3.667.014-4.947.072-4.358.2-6.78 2.618-6.98 6.98-.059 1.281-.073 1.689-.073 4.948 0 3.259.014 3.668.072 4.948.2 4.358 2.618 6.78 6.98 6.98 1.281.058 1.689.072 4.948.072 3.259 0 3.668-.014 4.948-.072 4.354-.2 6.782-2.618 6.979-6.98.059-1.28.073-1.689.073-4.948 0-3.259-.014-3.667-.072-4.947-.196-4.354-2.617-6.78-6.979-6.98-1.281-.059-1.69-.073-4.949-.073zm0 5.838c-3.403 0-6.162 2.759-6.162 6.162s2.759 6.163 6.162 6.163 6.162-2.759 6.162-6.163c0-3.403-2.759-6.162-6.162-6.162zm0 10.162c-2.209 0-4-1.79-4-4 0-2.209 1.791-4 4-4s4 1.791 4 4c0 2.21-1.791 4-4 4zm6.406-11.845c-.796 0-1.441.645-1.441 1.44s.645 1.44 1.441 1.44c.795 0 1.439-.645 1.439-1.44s-.644-1.44-1.439-1.44z"></path>
            </svg>
        }.into_any(),
        "artstation" => view! {
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M0 17.723l2.027 3.505h.001a2.424 2.424 0 0 0 2.164 1.333h13.457l-2.792-4.838H0zm24-3.295-3.455-5.997a2.426 2.426 0 0 0-2.164-1.332H6.635l9.875 17.104 5.018-8.708a2.42 2.42 0 0 0 .472-1.067zM4.019 11.139l6.508 11.301L4.606 7.63 3.206 4.6H0v.001l4.019 6.538z"></path>
            </svg>
        }.into_any(),
        "youtube" => view! {
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"></path>
            </svg>
        }.into_any(),
        _ => view! {
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path>
            </svg>
        }.into_any(),
    }
}

