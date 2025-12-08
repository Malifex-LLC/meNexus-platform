// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::settings::{
    AccountSettings, AppearanceSettings, DataSettings, NetworkSettings, NotificationSettings,
    PrivacySettings, ProfileSettings, SecuritySettings, SettingsSidebar, SettingsTab,
};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn SettingsLayout() -> impl IntoView {
    let active_tab = RwSignal::new(SettingsTab::Profile);

    view! {
        <MainLayout>
            <div class="h-full w-full flex flex-col bg-background">
                // Header - fixed at top
                <header class="flex-shrink-0 bg-background/95 backdrop-blur-lg border-b border-border/50 z-10">
                    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div class="py-6">
                            <div class="flex items-center gap-3">
                                <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center">
                                    <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                    </svg>
                                </div>
                                <div>
                                    <h1 class="text-2xl font-bold text-foreground">"Settings"</h1>
                                    <p class="text-sm text-foreground/50">
                                        {move || active_tab.get().description()}
                                    </p>
                                </div>
                            </div>
                        </div>

                        // Mobile tab selector (shown on small screens)
                        <div class="pb-4 lg:hidden">
                            <MobileTabSelector active_tab=active_tab/>
                        </div>
                    </div>
                </header>

                // Main content area - fills remaining height
                <div class="flex-1 min-h-0 max-w-6xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-6">
                    <div class="flex flex-col lg:flex-row gap-6 h-full">
                        // Sidebar - fixed height, doesn't scroll with content (hidden on mobile)
                        <div class="hidden lg:block flex-shrink-0">
                            <div class="sticky top-0">
                                <SettingsSidebar active_tab=active_tab/>
                            </div>
                        </div>

                        // Settings content - scrollable
                        <div class="flex-1 min-w-0 min-h-0 overflow-y-auto scrollbar-styled rounded-xl">
                            <div class="pb-8">
                                {move || {
                                    match active_tab.get() {
                                        SettingsTab::Profile => view! { <ProfileSettings/> }.into_any(),
                                        SettingsTab::Account => view! { <AccountSettings/> }.into_any(),
                                        SettingsTab::Privacy => view! { <PrivacySettings/> }.into_any(),
                                        SettingsTab::Notifications => view! { <NotificationSettings/> }.into_any(),
                                        SettingsTab::Appearance => view! { <AppearanceSettings/> }.into_any(),
                                        SettingsTab::Security => view! { <SecuritySettings/> }.into_any(),
                                        SettingsTab::Network => view! { <NetworkSettings/> }.into_any(),
                                        SettingsTab::Data => view! { <DataSettings/> }.into_any(),
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </MainLayout>
    }
}

/// Mobile-friendly tab selector dropdown
#[component]
fn MobileTabSelector(
    #[prop(into)] active_tab: RwSignal<SettingsTab>,
) -> impl IntoView {
    let (is_open, set_is_open) = signal(false);
    let tabs = SettingsTab::all();

    view! {
        <div class="relative">
            <button
                class="w-full flex items-center justify-between px-4 py-3 bg-card border border-border/50 rounded-xl text-left"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <div class="flex items-center gap-3">
                    <svg
                        class="w-5 h-5 text-brand"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        stroke-width="2"
                        inner_html=move || active_tab.get().icon_svg()
                    ></svg>
                    <span class="font-medium text-foreground">{move || active_tab.get().label()}</span>
                </div>
                <svg class=move || format!(
                    "w-5 h-5 text-foreground/40 transition-transform {}",
                    if is_open.get() { "rotate-180" } else { "" }
                ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                </svg>
            </button>

            {move || {
                if is_open.get() {
                    let tabs_list = tabs.clone();
                    view! {
                        <div class="absolute top-full left-0 right-0 mt-2 bg-panel border border-border/50 rounded-xl shadow-xl z-50 overflow-hidden max-h-80 overflow-y-auto scrollbar-thin">
                            {tabs_list.into_iter().map(|tab| {
                                view! {
                                    <button
                                        class=move || format!(
                                            "w-full flex items-center gap-3 px-4 py-3 text-left transition-colors {}",
                                            if active_tab.get() == tab {
                                                "bg-brand/10 text-brand"
                                            } else {
                                                "text-foreground/60 hover:bg-foreground/5 hover:text-foreground"
                                            }
                                        )
                                        on:click=move |_| {
                                            active_tab.set(tab);
                                            set_is_open.set(false);
                                        }
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
                                        <div>
                                            <p class="font-medium">{tab.label()}</p>
                                            <p class="text-xs text-foreground/40">{tab.description()}</p>
                                        </div>
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
    }
}
