// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{RecentSearch, SearchCategory, SearchScope};
use leptos::prelude::*;

/// Advanced search bar with suggestions and recent searches
#[component]
pub fn SearchBar(
    #[prop(into)] query: RwSignal<String>,
    #[prop(into)] scope: RwSignal<SearchScope>,
    #[prop(into)] on_search: Callback<String>,
) -> impl IntoView {
    let (is_focused, set_is_focused) = signal(false);
    let (show_suggestions, set_show_suggestions) = signal(false);
    let recent_searches = RecentSearch::mock_recent();

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let q = query.get();
        if !q.trim().is_empty() {
            on_search.run(q);
            set_show_suggestions.set(false);
        }
    };

    let handle_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        query.set(value.clone());
        set_show_suggestions.set(!value.is_empty() || is_focused.get());
    };

    view! {
        <div class="relative w-full">
            <form on:submit=handle_submit class="relative">
                // Search input container
                <div class=move || format!(
                    "relative flex items-center bg-card border rounded-2xl transition-all duration-300 {}",
                    if is_focused.get() {
                        "border-brand/50 ring-2 ring-brand/20 shadow-lg shadow-brand/5"
                    } else {
                        "border-border/50 hover:border-border"
                    }
                )>
                    // Search icon
                    <div class="pl-4 pr-2">
                        <svg class=move || format!(
                            "w-5 h-5 transition-colors {}",
                            if is_focused.get() { "text-brand" } else { "text-foreground/40" }
                        ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                    </div>

                    // Input field
                    <input
                        type="text"
                        placeholder="Search users, synapses, posts, tags..."
                        class="flex-1 bg-transparent py-3.5 text-foreground placeholder-foreground/40 focus:outline-none text-base"
                        prop:value=move || query.get()
                        on:input=handle_input
                        on:focus=move |_| {
                            set_is_focused.set(true);
                            set_show_suggestions.set(true);
                        }
                        on:blur=move |_| {
                            // Delay to allow click on suggestions
                            set_timeout(move || {
                                set_is_focused.set(false);
                                set_show_suggestions.set(false);
                            }, std::time::Duration::from_millis(200));
                        }
                    />

                    // Clear button
                    {move || {
                        if !query.get().is_empty() {
                            view! {
                                <button
                                    type="button"
                                    class="p-2 text-foreground/40 hover:text-foreground transition-colors"
                                    on:click=move |_| {
                                        query.set(String::new());
                                        set_show_suggestions.set(false);
                                    }
                                >
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                    </svg>
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}

                    // Scope toggle
                    <div class="flex items-center border-l border-border/30 px-2">
                        <button
                            type="button"
                            class=move || format!(
                                "px-3 py-1.5 text-xs font-medium rounded-lg transition-all {}",
                                if scope.get() == SearchScope::MyNetwork {
                                    "bg-brand/15 text-brand"
                                } else {
                                    "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                                }
                            )
                            on:click=move |_| scope.set(SearchScope::MyNetwork)
                        >
                            "My Network"
                        </button>
                        <button
                            type="button"
                            class=move || format!(
                                "px-3 py-1.5 text-xs font-medium rounded-lg transition-all {}",
                                if scope.get() == SearchScope::Global {
                                    "bg-brand/15 text-brand"
                                } else {
                                    "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                                }
                            )
                            on:click=move |_| scope.set(SearchScope::Global)
                        >
                            "Global"
                        </button>
                    </div>

                    // Search button
                    <button
                        type="submit"
                        class="m-1.5 px-5 py-2 bg-brand hover:bg-brand/90 text-white rounded-xl font-medium text-sm transition-all shadow-lg shadow-brand/20"
                    >
                        "Search"
                    </button>
                </div>
            </form>

            // Suggestions dropdown
            {move || {
                let recents = recent_searches.clone();
                if show_suggestions.get() && query.get().is_empty() {
                    view! {
                        <div class="absolute top-full left-0 right-0 mt-2 bg-panel border border-border/50 rounded-2xl shadow-2xl z-50 overflow-hidden">
                            // Recent searches
                            <div class="p-3 border-b border-border/30">
                                <div class="flex items-center justify-between mb-2">
                                    <span class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Recent Searches"</span>
                                    <button class="text-xs text-brand hover:underline">"Clear all"</button>
                                </div>
                                <div class="space-y-1">
                                    {recents.into_iter().map(|search| {
                                        let search_query = search.query.clone();
                                        let search_query_2 = search.query.clone();
                                        view! {
                                            <button
                                                class="w-full flex items-center gap-3 px-3 py-2 rounded-lg hover:bg-foreground/5 transition-colors text-left"
                                                on:click=move |_| {
                                                    query.set(search_query.clone());
                                                    on_search.run(search_query.clone());
                                                }
                                            >
                                                <svg class="w-4 h-4 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                </svg>
                                                <span class="flex-1 text-sm text-foreground">{search_query_2}</span>
                                                <span class="text-xs text-foreground/30 bg-foreground/5 px-2 py-0.5 rounded">{search.category.label()}</span>
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Quick search suggestions
                            <div class="p-3">
                                <span class="text-xs font-medium text-foreground/50 uppercase tracking-wider">"Try searching for"</span>
                                <div class="flex flex-wrap gap-2 mt-2">
                                    {["#RustLang", "#LocalFirst", "@neo", "@oracle", "Zion HQ", "CRDT"].into_iter().map(|suggestion| {
                                        let s = suggestion.to_string();
                                        let s2 = suggestion.to_string();
                                        view! {
                                            <button
                                                class="px-3 py-1.5 text-xs bg-foreground/5 hover:bg-foreground/10 text-foreground/70 hover:text-foreground rounded-lg transition-colors"
                                                on:click=move |_| {
                                                    query.set(s.clone());
                                                    on_search.run(s.clone());
                                                }
                                            >
                                                {s2}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
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

/// Compact search bar for sidebar/header use
#[component]
pub fn CompactSearchBar(
    #[prop(into)] on_search: Callback<String>,
) -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let handle_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let q = query.get();
        if !q.trim().is_empty() {
            on_search.run(q);
        }
    };

    view! {
        <form on:submit=handle_submit class="relative">
            <div class="relative flex items-center bg-foreground/5 hover:bg-foreground/[0.07] border border-border/30 rounded-xl transition-all focus-within:border-brand/50 focus-within:ring-1 focus-within:ring-brand/20">
                <svg class="w-4 h-4 ml-3 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
                <input
                    type="text"
                    placeholder="Search..."
                    class="flex-1 bg-transparent py-2 px-2 text-sm text-foreground placeholder-foreground/40 focus:outline-none"
                    prop:value=move || query.get()
                    on:input=move |ev| set_query.set(event_target_value(&ev))
                />
                {move || {
                    if !query.get().is_empty() {
                        view! {
                            <button
                                type="submit"
                                class="mr-1.5 p-1.5 text-brand hover:bg-brand/10 rounded-lg transition-colors"
                            >
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14 5l7 7m0 0l-7 7m7-7H3"/>
                                </svg>
                            </button>
                        }.into_any()
                    } else {
                        view! { <span class="w-7"></span> }.into_any()
                    }
                }}
            </div>
        </form>
    }
}
