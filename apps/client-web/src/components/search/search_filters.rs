// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{SearchCategory, SearchSortBy, SearchTimeRange};
use leptos::prelude::*;

/// Category tabs and advanced filters for search
#[component]
pub fn SearchFilters(
    #[prop(into)] active_category: RwSignal<SearchCategory>,
    #[prop(into)] sort_by: RwSignal<SearchSortBy>,
    #[prop(into)] time_range: RwSignal<SearchTimeRange>,
    #[prop(into)] only_verified: RwSignal<bool>,
    result_count: u32,
) -> impl IntoView {
    let (show_advanced, set_show_advanced) = signal(false);
    let categories = SearchCategory::all();

    view! {
        <div class="space-y-3">
            // Category tabs
            <div class="flex items-center gap-2 overflow-x-auto pb-2 scrollbar-hide">
                {categories.into_iter().map(|cat| {
                    view! {
                        <button
                            class=move || format!(
                                "flex items-center gap-2 px-4 py-2 rounded-xl text-sm font-medium transition-all whitespace-nowrap {}",
                                if active_category.get() == cat {
                                    "bg-brand text-white shadow-lg shadow-brand/20"
                                } else {
                                    "bg-foreground/5 text-foreground/60 hover:bg-foreground/10 hover:text-foreground"
                                }
                            )
                            on:click=move |_| active_category.set(cat)
                        >
                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" inner_html=cat.icon_svg()></svg>
                            <span>{cat.label()}</span>
                        </button>
                    }
                }).collect_view()}
            </div>

            // Results info and filter toggle
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <span class="text-sm text-foreground/50">
                        <span class="font-semibold text-foreground">{result_count}</span>
                        " results"
                    </span>
                    
                    // Quick sort buttons
                    <div class="flex items-center gap-1 bg-foreground/5 rounded-lg p-0.5">
                        {SearchSortBy::all().into_iter().take(3).map(|sort| {
                            view! {
                                <button
                                    class=move || format!(
                                        "px-2.5 py-1 text-xs font-medium rounded-md transition-all {}",
                                        if sort_by.get() == sort {
                                            "bg-card text-foreground shadow-sm"
                                        } else {
                                            "text-foreground/50 hover:text-foreground"
                                        }
                                    )
                                    on:click=move |_| sort_by.set(sort)
                                >
                                    {sort.label()}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                <button
                    class=move || format!(
                        "flex items-center gap-2 px-3 py-1.5 rounded-lg text-sm transition-all {}",
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
                    <span>"Filters"</span>
                    <svg class=move || format!(
                        "w-4 h-4 transition-transform {}",
                        if show_advanced.get() { "rotate-180" } else { "" }
                    ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                    </svg>
                </button>
            </div>

            // Advanced filters panel
            {move || {
                if show_advanced.get() {
                    view! {
                        <div class="bg-card border border-border/50 rounded-2xl p-4 space-y-4 animate-in slide-in-from-top-2">
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                                // Sort by
                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Sort by"</label>
                                    <div class="relative">
                                        <select
                                            class="w-full bg-foreground/5 border border-border/30 rounded-xl px-3 py-2.5 text-sm text-foreground focus:outline-none focus:border-brand/50 focus:ring-1 focus:ring-brand/20 appearance-none cursor-pointer"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                let sort = match value.as_str() {
                                                    "recent" => SearchSortBy::Recent,
                                                    "popular" => SearchSortBy::Popular,
                                                    "reputation" => SearchSortBy::Reputation,
                                                    _ => SearchSortBy::Relevance,
                                                };
                                                sort_by.set(sort);
                                            }
                                        >
                                            <option value="relevance" selected=move || sort_by.get() == SearchSortBy::Relevance>"Most relevant"</option>
                                            <option value="recent" selected=move || sort_by.get() == SearchSortBy::Recent>"Most recent"</option>
                                            <option value="popular" selected=move || sort_by.get() == SearchSortBy::Popular>"Most popular"</option>
                                            <option value="reputation" selected=move || sort_by.get() == SearchSortBy::Reputation>"Highest reputation"</option>
                                        </select>
                                        <svg class="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40 pointer-events-none" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                                        </svg>
                                    </div>
                                </div>

                                // Time range
                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Time range"</label>
                                    <div class="relative">
                                        <select
                                            class="w-full bg-foreground/5 border border-border/30 rounded-xl px-3 py-2.5 text-sm text-foreground focus:outline-none focus:border-brand/50 focus:ring-1 focus:ring-brand/20 appearance-none cursor-pointer"
                                            on:change=move |ev| {
                                                let value = event_target_value(&ev);
                                                let range = match value.as_str() {
                                                    "hour" => SearchTimeRange::PastHour,
                                                    "today" => SearchTimeRange::Today,
                                                    "week" => SearchTimeRange::ThisWeek,
                                                    "month" => SearchTimeRange::ThisMonth,
                                                    "year" => SearchTimeRange::ThisYear,
                                                    _ => SearchTimeRange::AnyTime,
                                                };
                                                time_range.set(range);
                                            }
                                        >
                                            <option value="any" selected=move || time_range.get() == SearchTimeRange::AnyTime>"Any time"</option>
                                            <option value="hour" selected=move || time_range.get() == SearchTimeRange::PastHour>"Past hour"</option>
                                            <option value="today" selected=move || time_range.get() == SearchTimeRange::Today>"Today"</option>
                                            <option value="week" selected=move || time_range.get() == SearchTimeRange::ThisWeek>"This week"</option>
                                            <option value="month" selected=move || time_range.get() == SearchTimeRange::ThisMonth>"This month"</option>
                                            <option value="year" selected=move || time_range.get() == SearchTimeRange::ThisYear>"This year"</option>
                                        </select>
                                        <svg class="absolute right-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40 pointer-events-none" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                                        </svg>
                                    </div>
                                </div>

                                // Verified filter
                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Filters"</label>
                                    <label class="flex items-center gap-3 bg-foreground/5 border border-border/30 rounded-xl px-3 py-2.5 cursor-pointer hover:bg-foreground/[0.07] transition-colors">
                                        <input
                                            type="checkbox"
                                            class="w-4 h-4 rounded border-border/50 text-brand focus:ring-brand/20 bg-card"
                                            prop:checked=move || only_verified.get()
                                            on:change=move |ev| only_verified.set(event_target_checked(&ev))
                                        />
                                        <span class="text-sm text-foreground">"Verified only"</span>
                                        <svg class="w-4 h-4 text-brand ml-auto" viewBox="0 0 24 24" fill="currentColor">
                                            <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                        </svg>
                                    </label>
                                </div>

                                // Reset filters
                                <div class="space-y-2">
                                    <label class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Actions"</label>
                                    <button
                                        class="w-full flex items-center justify-center gap-2 bg-foreground/5 border border-border/30 rounded-xl px-3 py-2.5 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/10 transition-colors"
                                        on:click=move |_| {
                                            sort_by.set(SearchSortBy::Relevance);
                                            time_range.set(SearchTimeRange::AnyTime);
                                            only_verified.set(false);
                                        }
                                    >
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                        </svg>
                                        <span>"Reset all"</span>
                                    </button>
                                </div>
                            </div>

                            // Active filters summary
                            {move || {
                                let has_filters = sort_by.get() != SearchSortBy::Relevance 
                                    || time_range.get() != SearchTimeRange::AnyTime 
                                    || only_verified.get();
                                
                                if has_filters {
                                    view! {
                                        <div class="flex items-center gap-2 pt-2 border-t border-border/30">
                                            <span class="text-xs text-foreground/40">"Active:"</span>
                                            <div class="flex flex-wrap gap-2">
                                                {move || {
                                                    if sort_by.get() != SearchSortBy::Relevance {
                                                        view! {
                                                            <span class="inline-flex items-center gap-1 px-2 py-0.5 bg-brand/10 text-brand text-xs rounded-lg">
                                                                {sort_by.get().label()}
                                                                <button
                                                                    class="hover:text-brand/70"
                                                                    on:click=move |_| sort_by.set(SearchSortBy::Relevance)
                                                                >
                                                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                                                    </svg>
                                                                </button>
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }
                                                }}
                                                {move || {
                                                    if time_range.get() != SearchTimeRange::AnyTime {
                                                        view! {
                                                            <span class="inline-flex items-center gap-1 px-2 py-0.5 bg-brand/10 text-brand text-xs rounded-lg">
                                                                {time_range.get().label()}
                                                                <button
                                                                    class="hover:text-brand/70"
                                                                    on:click=move |_| time_range.set(SearchTimeRange::AnyTime)
                                                                >
                                                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                                                    </svg>
                                                                </button>
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }
                                                }}
                                                {move || {
                                                    if only_verified.get() {
                                                        view! {
                                                            <span class="inline-flex items-center gap-1 px-2 py-0.5 bg-brand/10 text-brand text-xs rounded-lg">
                                                                "Verified"
                                                                <button
                                                                    class="hover:text-brand/70"
                                                                    on:click=move |_| only_verified.set(false)
                                                                >
                                                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                                                    </svg>
                                                                </button>
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }
                                                }}
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
