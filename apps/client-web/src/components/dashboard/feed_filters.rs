// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{ActiveSynapse, ContentType, SortOrder, TimeRange};
use leptos::prelude::*;

/// Advanced feed filtering component
#[component]
pub fn FeedFilters(
    #[prop(into)] sort_order: Signal<SortOrder>,
    #[prop(into)] on_sort_change: Callback<SortOrder>,
) -> impl IntoView {
    let (show_advanced, set_show_advanced) = signal(false);
    let (selected_content_type, set_selected_content_type) = signal(ContentType::All);
    let (selected_time_range, set_selected_time_range) = signal(TimeRange::AllTime);
    let (selected_synapse, set_selected_synapse) = signal::<Option<String>>(None);
    let (hide_seen, set_hide_seen) = signal(false);
    let (only_verified, set_only_verified) = signal(false);

    let synapses = ActiveSynapse::mock_synapses();

    let sort_options = [
        SortOrder::Latest,
        SortOrder::Trending,
        SortOrder::MostLiked,
        SortOrder::MostCommented,
        SortOrder::Following,
    ];

    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Main filter bar
            <div class="p-3 flex items-center gap-2 flex-wrap">
                // Sort order tabs
                <div class="flex items-center gap-1 p-1 bg-foreground/5 rounded-xl">
                    {sort_options.iter().map(|option| {
                        let opt = option.clone();
                        let opt_for_class = option.clone();
                        view! {
                            <button
                                class=move || format!(
                                    "flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                                    if sort_order.get() == opt_for_class {
                                        "bg-brand text-white shadow-sm"
                                    } else {
                                        "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                                    }
                                )
                                on:click=move |_| on_sort_change.run(opt.clone())
                            >
                                <span class="w-4 h-4 hidden sm:block" inner_html=option.icon_svg()></span>
                                <span class="hidden md:inline">{option.label()}</span>
                                <span class="md:hidden">{option.label().chars().take(3).collect::<String>()}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>

                <div class="flex-1"></div>

                // Quick filters
                <div class="flex items-center gap-2">
                    // Active filters count
                    {move || {
                        let count = (selected_content_type.get() != ContentType::All) as u8
                            + (selected_time_range.get() != TimeRange::AllTime) as u8
                            + selected_synapse.get().is_some() as u8
                            + hide_seen.get() as u8
                            + only_verified.get() as u8;
                        
                        if count > 0 {
                            view! {
                                <span class="px-2 py-0.5 rounded-full bg-brand/15 text-brand text-xs font-medium">
                                    {count}" active"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    // Advanced toggle
                    <button
                        class=move || format!(
                            "flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                            if show_advanced.get() {
                                "bg-brand/15 text-brand"
                            } else {
                                "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                            }
                        )
                        on:click=move |_| set_show_advanced.update(|v| *v = !*v)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>
                        </svg>
                        <span class="hidden sm:inline">"Filters"</span>
                    </button>

                    // Refresh button
                    <button class="p-2 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors" title="Refresh feed">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                        </svg>
                    </button>
                </div>
            </div>

            // Advanced filters panel
            {move || {
                if show_advanced.get() {
                    let synapses_list = synapses.clone();
                    view! {
                        <div class="px-4 pb-4 pt-0 space-y-4 border-t border-border/30 mt-1">
                            <div class="pt-4 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                                // Content type filter
                                <div>
                                    <label class="block text-xs font-medium text-foreground/50 mb-2">"Content Type"</label>
                                    <div class="flex flex-wrap gap-1">
                                        {[ContentType::All, ContentType::Posts, ContentType::Media, ContentType::Links, ContentType::Polls].iter().map(|ct| {
                                            let ct_val = ct.clone();
                                            let ct_class = ct.clone();
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "px-2 py-1 rounded-md text-xs font-medium transition-all {}",
                                                        if selected_content_type.get() == ct_class {
                                                            "bg-brand/15 text-brand"
                                                        } else {
                                                            "bg-foreground/5 text-foreground/50 hover:text-foreground"
                                                        }
                                                    )
                                                    on:click=move |_| set_selected_content_type.set(ct_val.clone())
                                                >
                                                    {ct.label()}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>

                                // Time range filter
                                <div>
                                    <label class="block text-xs font-medium text-foreground/50 mb-2">"Time Range"</label>
                                    <select
                                        class="w-full px-3 py-1.5 bg-foreground/5 border border-border/50 rounded-lg text-sm text-foreground focus:outline-none focus:border-brand/50"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            let tr = match val.as_str() {
                                                "hour" => TimeRange::LastHour,
                                                "today" => TimeRange::Today,
                                                "week" => TimeRange::ThisWeek,
                                                "month" => TimeRange::ThisMonth,
                                                _ => TimeRange::AllTime,
                                            };
                                            set_selected_time_range.set(tr);
                                        }
                                    >
                                        <option value="all">"All time"</option>
                                        <option value="hour">"Last hour"</option>
                                        <option value="today">"Today"</option>
                                        <option value="week">"This week"</option>
                                        <option value="month">"This month"</option>
                                    </select>
                                </div>

                                // Synapse filter
                                <div>
                                    <label class="block text-xs font-medium text-foreground/50 mb-2">"Synapse"</label>
                                    <select
                                        class="w-full px-3 py-1.5 bg-foreground/5 border border-border/50 rounded-lg text-sm text-foreground focus:outline-none focus:border-brand/50"
                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            set_selected_synapse.set(if val == "all" { None } else { Some(val) });
                                        }
                                    >
                                        <option value="all">"All Synapses"</option>
                                        {synapses_list.iter().map(|syn| {
                                            view! {
                                                <option value=syn.id.clone()>{syn.name.clone()}</option>
                                            }
                                        }).collect_view()}
                                    </select>
                                </div>

                                // Toggles
                                <div class="space-y-2">
                                    <label class="block text-xs font-medium text-foreground/50 mb-2">"Options"</label>
                                    
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input
                                            type="checkbox"
                                            class="w-4 h-4 rounded border-border/50 text-brand focus:ring-brand/50 bg-foreground/5"
                                            prop:checked=move || hide_seen.get()
                                            on:change=move |ev| set_hide_seen.set(event_target_checked(&ev))
                                        />
                                        <span class="text-sm text-foreground/70">"Hide seen"</span>
                                    </label>
                                    
                                    <label class="flex items-center gap-2 cursor-pointer">
                                        <input
                                            type="checkbox"
                                            class="w-4 h-4 rounded border-border/50 text-brand focus:ring-brand/50 bg-foreground/5"
                                            prop:checked=move || only_verified.get()
                                            on:change=move |ev| set_only_verified.set(event_target_checked(&ev))
                                        />
                                        <span class="text-sm text-foreground/70">"Verified only"</span>
                                    </label>
                                </div>
                            </div>

                            // Reset filters
                            <div class="flex items-center justify-between pt-2 border-t border-border/20">
                                <span class="text-xs text-foreground/30">"Design your own algorithm"</span>
                                <button
                                    class="text-xs text-foreground/50 hover:text-brand transition-colors"
                                    on:click=move |_| {
                                        set_selected_content_type.set(ContentType::All);
                                        set_selected_time_range.set(TimeRange::AllTime);
                                        set_selected_synapse.set(None);
                                        set_hide_seen.set(false);
                                        set_only_verified.set(false);
                                    }
                                >
                                    "Reset all"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
