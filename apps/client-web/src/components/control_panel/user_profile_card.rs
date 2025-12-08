// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::app::SessionUserProfile;
use leptos::prelude::*;
use leptos_router::components::A;
use synapse_core::domain::profiles::Profile;

// Mock user data for development
// #[derive(Clone)]
// pub struct UserProfile {
//     pub display_name: String,
//     pub handle: String,
//     pub avatar_url: Option<String>,
//     pub public_key: String,
//     pub followers_count: u32,
//     pub following_count: u32,
//     pub posts_count: u32,
//     pub is_online: bool,
// }

// fn get_mock_user() -> Option<UserProfile> {
//     Some(UserProfile {
//         display_name: "Neo Anderson".to_string(),
//         handle: "neo".to_string(),
//         avatar_url: None,
//         public_key: "02a3b5c7d9e1f2a4b6c8d0e2f4a6b8c0d2e4f6a8b0c2d4e6f8a0b2c4d6e8f0a2b4"
//             .to_string(),
//         followers_count: 1_247,
//         following_count: 89,
//         posts_count: 342,
//         is_online: true,
//     })
// }

fn format_count(count: u32) -> String {
    if count >= 1000 {
        format!("{:.1}K", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}

#[component]
pub fn UserProfileCard() -> impl IntoView {
    let session_user =
        use_context::<SessionUserProfile>().expect("SessionUserProfile context not found");

    let session_user = session_user.get().unwrap().unwrap();

    view! {
        <div class="p-2 bg-gradient-to-br from-panel to-background rounded-lg border border-border/50">
            {match session_user {
                Some(profile) => {
                    // let initials: String = profile.display_name.unwrap().clone()
                    //     .split_whitespace()
                    //     .take(2)
                    //     .filter_map(|s| s.chars().next())
                    //     .collect();

                    view! {
                        <div class="space-y-2">
                            // Avatar and basic info row
                            <div class="flex items-center gap-2">
                                // Compact avatar
                                <div class="relative flex-shrink-0">
                                    {if let Some(avatar_url) = profile.avatar_url.clone() {
                                        view! {
                                            <img
                                                src=avatar_url
                                                alt="Avatar"
                                                class="w-10 h-10 rounded-lg object-cover ring-1 ring-brand/30"
                                            />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-brand/30 to-brand/10 ring-1 ring-brand/30 flex items-center justify-center">
                                                // <span class="text-brand font-bold text-sm">{initials}</span>
                                                <span class="text-brand font-bold text-sm">User</span>
                                            </div>
                                        }.into_any()
                                    }}
                                    // {if profile.is_online {
                                    //     view! {
                                    //         <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-emerald-500 rounded-full border-2 border-panel"></div>
                                    //     }.into_any()
                                    // } else {
                                    //     view! { <span></span> }.into_any()
                                    // }}
                                </div>

                                // Name and handle
                                <div class="flex-1 min-w-0">
                                    <h2 class="font-bold text-foreground text-sm truncate">{profile.display_name.clone()}</h2>
                                    <p class="text-brand font-mono text-xs">{"@"}{profile.handle.clone()}</p>
                                </div>

                                // Settings button
                                <A href="/settings" attr:class="p-1.5 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors">
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                    </svg>
                                </A>
                            </div>

                            // Compact stats row
                            // <div class="flex items-center justify-between px-1">
                            //     <div class="flex items-center gap-3 text-[10px]">
                            //         <span class="text-foreground/60">
                            //             <span class="font-semibold text-foreground">{format_count(profile.posts_count)}</span>
                            //             " posts"
                            //         </span>
                            //         <span class="text-foreground/60">
                            //             <span class="font-semibold text-foreground">{format_count(profile.followers_count)}</span>
                            //             " followers"
                            //         </span>
                            //     </div>
                            //     <A href=format!("/profiles/{}", profile.handle) attr:class="text-[10px] text-brand hover:underline">
                            //         "View →"
                            //     </A>
                            // </div>
                        </div>
                    }.into_any()
                },
                None => view! {
                    <div class="text-center py-3">
                        <p class="text-foreground/50 text-xs">"Not logged in"</p>
                        <A href="/login" attr:class="text-brand text-xs hover:underline">"Sign in"</A>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
