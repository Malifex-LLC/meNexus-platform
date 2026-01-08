// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{SubscriptionStatus, SubscriptionTier, Supporter};
use crate::ui::components::{SupportersList, TierCard};
use leptos::prelude::*;

/// Subscription sidebar showing tiers and supporters
#[component]
pub fn SubscriptionSidebar(
    /// Available subscription tiers
    tiers: Vec<SubscriptionTier>,
    /// Recent supporters
    supporters: Vec<Supporter>,
    /// User's current subscription status
    subscription_status: ReadSignal<SubscriptionStatus>,
    /// Callback to close sidebar (mobile)
    on_close: Callback<()>,
) -> impl IntoView {
    let (selected_tier, set_selected_tier) = signal::<Option<String>>(None);
    let (billing_cycle, set_billing_cycle) = signal(BillingCycle::Monthly);
    let (show_all_tiers, set_show_all_tiers) = signal(false);

    // Get current tier if subscribed
    let current_tier_id = match subscription_status.get() {
        SubscriptionStatus::Subscribed { tier_id } => Some(tier_id),
        _ => None,
    };

    // Calculate selected tier price
    let selected_tier_details = {
        let tiers = tiers.clone();
        Memo::new(move |_| {
            selected_tier.get().and_then(|id| {
                tiers.iter().find(|t| t.id == id).cloned()
            })
        })
    };

    view! {
        <div class="flex flex-col h-full">
            // Header
            <div class="flex items-center justify-between p-4 border-b border-border/50">
                <h2 class="font-bold text-foreground text-lg">"Subscribe"</h2>
                <button
                    class="p-2 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors xl:hidden"
                    on:click=move |_| on_close.run(())
                >
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                    </svg>
                </button>
            </div>

            // Scrollable content
            <div class="flex-1 overflow-y-auto scrollbar-thin">
                <div class="p-4 space-y-6">
                    // Billing cycle toggle
                    <div class="flex items-center justify-center gap-2 p-1 bg-foreground/5 rounded-xl">
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-all {}",
                                if billing_cycle.get() == BillingCycle::Monthly {
                                    "bg-panel text-foreground shadow-sm"
                                } else {
                                    "text-foreground/60 hover:text-foreground"
                                }
                            )
                            on:click=move |_| set_billing_cycle.set(BillingCycle::Monthly)
                        >
                            "Monthly"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 px-4 py-2 rounded-lg text-sm font-medium transition-all relative {}",
                                if billing_cycle.get() == BillingCycle::Yearly {
                                    "bg-panel text-foreground shadow-sm"
                                } else {
                                    "text-foreground/60 hover:text-foreground"
                                }
                            )
                            on:click=move |_| set_billing_cycle.set(BillingCycle::Yearly)
                        >
                            "Yearly"
                            <span class="absolute -top-2 -right-2 px-1.5 py-0.5 bg-emerald-500 text-white text-xs font-bold rounded-full">
                                "-17%"
                            </span>
                        </button>
                    </div>

                    // Tiers list
                    <div class="space-y-3">
                        <div class="flex items-center justify-between">
                            <h3 class="text-sm font-semibold text-foreground/70 uppercase tracking-wider">"Choose a tier"</h3>
                            {if tiers.len() > 3 {
                                view! {
                                    <button
                                        class="text-brand text-sm hover:underline"
                                        on:click=move |_| set_show_all_tiers.update(|v| *v = !*v)
                                    >
                                        {move || if show_all_tiers.get() { "Show less" } else { "Show all" }}
                                    </button>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>

                        {move || {
                            let tiers_to_show = if show_all_tiers.get() {
                                tiers.clone()
                            } else {
                                tiers.iter().take(3).cloned().collect()
                            };

                            tiers_to_show.into_iter().map(|tier| {
                                let tier_id = tier.id.clone();
                                let is_current = current_tier_id.as_ref() == Some(&tier_id);
                                let is_selected = selected_tier.get().as_ref() == Some(&tier_id);

                                view! {
                                    <TierCard
                                        tier=tier
                                        is_selected=is_selected
                                        is_current=is_current
                                        on_select=Callback::new(move |id: String| {
                                            set_selected_tier.set(Some(id));
                                        })
                                    />
                                }
                            }).collect_view()
                        }}
                    </div>

                    // Subscribe button
                    {move || {
                        if let Some(tier) = selected_tier_details.get() {
                            let price = if billing_cycle.get() == BillingCycle::Yearly {
                                tier.price_yearly.unwrap_or(tier.price_monthly * 12.0)
                            } else {
                                tier.price_monthly
                            };
                            let period = if billing_cycle.get() == BillingCycle::Yearly { "year" } else { "month" };

                            view! {
                                <div class="sticky bottom-0 pt-4 pb-2 bg-gradient-to-t from-panel via-panel to-transparent">
                                    <button class="w-full flex items-center justify-center gap-2 px-6 py-3 bg-brand hover:bg-brand/90 text-white font-semibold rounded-xl shadow-lg shadow-brand/30 transition-all active:scale-[0.98]">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                                        </svg>
                                        "Subscribe for $"{format!("{:.0}", price)}"/" {period}
                                    </button>
                                    <p class="text-center text-foreground/40 text-xs mt-2">
                                        "Cancel anytime • Instant access"
                                    </p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="p-4 bg-foreground/5 rounded-xl text-center">
                                    <p class="text-foreground/50 text-sm">"Select a tier to continue"</p>
                                </div>
                            }.into_any()
                        }
                    }}

                    // Divider
                    <div class="flex items-center gap-3">
                        <div class="flex-1 h-px bg-border/30"></div>
                        <span class="text-foreground/40 text-xs">or</span>
                        <div class="flex-1 h-px bg-border/30"></div>
                    </div>

                    // One-time support option
                    <button class="w-full flex items-center justify-center gap-2 px-4 py-3 border border-border/50 rounded-xl text-foreground/70 hover:border-brand/30 hover:text-foreground hover:bg-foreground/5 transition-all">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        "Send a one-time tip"
                    </button>

                    // Recent supporters
                    <div class="pt-4">
                        <SupportersList supporters=supporters.clone() />
                    </div>

                    // Features/guarantees
                    <div class="space-y-3 pt-4 border-t border-border/30">
                        <h3 class="text-sm font-semibold text-foreground/70">"Why subscribe?"</h3>
                        <div class="space-y-2">
                            <FeatureItem icon="lock" text="Unlock exclusive content" />
                            <FeatureItem icon="download" text="Download resources & files" />
                            <FeatureItem icon="chat" text="Direct creator access" />
                            <FeatureItem icon="cancel" text="Cancel anytime" />
                        </div>
                    </div>

                    // Security note
                    <div class="flex items-start gap-2 p-3 bg-foreground/5 rounded-xl text-xs text-foreground/50">
                        <svg class="w-4 h-4 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                        </svg>
                        <p>"Payments are secure and encrypted. Your support goes directly to the creator."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Billing cycle option
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BillingCycle {
    #[default]
    Monthly,
    Yearly,
}

/// Feature item in the sidebar
#[component]
fn FeatureItem(icon: &'static str, text: &'static str) -> impl IntoView {
    let icon_view = match icon {
        "lock" => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z"></path>
            </svg>
        }.into_any(),
        "download" => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
            </svg>
        }.into_any(),
        "chat" => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
            </svg>
        }.into_any(),
        "cancel" => view! {
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        }.into_any(),
        _ => view! { <span class="w-4 h-4"></span> }.into_any(),
    };

    view! {
        <div class="flex items-center gap-2 text-foreground/70 text-sm">
            <span class="text-brand">{icon_view}</span>
            <span>{text}</span>
        </div>
    }
}

