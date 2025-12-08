// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::SettingsTab;
use leptos::prelude::*;

/// Settings navigation sidebar
#[component]
pub fn SettingsSidebar(
    #[prop(into)] active_tab: RwSignal<SettingsTab>,
) -> impl IntoView {
    let tabs = SettingsTab::all();

    view! {
        <nav class="w-full lg:w-64 flex-shrink-0">
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                // Header
                <div class="px-4 py-3 border-b border-border/30 bg-foreground/[0.02]">
                    <h2 class="font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                        "Settings"
                    </h2>
                </div>

                // Navigation items
                <div class="p-2 space-y-1">
                    {tabs.into_iter().map(|tab| {
                        view! {
                            <button
                                class=move || format!(
                                    "w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {}",
                                    if active_tab.get() == tab {
                                        "bg-brand/15 text-brand"
                                    } else {
                                        "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                                    }
                                )
                                on:click=move |_| active_tab.set(tab)
                            >
                                <svg
                                    class=move || format!(
                                        "w-5 h-5 {}",
                                        if active_tab.get() == tab { "text-brand" } else { "text-foreground/40" }
                                    )
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    inner_html=tab.icon_svg()
                                ></svg>
                                <span class="flex-1 text-left">{tab.label()}</span>
                                {move || {
                                    if active_tab.get() == tab {
                                        view! {
                                            <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Footer with version info
                <div class="px-4 py-3 border-t border-border/30 bg-foreground/[0.02]">
                    <div class="flex items-center justify-between text-xs text-foreground/40">
                        <span>"meNexus v0.3.0-alpha"</span>
                        <a href="/changelog" class="hover:text-brand transition-colors">"Changelog"</a>
                    </div>
                </div>
            </div>

            // Quick actions
            <div class="mt-4 bg-card border border-border/50 rounded-2xl p-4">
                <h3 class="text-xs font-medium text-foreground/50 uppercase tracking-wider mb-3">"Quick Actions"</h3>
                <div class="space-y-2">
                    <button class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 transition-colors">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        <span>"Help & Support"</span>
                    </button>
                    <button class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 transition-colors">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/>
                        </svg>
                        <span>"Documentation"</span>
                    </button>
                    <button class="w-full flex items-center gap-2 px-3 py-2 rounded-lg text-sm text-rose-400 hover:bg-rose-500/10 transition-colors">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/>
                        </svg>
                        <span>"Sign Out"</span>
                    </button>
                </div>
            </div>
        </nav>
    }
}
