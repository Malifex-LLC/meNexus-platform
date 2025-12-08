// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{BlockedUser, DMPermission, ProfileVisibility};
use leptos::prelude::*;

/// Privacy settings panel
#[component]
pub fn PrivacySettings() -> impl IntoView {
    let (profile_visibility, set_profile_visibility) = signal(ProfileVisibility::Public);
    let (dm_permission, set_dm_permission) = signal(DMPermission::Following);
    let (show_online_status, set_show_online_status) = signal(true);
    let (show_read_receipts, set_show_read_receipts) = signal(true);
    let (allow_mentions, set_allow_mentions) = signal(true);
    let (allow_tagging, set_allow_tagging) = signal(true);
    let (discoverable, set_discoverable) = signal(true);
    let (show_in_suggestions, set_show_in_suggestions) = signal(true);

    let blocked_users = BlockedUser::mock_blocked();

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Privacy Settings"</h2>
                <p class="text-sm text-foreground/50">"Control who can see and interact with your content"</p>
            </div>

            // Profile Visibility
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                    </svg>
                    "Profile Visibility"
                </h3>

                <div class="space-y-2">
                    {[ProfileVisibility::Public, ProfileVisibility::NetworkOnly, ProfileVisibility::Private].into_iter().map(|option| {
                        view! {
                            <label class=move || format!(
                                "flex items-center justify-between p-4 rounded-xl cursor-pointer transition-all {}",
                                if profile_visibility.get() == option {
                                    "bg-brand/10 border-2 border-brand/50"
                                } else {
                                    "bg-foreground/5 border-2 border-transparent hover:bg-foreground/10"
                                }
                            )>
                                <div class="flex items-center gap-3">
                                    <input
                                        type="radio"
                                        name="visibility"
                                        class="sr-only"
                                        checked=move || profile_visibility.get() == option
                                        on:change=move |_| set_profile_visibility.set(option)
                                    />
                                    <div class=move || format!(
                                        "w-5 h-5 rounded-full border-2 flex items-center justify-center {}",
                                        if profile_visibility.get() == option {
                                            "border-brand bg-brand"
                                        } else {
                                            "border-foreground/30"
                                        }
                                    )>
                                        {move || {
                                            if profile_visibility.get() == option {
                                                view! {
                                                    <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                    </div>
                                    <div>
                                        <p class="text-sm font-medium text-foreground">{option.label()}</p>
                                        <p class="text-xs text-foreground/50">{option.description()}</p>
                                    </div>
                                </div>
                            </label>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Direct Messages
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                    </svg>
                    "Direct Messages"
                </h3>

                <div class="space-y-2">
                    <label class="text-sm font-medium text-foreground">"Who can send you direct messages?"</label>
                    <div class="relative">
                        <select
                            class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-foreground focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 appearance-none cursor-pointer"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_dm_permission.set(match value.as_str() {
                                    "everyone" => DMPermission::Everyone,
                                    "mutuals" => DMPermission::Mutuals,
                                    "nobody" => DMPermission::Nobody,
                                    _ => DMPermission::Following,
                                });
                            }
                        >
                            <option value="everyone" selected=move || dm_permission.get() == DMPermission::Everyone>"Everyone"</option>
                            <option value="following" selected=move || dm_permission.get() == DMPermission::Following>"People you follow"</option>
                            <option value="mutuals" selected=move || dm_permission.get() == DMPermission::Mutuals>"Mutual followers only"</option>
                            <option value="nobody" selected=move || dm_permission.get() == DMPermission::Nobody>"Nobody"</option>
                        </select>
                        <svg class="absolute right-4 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40 pointer-events-none" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                        </svg>
                    </div>
                </div>

                // Message toggles
                <div class="space-y-3 pt-2">
                    <ToggleSetting
                        label="Show read receipts"
                        description="Let others know when you've read their messages"
                        checked=show_read_receipts
                        on_change=move |v| set_show_read_receipts.set(v)
                    />
                </div>
            </div>

            // Online & Activity Status
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M5.636 18.364a9 9 0 010-12.728m12.728 0a9 9 0 010 12.728m-9.9-2.829a5 5 0 010-7.07m7.072 0a5 5 0 010 7.07M13 12a1 1 0 11-2 0 1 1 0 012 0z"/>
                    </svg>
                    "Online & Activity Status"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Show online status"
                        description="Let others see when you're active"
                        checked=show_online_status
                        on_change=move |v| set_show_online_status.set(v)
                    />
                </div>
            </div>

            // Mentions & Tagging
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>
                    </svg>
                    "Mentions & Tagging"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Allow mentions"
                        description="Let others @mention you in posts"
                        checked=allow_mentions
                        on_change=move |v| set_allow_mentions.set(v)
                    />
                    <ToggleSetting
                        label="Allow photo tagging"
                        description="Let others tag you in photos"
                        checked=allow_tagging
                        on_change=move |v| set_allow_tagging.set(v)
                    />
                </div>
            </div>

            // Discoverability
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                    </svg>
                    "Discoverability"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Searchable profile"
                        description="Allow your profile to appear in search results"
                        checked=discoverable
                        on_change=move |v| set_discoverable.set(v)
                    />
                    <ToggleSetting
                        label="Show in suggestions"
                        description="Recommend your profile to other users"
                        checked=show_in_suggestions
                        on_change=move |v| set_show_in_suggestions.set(v)
                    />
                </div>
            </div>

            // Blocked Users
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <div class="flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                        </svg>
                        "Blocked Users"
                    </h3>
                    <span class="text-xs text-foreground/40">{blocked_users.len()}" blocked"</span>
                </div>

                {if blocked_users.is_empty() {
                    view! {
                        <p class="text-sm text-foreground/50 py-4 text-center">"You haven't blocked anyone yet"</p>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-2">
                            {blocked_users.into_iter().map(|user| {
                                let initials: String = user.display_name
                                    .split_whitespace()
                                    .take(2)
                                    .filter_map(|s| s.chars().next())
                                    .collect::<String>()
                                    .to_uppercase();
                                
                                view! {
                                    <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
                                        <div class="flex items-center gap-3">
                                            <div class="w-9 h-9 rounded-full bg-foreground/10 flex items-center justify-center">
                                                <span class="text-foreground/40 font-bold text-xs">{initials}</span>
                                            </div>
                                            <div>
                                                <p class="text-sm font-medium text-foreground">{user.display_name}</p>
                                                <p class="text-xs text-foreground/40">"@"{user.handle}" · Blocked "{user.blocked_date}</p>
                                            </div>
                                        </div>
                                        <button class="px-3 py-1.5 text-xs text-rose-400 hover:bg-rose-500/10 rounded-lg transition-colors">
                                            "Unblock"
                                        </button>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}

/// Reusable toggle setting component
#[component]
fn ToggleSetting(
    label: &'static str,
    description: &'static str,
    #[prop(into)] checked: ReadSignal<bool>,
    on_change: impl Fn(bool) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
            <div>
                <p class="text-sm font-medium text-foreground">{label}</p>
                <p class="text-xs text-foreground/40">{description}</p>
            </div>
            <button
                class=move || format!(
                    "relative w-11 h-6 rounded-full transition-colors {}",
                    if checked.get() { "bg-brand" } else { "bg-foreground/20" }
                )
                on:click=move |_| on_change(!checked.get())
            >
                <span class=move || format!(
                    "absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform {}",
                    if checked.get() { "translate-x-5" } else { "translate-x-0" }
                )></span>
            </button>
        </div>
    }
}
