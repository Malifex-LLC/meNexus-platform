// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{AccessLevel, ContentFilter, ContentSort, ContentType};
use crate::ui::components::creator_feed::ViewMode;
use leptos::prelude::*;

/// Filter and sort bar for content
#[component]
pub fn ContentFilterBar(
    /// Current filter state
    filter: ReadSignal<ContentFilter>,
    /// Setter for filter state
    set_filter: WriteSignal<ContentFilter>,
    /// Current sort state
    sort: ReadSignal<ContentSort>,
    /// Setter for sort state
    set_sort: WriteSignal<ContentSort>,
    /// Current view mode
    view_mode: ReadSignal<ViewMode>,
    /// Setter for view mode
    set_view_mode: WriteSignal<ViewMode>,
    /// Available tags
    tags: Vec<String>,
) -> impl IntoView {
    let (show_filters, set_show_filters) = signal(false);
    let (show_sort, set_show_sort) = signal(false);
    let (search_query, set_search_query) = signal(String::new());

    // Content type options
    let content_types = vec![
        (None, "All"),
        (Some(ContentType::Image), "Images"),
        (Some(ContentType::Gallery), "Galleries"),
        (Some(ContentType::Video), "Videos"),
        (Some(ContentType::Audio), "Audio"),
        (Some(ContentType::Text), "Articles"),
        (Some(ContentType::File), "Downloads"),
        (Some(ContentType::Poll), "Polls"),
    ];

    // Access level options
    let access_levels = vec![
        (None, "All Access"),
        (Some(AccessLevel::Free), "Free"),
        (Some(AccessLevel::Subscribers), "Subscribers"),
    ];

    // Sort options
    let sort_options = vec![
        ContentSort::Newest,
        ContentSort::Oldest,
        ContentSort::MostLiked,
        ContentSort::MostCommented,
    ];

    // Handle search input
    let on_search_input = move |ev: leptos::ev::Event| {
        let value = event_target_value(&ev);
        set_search_query.set(value.clone());
        set_filter.update(|f| {
            f.search_query = if value.is_empty() { None } else { Some(value) };
        });
    };

    // Clear all filters
    let clear_filters = move |_| {
        set_filter.set(ContentFilter::default());
        set_search_query.set(String::new());
    };

    // Check if any filters are active
    let has_active_filters = Memo::new(move |_| {
        let f = filter.get();
        f.content_type.is_some() || f.access_level.is_some() || f.tag.is_some() || f.search_query.is_some()
    });

    view! {
        <div class="space-y-4">
            // Main filter bar
            <div class="flex flex-wrap items-center gap-2 sm:gap-3">
                // Search input
                <div class="relative flex-1 min-w-[200px]">
                    <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                    </svg>
                    <input
                        type="text"
                        placeholder="Search posts..."
                        class="w-full pl-10 pr-4 py-2.5 bg-foreground/5 border border-border/30 rounded-xl text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-1 focus:ring-brand/20 transition-all text-sm"
                        prop:value=move || search_query.get()
                        on:input=on_search_input
                    />
                    {move || {
                        if !search_query.get().is_empty() {
                            view! {
                                <button
                                    class="absolute right-3 top-1/2 -translate-y-1/2 p-1 rounded-full hover:bg-foreground/10 text-foreground/40 hover:text-foreground transition-colors"
                                    on:click=move |_| {
                                        set_search_query.set(String::new());
                                        set_filter.update(|f| f.search_query = None);
                                    }
                                >
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </button>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>

                // Filter toggle button
                <div class="relative">
                    <button
                        class=move || format!(
                            "flex items-center gap-2 px-4 py-2.5 rounded-xl border transition-all text-sm font-medium {}",
                            if has_active_filters.get() || show_filters.get() {
                                "bg-brand/10 border-brand/30 text-brand"
                            } else {
                                "bg-foreground/5 border-border/30 text-foreground/70 hover:text-foreground hover:border-border/50"
                            }
                        )
                        on:click=move |_| set_show_filters.update(|v| *v = !*v)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"></path>
                        </svg>
                        "Filter"
                        {move || {
                            if has_active_filters.get() {
                                let count = [
                                    filter.get().content_type.is_some(),
                                    filter.get().access_level.is_some(),
                                    filter.get().tag.is_some(),
                                ].iter().filter(|&&x| x).count();
                                if count > 0 {
                                    view! {
                                        <span class="min-w-5 h-5 px-1.5 flex items-center justify-center rounded-full bg-brand text-white text-xs font-bold">
                                            {count}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </button>
                </div>

                // Sort dropdown
                <div class="relative">
                    <button
                        class="flex items-center gap-2 px-4 py-2.5 rounded-xl border border-border/30 bg-foreground/5 text-foreground/70 hover:text-foreground hover:border-border/50 transition-all text-sm font-medium"
                        on:click=move |_| set_show_sort.update(|v| *v = !*v)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12"></path>
                        </svg>
                        {move || sort.get().label()}
                        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                        </svg>
                    </button>

                    // Sort dropdown menu
                    {move || {
                        if show_sort.get() {
                            view! {
                                <div class="absolute right-0 top-full mt-2 w-48 bg-panel border border-border/50 rounded-xl shadow-xl z-20 py-1">
                                    {sort_options.iter().map(|opt| {
                                        let opt_clone = opt.clone();
                                        let is_selected = sort.get() == *opt;
                                        view! {
                                            <button
                                                class=format!(
                                                    "w-full flex items-center justify-between px-4 py-2 text-sm transition-colors {}",
                                                    if is_selected { "bg-brand/10 text-brand" } else { "text-foreground/70 hover:bg-foreground/5 hover:text-foreground" }
                                                )
                                                on:click=move |_| {
                                                    set_sort.set(opt_clone.clone());
                                                    set_show_sort.set(false);
                                                }
                                            >
                                                <span>{opt.label()}</span>
                                                {if is_selected {
                                                    view! {
                                                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                                                            <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41L9 16.17z"></path>
                                                        </svg>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
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

                // View mode toggle
                <div class="hidden sm:flex items-center gap-1 p-1 bg-foreground/5 rounded-xl border border-border/30">
                    <button
                        class=move || format!(
                            "p-2 rounded-lg transition-all {}",
                            if view_mode.get() == ViewMode::Feed {
                                "bg-panel text-foreground shadow-sm"
                            } else {
                                "text-foreground/50 hover:text-foreground"
                            }
                        )
                        on:click=move |_| set_view_mode.set(ViewMode::Feed)
                        title="List view"
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path>
                        </svg>
                    </button>
                    <button
                        class=move || format!(
                            "p-2 rounded-lg transition-all {}",
                            if view_mode.get() == ViewMode::Grid {
                                "bg-panel text-foreground shadow-sm"
                            } else {
                                "text-foreground/50 hover:text-foreground"
                            }
                        )
                        on:click=move |_| set_view_mode.set(ViewMode::Grid)
                        title="Grid view"
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path>
                        </svg>
                    </button>
                </div>
            </div>

            // Expanded filters panel
            {move || {
                if show_filters.get() {
                    view! {
                        <div class="p-4 bg-foreground/5 rounded-xl border border-border/30 space-y-4">
                            // Content type filter
                            <div class="space-y-2">
                                <label class="text-xs font-semibold text-foreground/50 uppercase tracking-wider">"Content Type"</label>
                                <div class="flex flex-wrap gap-2">
                                    {content_types.iter().map(|(ct, label)| {
                                        let ct_clone = ct.clone();
                                        let is_selected = filter.get().content_type == *ct;
                                        view! {
                                            <button
                                                class=format!(
                                                    "px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                                                    if is_selected {
                                                        "bg-brand text-white"
                                                    } else {
                                                        "bg-panel border border-border/30 text-foreground/70 hover:text-foreground hover:border-border/50"
                                                    }
                                                )
                                                on:click=move |_| {
                                                    set_filter.update(|f| f.content_type = ct_clone.clone());
                                                }
                                            >
                                                {*label}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Access level filter
                            <div class="space-y-2">
                                <label class="text-xs font-semibold text-foreground/50 uppercase tracking-wider">"Access Level"</label>
                                <div class="flex flex-wrap gap-2">
                                    {access_levels.iter().map(|(al, label)| {
                                        let al_clone = al.clone();
                                        let is_selected = filter.get().access_level == *al;
                                        view! {
                                            <button
                                                class=format!(
                                                    "px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                                                    if is_selected {
                                                        "bg-brand text-white"
                                                    } else {
                                                        "bg-panel border border-border/30 text-foreground/70 hover:text-foreground hover:border-border/50"
                                                    }
                                                )
                                                on:click=move |_| {
                                                    set_filter.update(|f| f.access_level = al_clone.clone());
                                                }
                                            >
                                                {*label}
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            // Tags filter
                            {if !tags.is_empty() {
                                view! {
                                    <div class="space-y-2">
                                        <label class="text-xs font-semibold text-foreground/50 uppercase tracking-wider">"Tags"</label>
                                        <div class="flex flex-wrap gap-2">
                                            <button
                                                class=format!(
                                                    "px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                                                    if filter.get().tag.is_none() {
                                                        "bg-brand text-white"
                                                    } else {
                                                        "bg-panel border border-border/30 text-foreground/70 hover:text-foreground hover:border-border/50"
                                                    }
                                                )
                                                on:click=move |_| {
                                                    set_filter.update(|f| f.tag = None);
                                                }
                                            >
                                                "All"
                                            </button>
                                            {tags.iter().map(|tag| {
                                                let tag_clone = tag.clone();
                                                let tag_for_click = tag.clone();
                                                let is_selected = filter.get().tag.as_ref() == Some(tag);
                                                view! {
                                                    <button
                                                        class=format!(
                                                            "px-3 py-1.5 rounded-lg text-sm font-medium transition-all {}",
                                                            if is_selected {
                                                                "bg-brand text-white"
                                                            } else {
                                                                "bg-panel border border-border/30 text-foreground/70 hover:text-foreground hover:border-border/50"
                                                            }
                                                        )
                                                        on:click=move |_| {
                                                            set_filter.update(|f| f.tag = Some(tag_for_click.clone()));
                                                        }
                                                    >
                                                        "#"{tag_clone}
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}

                            // Clear filters button
                            {move || {
                                if has_active_filters.get() {
                                    view! {
                                        <button
                                            class="flex items-center gap-2 text-sm text-brand hover:underline"
                                            on:click=clear_filters
                                        >
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                                            </svg>
                                            "Clear all filters"
                                        </button>
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

            // Active filter pills
            {move || {
                let f = filter.get();
                let has_filters = f.content_type.is_some() || f.access_level.is_some() || f.tag.is_some();

                if has_filters && !show_filters.get() {
                    view! {
                        <div class="flex flex-wrap items-center gap-2">
                            <span class="text-xs text-foreground/50">"Active filters:"</span>

                            {if let Some(ct) = f.content_type.clone() {
                                view! {
                                    <FilterPill
                                        label=ct.label().to_string()
                                        on_remove=Callback::new(move |_| set_filter.update(|f| f.content_type = None))
                                    />
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}

                            {if let Some(al) = f.access_level.clone() {
                                view! {
                                    <FilterPill
                                        label=al.display_name()
                                        on_remove=Callback::new(move |_| set_filter.update(|f| f.access_level = None))
                                    />
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}

                            {if let Some(tag) = f.tag.clone() {
                                view! {
                                    <FilterPill
                                        label=format!("#{}", tag)
                                        on_remove=Callback::new(move |_| set_filter.update(|f| f.tag = None))
                                    />
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}

                            <button
                                class="text-xs text-brand hover:underline ml-2"
                                on:click=clear_filters
                            >
                                "Clear all"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Individual filter pill with remove button
#[component]
fn FilterPill(label: String, on_remove: Callback<()>) -> impl IntoView {
    view! {
        <span class="inline-flex items-center gap-1 px-2 py-1 bg-brand/10 text-brand rounded-lg text-xs font-medium">
            {label}
            <button
                class="p-0.5 rounded-full hover:bg-brand/20 transition-colors"
                on:click=move |_| on_remove.run(())
            >
                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
        </span>
    }
}

