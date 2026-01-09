// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::app::SessionUserProfile;
use leptos::prelude::*;
use leptos_router::components::A;

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

    view! {
        <div class="p-2 bg-gradient-to-br from-panel to-background rounded-lg border border-border/50">
            <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                {move || {
                    match session_user.get() {
                        // Resource is ready and we have a profile
                        Some(Ok(Some(profile))) => {
                            let display_name = profile
                                .display_name
                                .clone()
                                .unwrap_or_else(|| "User".to_string());
                            let handle_str = profile
                                .handle
                                .clone()
                                .unwrap_or_else(|| "user".to_string());
                            let avatar_url = profile.avatar_url.clone();
                            //let posts_count = profile.posts_count;
                            //let followers_count = profile.followers_count;

                            view! {
                                <div class="space-y-2">
                                    // Avatar and basic info row
                                    <div class="flex items-center gap-2">
                                        // Compact avatar
                                        <div class="relative flex-shrink-0">
                                            {
                                                match avatar_url {
                                                    Some(url) => view! {
                                                        <img
                                                            src=url
                                                            alt="Avatar"
                                                            class="w-10 h-10 rounded-lg object-cover ring-1 ring-brand/30"
                                                        />
                                                    }.into_any(),
                                                    None => view! {
                                                        <div class="w-10 h-10 rounded-lg bg-gradient-to-br from-brand/30 to-brand/10 ring-1 ring-brand/30 flex items-center justify-center">
                                                            <span class="text-brand font-bold text-sm">"User"</span>
                                                        </div>
                                                    }.into_any(),
                                                }
                                            }
                                        </div>

                                        // Name and handle
                                        <div class="flex-1 min-w-0">
                                            <h2 class="font-bold text-foreground text-sm truncate">
                                                {display_name}
                                            </h2>
                                            <p class="text-brand font-mono text-xs">
                                                {"@"}{handle_str.clone()}
                                            </p>
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
                                    <div class="flex items-center justify-between px-1">
                                        <div class="flex items-center gap-3 text-[10px]">
                                            <span class="text-foreground/60">
                                                <span class="font-semibold text-foreground">
                                                    //{format_count(posts_count)}
                                                    {format_count(1337)}
                                                </span>
                                                " posts"
                                            </span>
                                            <span class="text-foreground/60">
                                                <span class="font-semibold text-foreground">
                                                    //{format_count(followers_count)}
                                                    {format_count(669)}
                                                </span>
                                                " followers"
                                            </span>
                                            <span class="text-foreground/60">
                                                <span class="font-semibold text-foreground">
                                                    //{format_count(followers_count)}
                                                    {format_count(336)}
                                                </span>
                                                " following"
                                            </span>
                                        </div>
                                        <A
                                            href=format!("/profile")
                                            attr:class="text-[10px] text-brand hover:underline"
                                        >
                                            "View →"
                                        </A>
                                    </div>
                                </div>
                            }.into_any()
                        }

                        // No profile or error → treat as not logged in
                        _ => view! {
                            <div class="text-center py-3">
                                <p class="text-foreground/50 text-xs">"Not logged in"</p>
                                <A href="/login" attr:class="text-brand text-xs hover:underline">
                                    "Sign in"
                                </A>
                            </div>
                        }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}
