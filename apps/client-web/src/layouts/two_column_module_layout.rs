// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::{ModulePanel, TabbedModules, tabs};
use leptos::prelude::*;
use module_activity::ui::components::activity_feed::ActivityFeed;
use module_chat::ui::components::chat_feed::ChatFeed;
use module_creator::ui::components::creator_feed::CreatorFeed;
use module_livestream::ui::components::LivestreamFeed;
use module_members::ui::components::members_list::MembersList;
use module_posts::ui::components::post_feed::PostsFeed;

/// Column identifiers for mobile switching
#[derive(Clone, Copy, PartialEq)]
enum ActiveColumn {
    Left,
    Right,
}

use crate::app::SessionUserProfile;

#[component]
pub fn TwoColumnModuleLayout() -> impl IntoView {
    let session_user =
        use_context::<SessionUserProfile>().expect("SessionUserProfile context not found");

    let session_user_profile = session_user.get().unwrap().unwrap().unwrap();

    let (active_column, set_active_column) = signal(ActiveColumn::Left);

    // Define tabs for left column (Posts + Livestream)
    let left_column_tabs = vec![tabs::posts(), tabs::chat()];

    // Define tabs for right column (Chat + Activity)
    let right_column_tabs = vec![tabs::members(), tabs::activity()];

    view! {
        <div class="flex flex-col h-full w-full overflow-hidden">
            // Main content area
            <div class="flex-1 min-h-0 overflow-hidden relative">
                // Desktop: Grid layout
                // Mobile: Stack with only active column visible
                <div class="h-full w-full lg:grid lg:grid-cols-12 lg:gap-0">
                    // Left column (Posts + Livestream)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-8 lg:block {}",
                        if active_column.get() == ActiveColumn::Left { "block" } else { "hidden" }
                    )>
                        <TabbedModules tabs=left_column_tabs>
                            <ModulePanel id="posts">
                                <PostsFeed session_user_profile=session_user_profile/>
                            </ModulePanel>
                            <ModulePanel id="chat">
                                <ChatFeed/>
                            </ModulePanel>
                        </TabbedModules>
                    </div>

                    // Right column (Chat + Activity)
                    <div class=move || format!(
                        "h-full overflow-hidden lg:col-span-4 lg:border-l lg:border-border/50 lg:block {}",
                        if active_column.get() == ActiveColumn::Right { "block" } else { "hidden" }
                    )>
                        <TabbedModules tabs=right_column_tabs>
                            <ModulePanel id="members">
                                <MembersList/>
                            </ModulePanel>
                            <ModulePanel id="activity">
                                <ActivityFeed/>
                            </ModulePanel>
                        </TabbedModules>
                    </div>
                </div>
            </div>

            // Mobile column switcher (hidden on desktop)
            <div class="flex-shrink-0 lg:hidden border-t border-border/50 bg-panel/80 backdrop-blur-sm safe-area-bottom">
                <div class="flex items-center justify-center gap-1 p-2">
                    <button
                        class=move || format!(
                            "flex-1 flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl font-medium text-sm transition-all {}",
                            if active_column.get() == ActiveColumn::Left {
                                "bg-brand text-white shadow-lg shadow-brand/20"
                            } else {
                                "bg-foreground/5 text-foreground/60 hover:text-foreground hover:bg-foreground/10"
                            }
                        )
                        on:click=move |_| set_active_column.set(ActiveColumn::Left)
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path>
                        </svg>
                        "Feed"
                        {move || {
                            if active_column.get() == ActiveColumn::Left {
                                view! {
                                    <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </button>

                    <button
                        class=move || format!(
                            "flex-1 flex items-center justify-center gap-2 py-2.5 px-4 rounded-xl font-medium text-sm transition-all {}",
                            if active_column.get() == ActiveColumn::Right {
                                "bg-brand text-white shadow-lg shadow-brand/20"
                            } else {
                                "bg-foreground/5 text-foreground/60 hover:text-foreground hover:bg-foreground/10"
                            }
                        )
                        on:click=move |_| set_active_column.set(ActiveColumn::Right)
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                        </svg>
                        "Social"
                        // Notification badge placeholder
                        <span class="min-w-5 h-5 px-1.5 flex items-center justify-center rounded-full bg-rose-500 text-white text-xs font-bold">
                            "3"
                        </span>
                    </button>
                </div>
            </div>
        </div>
    }
}
