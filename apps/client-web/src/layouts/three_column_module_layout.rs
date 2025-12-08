// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use module_posts::ui::components::post_feed::PostsFeed;

/// Column identifiers for mobile switching
#[derive(Clone, Copy, PartialEq)]
enum ActiveColumn {
    Left,
    Center,
    Right,
}

use crate::app::SessionUserProfile;

#[component]
pub fn ThreeColumnModuleLayout() -> impl IntoView {
    let session_user =
        use_context::<SessionUserProfile>().expect("SessionUserProfile context not found");

    let session_user_profile = session_user.get().unwrap().unwrap().unwrap();

    let (active_column, set_active_column) = signal(ActiveColumn::Center);

    view! {
        <div class="flex flex-col h-full w-full overflow-hidden">
            // Main content area
            <div class="flex-1 min-h-0 overflow-hidden relative">
                // Desktop: Grid layout / Mobile: Stack with active column
                <div class="h-full w-full lg:grid lg:grid-cols-12 lg:gap-0">
                    // Left column
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-3 lg:border-r lg:border-border/50 lg:block {}",
                        if active_column.get() == ActiveColumn::Left { "block" } else { "hidden" }
                    )>
                        <div class="h-full flex flex-col bg-panel/30 p-4">
                            <h2 class="text-lg font-bold text-foreground mb-4">"Left Panel"</h2>
                            <p class="text-foreground/50 text-sm">"Content for the left column"</p>
                        </div>
                    </div>

                    // Center column (Main feed)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-6 lg:block {}",
                        if active_column.get() == ActiveColumn::Center { "block" } else { "hidden" }
                    )>
                        <PostsFeed session_user_profile=session_user_profile/>
                    </div>

                    // Right column
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-3 lg:border-l lg:border-border/50 lg:block {}",
                        if active_column.get() == ActiveColumn::Right { "block" } else { "hidden" }
                    )>
                        <div class="h-full flex flex-col bg-panel/30 p-4">
                            <h2 class="text-lg font-bold text-foreground mb-4">"Right Panel"</h2>
                            <p class="text-foreground/50 text-sm">"Content for the right column"</p>
                        </div>
                    </div>
                </div>
            </div>

            // Mobile column switcher (hidden on desktop)
            <div class="flex-shrink-0 lg:hidden border-t border-border/50 bg-panel/80 backdrop-blur-sm safe-area-bottom">
                <div class="flex items-center justify-center gap-1 p-2">
                    <button
                        class=move || format!(
                            "flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded-xl font-medium text-xs transition-all {}",
                            if active_column.get() == ActiveColumn::Left {
                                "bg-brand text-white"
                            } else {
                                "bg-foreground/5 text-foreground/60"
                            }
                        )
                        on:click=move |_| set_active_column.set(ActiveColumn::Left)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h7"></path>
                        </svg>
                        "Left"
                    </button>

                    <button
                        class=move || format!(
                            "flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded-xl font-medium text-xs transition-all {}",
                            if active_column.get() == ActiveColumn::Center {
                                "bg-brand text-white"
                            } else {
                                "bg-foreground/5 text-foreground/60"
                            }
                        )
                        on:click=move |_| set_active_column.set(ActiveColumn::Center)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                        </svg>
                        "Feed"
                    </button>

                    <button
                        class=move || format!(
                            "flex-1 flex items-center justify-center gap-1.5 py-2 px-3 rounded-xl font-medium text-xs transition-all {}",
                            if active_column.get() == ActiveColumn::Right {
                                "bg-brand text-white"
                            } else {
                                "bg-foreground/5 text-foreground/60"
                            }
                        )
                        on:click=move |_| set_active_column.set(ActiveColumn::Right)
                    >
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16m-7 6h7"></path>
                        </svg>
                        "Right"
                    </button>
                </div>
            </div>
        </div>
    }
}
