// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::Supporter;
use leptos::prelude::*;
use leptos_router::components::A;

/// List of recent supporters/subscribers
#[component]
pub fn SupportersList(
    /// List of supporters to display
    supporters: Vec<Supporter>,
    /// Maximum number to show initially
    #[prop(default = 5)]
    max_initial: usize,
) -> impl IntoView {
    let (show_all, set_show_all) = signal(false);

    let supporters_to_show = {
        let supporters = supporters.clone();
        let max = max_initial;
        Memo::new(move |_| {
            if show_all.get() {
                supporters.clone()
            } else {
                supporters.iter().take(max).cloned().collect()
            }
        })
    };

    let total_supporters = supporters.len();
    let new_supporters: usize = supporters.iter().filter(|s| s.is_new).count();

    view! {
        <div class="space-y-3">
            // Header
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <h3 class="text-sm font-semibold text-foreground/70 uppercase tracking-wider">"Recent Supporters"</h3>
                    {if new_supporters > 0 {
                        view! {
                            <span class="px-1.5 py-0.5 bg-emerald-500/20 text-emerald-400 text-xs font-medium rounded-full">
                                "+"{new_supporters}" new"
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                {if total_supporters > max_initial {
                    view! {
                        <button
                            class="text-brand text-sm hover:underline"
                            on:click=move |_| set_show_all.update(|v| *v = !*v)
                        >
                            {move || if show_all.get() {
                                "Show less".to_string()
                            } else {
                                format!("View all ({})", total_supporters)
                            }}
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Supporters list
            <div class="space-y-2">
                {move || {
                    supporters_to_show.get().into_iter().map(|supporter| {
                        view! { <SupporterCard supporter=supporter /> }
                    }).collect_view()
                }}
            </div>

            // Empty state
            {if supporters.is_empty() {
                view! {
                    <div class="p-6 text-center">
                        <div class="w-12 h-12 rounded-full bg-foreground/5 flex items-center justify-center mx-auto mb-3">
                            <svg class="w-6 h-6 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path>
                            </svg>
                        </div>
                        <p class="text-foreground/50 text-sm">"Be the first to support this creator!"</p>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

/// Individual supporter card
#[component]
fn SupporterCard(supporter: Supporter) -> impl IntoView {
    let handle = supporter.handle.clone();
    let handle_for_info = supporter.handle.clone();
    let display_name = supporter.display_name.clone();
    let display_name_for_alt = supporter.display_name.clone();
    let display_name_for_link = supporter.display_name.clone();
    let avatar_url = supporter.avatar_url.clone();
    let tier_name = supporter.tier.name.clone();
    let tier_color = supporter.tier.badge_color.clone();
    let tier_icon = supporter.tier.badge_icon.clone();
    let message = supporter.message.clone();
    let is_new = supporter.is_new;
    let total_support = supporter.total_support;

    // Generate initials for avatar fallback
    let initials: String = display_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    view! {
        <div class=format!(
            "relative p-3 rounded-xl border transition-all hover:bg-foreground/5 {}",
            if is_new { "bg-emerald-500/5 border-emerald-500/20" } else { "bg-transparent border-border/30" }
        )>
            // New badge
            {if is_new {
                view! {
                    <div class="absolute -top-1.5 -right-1.5">
                        <span class="flex h-3 w-3">
                            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                            <span class="relative inline-flex rounded-full h-3 w-3 bg-emerald-500"></span>
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            <div class="flex items-start gap-3">
                // Avatar
                <A href=format!("/profiles/{}", handle) attr:class="flex-shrink-0">
                    {if let Some(url) = avatar_url {
                        view! {
                            <img
                                src=url
                                alt=format!("{}'s avatar", display_name_for_alt)
                                class="w-10 h-10 rounded-full object-cover ring-2 ring-border/30"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-2 ring-border/30 flex items-center justify-center">
                                <span class="text-brand font-bold text-sm">{initials.clone()}</span>
                            </div>
                        }.into_any()
                    }}
                </A>

                // Info
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                        <A href=format!("/profiles/{}", handle_for_info) attr:class="font-semibold text-foreground text-sm hover:underline truncate">
                            {display_name_for_link}
                        </A>
                        // Tier badge
                        <span
                            class="flex items-center gap-0.5 px-1.5 py-0.5 rounded text-xs font-medium"
                            style=format!("background: {}20; color: {}", tier_color, tier_color)
                        >
                            {tier_icon.clone().unwrap_or_default()}
                            <span class="hidden sm:inline ml-0.5">{tier_name.clone()}</span>
                        </span>
                    </div>

                    // Support amount
                    <p class="text-foreground/50 text-xs mt-0.5">
                        "Total support: "
                        <span class="text-foreground/70 font-medium">"$"{format!("{:.0}", total_support)}</span>
                    </p>

                    // Message (if any)
                    {if let Some(msg) = message {
                        view! {
                            <div class="mt-2 p-2 bg-foreground/5 rounded-lg">
                                <p class="text-foreground/70 text-sm italic">"\""{ msg }"\""</p>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Compact supporter display for leaderboard/wall
#[component]
pub fn SupporterWall(
    /// List of top supporters
    supporters: Vec<Supporter>,
    /// Title for the wall
    #[prop(default = "Top Supporters")]
    title: &'static str,
) -> impl IntoView {
    view! {
        <div class="space-y-3">
            <h3 class="text-sm font-semibold text-foreground/70 uppercase tracking-wider">{title}</h3>

            <div class="grid grid-cols-5 sm:grid-cols-6 md:grid-cols-8 gap-2">
                {supporters.iter().take(16).map(|supporter| {
                    let handle = supporter.handle.clone();
                    let display_name = supporter.display_name.clone();
                    let display_name_for_title = supporter.display_name.clone();
                    let display_name_for_alt = supporter.display_name.clone();
                    let avatar_url = supporter.avatar_url.clone();
                    let tier_color = supporter.tier.badge_color.clone();
                    let tier_color_for_dot = supporter.tier.badge_color.clone();

                    let initials: String = display_name
                        .split_whitespace()
                        .take(2)
                        .filter_map(|s| s.chars().next())
                        .collect::<String>()
                        .to_uppercase();

                    view! {
                        <A
                            href=format!("/profiles/{}", handle)
                            attr:class="group relative"
                            attr:title=display_name_for_title
                        >
                            {if let Some(url) = avatar_url {
                                view! {
                                    <img
                                        src=url
                                        alt=format!("{}'s avatar", display_name_for_alt)
                                        class="w-full aspect-square rounded-lg object-cover ring-2 transition-all group-hover:ring-brand/50 group-hover:scale-105"
                                        style=format!("--tw-ring-color: {}40", tier_color)
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <div
                                        class="w-full aspect-square rounded-lg bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 transition-all group-hover:ring-brand/50 group-hover:scale-105"
                                        style=format!("--tw-ring-color: {}40", tier_color)
                                    >
                                        <span class="text-brand font-bold text-xs">{initials.clone()}</span>
                                    </div>
                                }.into_any()
                            }}
                            // Tier indicator dot
                            <div
                                class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-panel"
                                style=format!("background: {}", tier_color_for_dot)
                            ></div>
                        </A>
                    }
                }).collect_view()}
            </div>

            {if supporters.len() > 16 {
                view! {
                    <button class="w-full py-2 text-brand text-sm hover:underline">
                        "View all "{supporters.len()}" supporters"
                    </button>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

