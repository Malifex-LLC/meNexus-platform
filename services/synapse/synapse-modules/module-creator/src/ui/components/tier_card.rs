// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::SubscriptionTier;
use leptos::prelude::*;

/// Subscription tier card component
#[component]
pub fn TierCard(
    /// The subscription tier data
    tier: SubscriptionTier,
    /// Whether this tier is currently selected/active
    #[prop(default = false)]
    is_selected: bool,
    /// Whether user is already subscribed to this tier
    #[prop(default = false)]
    is_current: bool,
    /// Callback when tier is selected
    on_select: Callback<String>,
) -> impl IntoView {
    let tier_id = tier.id.clone();
    let tier_name = tier.name.clone();
    let tier_price = tier.price_monthly;
    let tier_yearly = tier.price_yearly;
    let tier_description = tier.description.clone();
    let tier_benefits = tier.benefits.clone();
    let tier_subscribers = tier.subscriber_count;
    let tier_is_limited = tier.is_limited;
    let tier_limit = tier.limit;
    let tier_badge_color = tier.badge_color.clone();
    let tier_badge_icon = tier.badge_icon.clone();
    let tier_is_featured = tier.is_featured;

    // Calculate yearly savings
    let yearly_savings = tier_yearly.map(|yearly| {
        let monthly_total = tier_price * 12.0;
        let savings = monthly_total - yearly;
        let percent = (savings / monthly_total) * 100.0;
        (savings, percent)
    });

    view! {
        <div
            class=move || format!(
                "relative p-4 rounded-xl border-2 transition-all cursor-pointer group {}",
                if is_current {
                    "bg-brand/5 border-brand ring-2 ring-brand/20"
                } else if is_selected {
                    "bg-foreground/5 border-brand/50 ring-1 ring-brand/30"
                } else if tier_is_featured {
                    "bg-foreground/5 border-border/50 hover:border-brand/30 hover:bg-foreground/10"
                } else {
                    "bg-transparent border-border/30 hover:border-border/50 hover:bg-foreground/5"
                }
            )
            on:click={
                let tier_id = tier_id.clone();
                move |_| on_select.run(tier_id.clone())
            }
        >
            // Featured badge
            {if tier_is_featured {
                view! {
                    <div class="absolute -top-3 left-1/2 -translate-x-1/2">
                        <span class="px-3 py-1 bg-brand text-white text-xs font-bold rounded-full shadow-lg shadow-brand/30">
                            "MOST POPULAR"
                        </span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Limited badge
            {if tier_is_limited {
                if let Some(limit) = tier_limit {
                    let remaining = limit.saturating_sub(tier_subscribers);
                    view! {
                        <div class="absolute -top-3 right-3">
                            <span class=format!(
                                "px-2 py-1 text-xs font-bold rounded-full {}",
                                if remaining < 5 { "bg-rose-500/20 text-rose-400" } else { "bg-amber-500/20 text-amber-400" }
                            )>
                                {remaining}" spots left"
                            </span>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            } else {
                view! { <span></span> }.into_any()
            }}

            // Header
            <div class="flex items-start justify-between gap-2 mb-3">
                <div class="flex items-center gap-2">
                    // Tier badge/icon
                    <div
                        class="w-10 h-10 rounded-lg flex items-center justify-center text-lg"
                        style=format!("background: {}20; color: {}", tier_badge_color, tier_badge_color)
                    >
                        {tier_badge_icon.clone().unwrap_or_else(|| "★".to_string())}
                    </div>
                    <div>
                        <h3 class="font-bold text-foreground">{tier_name}</h3>
                        <p class="text-xs text-foreground/50">{tier_subscribers}" subscribers"</p>
                    </div>
                </div>

                // Selection indicator
                {if is_current {
                    view! {
                        <span class="px-2 py-1 bg-brand text-white text-xs font-medium rounded-full">
                            "Current"
                        </span>
                    }.into_any()
                } else {
                    view! {
                        <div class=move || format!(
                            "w-5 h-5 rounded-full border-2 transition-all {}",
                            if is_selected {
                                "border-brand bg-brand"
                            } else {
                                "border-border/50 group-hover:border-brand/50"
                            }
                        )>
                            {move || if is_selected {
                                view! {
                                    <svg class="w-full h-full text-white p-0.5" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
                                    </svg>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    }.into_any()
                }}
            </div>

            // Price
            <div class="mb-3">
                <div class="flex items-baseline gap-1">
                    <span class="text-2xl font-bold text-foreground">"$"{format!("{:.0}", tier_price)}</span>
                    <span class="text-foreground/50 text-sm">"/month"</span>
                </div>
                {if let Some((_savings, percent)) = yearly_savings {
                    view! {
                        <p class="text-emerald-400 text-xs mt-1">
                            "Save "{format!("{:.0}%", percent)}" with yearly (${:.0}/yr)"
                            // Workaround: Use string interpolation properly
                        </p>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Description
            <p class="text-foreground/60 text-sm mb-4">{tier_description}</p>

            // Benefits list
            <ul class="space-y-2">
                {tier_benefits.iter().map(|benefit| {
                    let benefit = benefit.clone();
                    let is_inherited = benefit.starts_with("Everything in");
                    view! {
                        <li class="flex items-start gap-2 text-sm">
                            {if is_inherited {
                                view! {
                                    <svg class="w-4 h-4 text-foreground/40 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18"></path>
                                    </svg>
                                }.into_any()
                            } else {
                                view! {
                                    <svg class="w-4 h-4 text-brand flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                }.into_any()
                            }}
                            <span class=if is_inherited { "text-foreground/50 italic" } else { "text-foreground/80" }>
                                {benefit}
                            </span>
                        </li>
                    }
                }).collect_view()}
            </ul>

            // Hover effect
            <div class="absolute inset-0 rounded-xl bg-gradient-to-t from-brand/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
        </div>
    }
}

/// Compact tier card for sidebar display
#[component]
pub fn TierCardCompact(
    /// The subscription tier data
    tier: SubscriptionTier,
    /// Whether user is subscribed to this tier
    #[prop(default = false)]
    is_current: bool,
    /// Callback when selected
    on_select: Callback<String>,
) -> impl IntoView {
    let tier_id = tier.id.clone();
    let tier_name = tier.name.clone();
    let tier_price = tier.price_monthly;
    let tier_badge_color = tier.badge_color.clone();
    let tier_badge_icon = tier.badge_icon.clone();
    let tier_is_featured = tier.is_featured;
    let tier_subscribers = tier.subscriber_count;

    view! {
        <button
            class=move || format!(
                "w-full flex items-center gap-3 p-3 rounded-xl border transition-all text-left {}",
                if is_current {
                    "bg-brand/10 border-brand/30"
                } else if tier_is_featured {
                    "bg-foreground/5 border-brand/20 hover:border-brand/40"
                } else {
                    "bg-transparent border-border/30 hover:border-border/50 hover:bg-foreground/5"
                }
            )
            on:click={
                let tier_id = tier_id.clone();
                move |_| on_select.run(tier_id.clone())
            }
        >
            // Badge icon
            <div
                class="w-10 h-10 rounded-lg flex items-center justify-center text-sm flex-shrink-0"
                style=format!("background: {}20; color: {}", tier_badge_color, tier_badge_color)
            >
                {tier_badge_icon.clone().unwrap_or_else(|| "★".to_string())}
            </div>

            // Info
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2">
                    <span class="font-semibold text-foreground text-sm truncate">{tier_name}</span>
                    {if tier_is_featured {
                        view! {
                            <span class="px-1.5 py-0.5 bg-brand/20 text-brand text-xs font-medium rounded">
                                "Popular"
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <p class="text-foreground/50 text-xs">{tier_subscribers}" subscribers"</p>
            </div>

            // Price
            <div class="text-right flex-shrink-0">
                <p class="font-bold text-foreground">"$"{format!("{:.0}", tier_price)}</p>
                <p class="text-foreground/40 text-xs">"/mo"</p>
            </div>

            // Current indicator
            {if is_current {
                view! {
                    <div class="absolute top-2 right-2">
                        <svg class="w-4 h-4 text-brand" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
                        </svg>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </button>
    }
}

