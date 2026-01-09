// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::profile::{
    ProfileData, ProfileHeader, ProfileOverview, ProfileShowcase, ProfileTab, ProfileTabs,
};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn ProfileLayout() -> impl IntoView {
    // Mock profile data - would come from route params and server fetch in production
    let profile = ProfileData::mock();
    let profile_for_header = profile.clone();
    let profile_for_overview = profile.clone();
    let profile_for_showcase = profile.clone();

    // Active tab state
    let (active_tab, set_active_tab) = signal(ProfileTab::Overview);

    // Tab change handler
    let on_tab_change = Callback::new(move |tab: ProfileTab| {
        set_active_tab.set(tab);
    });

    view! {
        <MainLayout>
            <div class="h-full w-full overflow-y-auto scrollbar-styled bg-background">
                // Profile Header
                <ProfileHeader profile=profile_for_header />

                // Tab Navigation - sticky below header
                <div class="sticky top-0 z-30 bg-panel/95 backdrop-blur-lg border-b border-border/50">
                    <ProfileTabs 
                        active_tab=Signal::derive(move || active_tab.get())
                        on_tab_change=on_tab_change
                    />
                </div>

                // Tab Content - fills remaining space
                <div class="bg-panel">
                    {move || {
                        match active_tab.get() {
                            ProfileTab::Overview => view! {
                                <ProfileOverview profile=profile_for_overview.clone() />
                            }.into_any(),
                            ProfileTab::Showcase => view! {
                                <ProfileShowcase profile=profile_for_showcase.clone() />
                            }.into_any(),
                        }
                    }}
                </div>

                // Footer spacer
                <div class="h-12 bg-panel"></div>
            </div>
        </MainLayout>
    }
}
