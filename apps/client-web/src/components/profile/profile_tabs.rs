// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Available tabs on the profile page
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProfileTab {
    Overview,
    Showcase,
}

impl ProfileTab {
    pub fn label(&self) -> &'static str {
        match self {
            ProfileTab::Overview => "Overview",
            ProfileTab::Showcase => "Showcase",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            ProfileTab::Overview => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"/></svg>"#,
            ProfileTab::Showcase => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"/></svg>"#,
        }
    }
}

/// Tab navigation component
#[component]
pub fn ProfileTabs(
    #[prop(into)] active_tab: Signal<ProfileTab>,
    #[prop(into)] on_tab_change: Callback<ProfileTab>,
) -> impl IntoView {
    let tabs = [ProfileTab::Overview, ProfileTab::Showcase];

    view! {
        <div class="border-b border-border/30 bg-panel/30 backdrop-blur-sm sticky top-0 z-10">
            <nav class="flex items-center gap-1 px-4 sm:px-6">
                {tabs.iter().map(|tab| {
                    let tab = *tab;
                    let is_active = move || active_tab.get() == tab;
                    
                    view! {
                        <button
                            class=move || format!(
                                "relative flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors {}",
                                if is_active() {
                                    "text-brand"
                                } else {
                                    "text-foreground/50 hover:text-foreground/80"
                                }
                            )
                            on:click=move |_| on_tab_change.run(tab)
                        >
                            <span class="w-4 h-4" inner_html=tab.icon_svg()></span>
                            {tab.label()}
                            
                            // Active indicator
                            {move || {
                                if is_active() {
                                    view! {
                                        <span class="absolute inset-x-0 bottom-0 h-0.5 bg-brand rounded-full"></span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </button>
                    }
                }).collect_view()}
            </nav>
        </div>
    }
}
