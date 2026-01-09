// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone)]
pub struct FollowedUser {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_online: bool,
}

fn get_mock_following() -> Vec<FollowedUser> {
    vec![
        FollowedUser {
            handle: "morpheus".to_string(),
            display_name: "Morpheus".to_string(),
            avatar_url: None,
            is_online: true,
        },
        FollowedUser {
            handle: "trinity".to_string(),
            display_name: "Trinity".to_string(),
            avatar_url: None,
            is_online: true,
        },
        FollowedUser {
            handle: "oracle".to_string(),
            display_name: "The Oracle".to_string(),
            avatar_url: None,
            is_online: false,
        },
        FollowedUser {
            handle: "niobe".to_string(),
            display_name: "Niobe".to_string(),
            avatar_url: None,
            is_online: true,
        },
    ]
}

#[component]
pub fn FollowingList() -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(true);
    let following = get_mock_following();
    let online_count = following.iter().filter(|u| u.is_online).count();

    view! {
        <div class="space-y-1">
            // Header
            <button
                class="w-full flex items-center justify-between px-1 py-1 text-[10px] font-medium text-foreground/40 uppercase tracking-wider hover:text-foreground/60 transition-colors"
                on:click=move |_| set_is_expanded.update(|v| *v = !*v)
            >
                <div class="flex items-center gap-1.5">
                    <svg class="w-3 h-3 text-info" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"></path>
                    </svg>
                    <span>"Following"</span>
                    <span class="text-foreground/30">"("{following.len()}")"</span>
                    <span class="text-status-online/70">{online_count}" online"</span>
                </div>
                <svg
                    class=move || format!("w-3 h-3 transition-transform {}", if is_expanded.get() { "" } else { "-rotate-90" })
                    fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                </svg>
            </button>

            // List
            {move || {
                if is_expanded.get() {
                    view! {
                        <div class="space-y-0.5">
                            {following.iter().map(|user| {
                                let handle = user.handle.clone();
                                let display_name = user.display_name.clone();
                                let is_online = user.is_online;

                                let initials: String = display_name
                                    .split_whitespace()
                                    .take(2)
                                    .filter_map(|s| s.chars().next())
                                    .collect();

                                view! {
                                    <A
                                        href=format!("/profiles/{}", handle)
                                        attr:class="group flex items-center gap-2 px-2 py-1 rounded-lg hover:bg-foreground/5 transition-colors"
                                    >
                                        // Avatar
                                        <div class="relative flex-shrink-0">
                                            <div class="w-6 h-6 rounded-full bg-gradient-to-br from-info/20 to-info/5 flex items-center justify-center">
                                                <span class="text-info font-bold text-[10px]">{initials}</span>
                                            </div>
                                            <div class=format!(
                                                "absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full border border-panel {}",
                                                if is_online { "status-online" } else { "status-offline" }
                                            )></div>
                                        </div>

                                        <div class="flex-1 min-w-0">
                                            <span class="text-xs text-foreground/70 group-hover:text-foreground truncate block">{display_name}</span>
                                        </div>
                                    </A>
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
