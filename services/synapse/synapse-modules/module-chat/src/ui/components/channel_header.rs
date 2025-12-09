// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{ChatRoom, ChatUser};
use leptos::prelude::*;

/// Channel header showing room info, members, and actions
#[component]
pub fn ChannelHeader(
    /// The current room data
    #[prop(into)]
    room: ChatRoom,
    /// Online members in the room
    #[prop(into, optional)]
    online_members: Option<Vec<ChatUser>>,
) -> impl IntoView {
    let (show_members, set_show_members) = signal(false);
    let (show_settings, set_show_settings) = signal(false);
    let (is_pinned, set_is_pinned) = signal(false);

    let online_members = online_members.unwrap_or_else(|| {
        ChatUser::mock_users().into_iter().filter(|u| u.is_online).collect()
    });
    let online_count = online_members.len();
    let members_for_dropdown = online_members.clone();

    view! {
        <header class="flex-shrink-0 border-b border-border/50 bg-panel/30 backdrop-blur-sm">
            <div class="flex items-center justify-between px-4 py-3">
                // Left: Channel info
                <div class="flex items-center gap-3">
                    // Channel icon
                    <div class=move || format!(
                        "w-10 h-10 rounded-xl flex items-center justify-center {}",
                        if room.is_private {
                            "bg-warning/20"
                        } else {
                            "bg-brand/20"
                        }
                    )>
                        {if room.is_private {
                            view! {
                                <svg class="w-5 h-5 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"></path>
                                </svg>
                            }.into_any()
                        }}
                    </div>

                    // Channel name & description
                    <div class="flex flex-col">
                        <div class="flex items-center gap-2">
                            <h1 class="text-lg font-bold text-foreground">
                                {if room.is_private {
                                    view! { <span class="text-warning mr-1">"🔒"</span> }.into_any()
                                } else {
                                    view! { <span class="text-brand font-mono mr-1">"#"</span> }.into_any()
                                }}
                                {room.name.clone()}
                            </h1>
                            
                            // Pin indicator
                            {move || {
                                if is_pinned.get() {
                                    view! {
                                        <span class="px-1.5 py-0.5 rounded bg-brand/20 text-brand text-xs font-medium">"Pinned"</span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }
                            }}
                        </div>
                        
                        {if let Some(desc) = room.description.clone() {
                            view! {
                                <p class="text-sm text-foreground/50 truncate max-w-md">{desc}</p>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>

                // Right: Actions
                <div class="flex items-center gap-2">
                    // Online members preview
                    <div class="relative">
                        <button 
                            class=move || format!(
                                "flex items-center gap-2 px-3 py-1.5 rounded-lg transition-colors {}",
                                if show_members.get() {
                                    "bg-foreground/10 text-foreground"
                                } else {
                                    "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                                }
                            )
                            on:click=move |_| set_show_members.update(|v| *v = !*v)
                        >
                            // Stacked avatars
                            <div class="flex -space-x-2">
                                {online_members.iter().take(3).map(|user| {
                                    let initials = user.display_name
                                        .chars()
                                        .next()
                                        .map(|c| c.to_uppercase().to_string())
                                        .unwrap_or_default();
                                    view! {
                                        <div class="w-6 h-6 rounded-full bg-gradient-to-br from-brand/40 to-brand/20 flex items-center justify-center text-[10px] font-bold text-brand ring-2 ring-panel">
                                            {initials}
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            
                            // Count
                            <span class="text-sm font-medium">
                                <span class="text-success">{online_count}</span>
                                <span class="text-foreground/40">" online"</span>
                            </span>
                            
                            // Chevron
                            <svg class=move || format!(
                                "w-4 h-4 transition-transform {}",
                                if show_members.get() { "rotate-180" } else { "" }
                            ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>

                        // Members dropdown
                        {move || {
                            if show_members.get() {
                                view! {
                                    <div class="absolute top-full right-0 mt-2 w-64 bg-panel border border-border/50 rounded-xl shadow-xl z-50 overflow-hidden">
                                        <div class="px-3 py-2 border-b border-border/30 bg-foreground/5">
                                            <div class="flex items-center justify-between">
                                                <span class="text-sm font-semibold text-foreground">"Online Members"</span>
                                                <span class="text-xs text-success">{online_count}" online"</span>
                                            </div>
                                        </div>
                                        <div class="max-h-64 overflow-y-auto scrollbar-thin py-1">
                                            {members_for_dropdown.iter().map(|user| {
                                                let initials = user.display_name
                                                    .chars()
                                                    .next()
                                                    .map(|c| c.to_uppercase().to_string())
                                                    .unwrap_or_default();
                                                let handle = user.handle.clone();
                                                let display_name = user.display_name.clone();
                                                view! {
                                                    <button class="w-full flex items-center gap-3 px-3 py-2 hover:bg-foreground/5 transition-colors">
                                                        <div class="relative">
                                                            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center text-xs font-bold text-brand">
                                                                {initials}
                                                            </div>
                                                            <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 status-online rounded-full border-2 border-panel"></div>
                                                        </div>
                                                        <div class="flex flex-col items-start">
                                                            <span class="text-sm font-medium text-foreground">{display_name}</span>
                                                            <span class="text-xs text-brand/70">{"@"}{handle}</span>
                                                        </div>
                                                    </button>
                                                }
                                            }).collect_view()}
                                        </div>
                                        <div class="px-3 py-2 border-t border-border/30 bg-foreground/5">
                                            <button class="w-full text-center text-sm text-brand hover:text-brand/80 transition-colors">
                                                "View all "{room.member_count}" members →"
                                            </button>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </div>

                    // Divider
                    <div class="w-px h-6 bg-border/50"></div>

                    // Search
                    <button 
                        class="p-2 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors"
                        title="Search in channel"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                    </button>

                    // Pin channel
                    <button 
                        class=move || format!(
                            "p-2 rounded-lg transition-colors {}",
                            if is_pinned.get() {
                                "text-brand bg-brand/10"
                            } else {
                                "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                            }
                        )
                        title=move || if is_pinned.get() { "Unpin channel" } else { "Pin channel" }
                        on:click=move |_| set_is_pinned.update(|v| *v = !*v)
                    >
                        <svg class="w-5 h-5" fill=move || if is_pinned.get() { "currentColor" } else { "none" } viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"></path>
                        </svg>
                    </button>

                    // Notifications
                    <button 
                        class="p-2 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors"
                        title="Notification settings"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                        </svg>
                    </button>

                    // Settings
                    <div class="relative">
                        <button 
                            class=move || format!(
                                "p-2 rounded-lg transition-colors {}",
                                if show_settings.get() {
                                    "bg-foreground/10 text-foreground"
                                } else {
                                    "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                                }
                            )
                            title="Channel settings"
                            on:click=move |_| set_show_settings.update(|v| *v = !*v)
                        >
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                            </svg>
                        </button>

                        // Settings dropdown
                        {move || {
                            if show_settings.get() {
                                view! {
                                    <div class="absolute top-full right-0 mt-2 w-56 bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-2">
                                        <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                            </svg>
                                            "Edit Channel"
                                        </button>
                                        <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"></path>
                                            </svg>
                                            "Invite Members"
                                        </button>
                                        <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"></path>
                                            </svg>
                                            "Pinned Messages"
                                        </button>
                                        <div class="my-1 border-t border-border/30"></div>
                                        <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-foreground/80 hover:bg-foreground/5 hover:text-foreground transition-colors">
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" clip-rule="evenodd"></path>
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2"></path>
                                            </svg>
                                            "Mute Channel"
                                        </button>
                                        <button class="w-full flex items-center gap-3 px-4 py-2 text-sm text-error hover:bg-error/10 transition-colors">
                                            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                                            </svg>
                                            "Leave Channel"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            // Connection status bar
            <div class="flex items-center gap-4 px-4 py-1.5 border-t border-border/20 bg-background/30">
                <div class="flex items-center gap-2 text-xs">
                    <span class="flex items-center gap-1.5 text-success">
                        <span class="w-1.5 h-1.5 rounded-full status-online animate-pulse"></span>
                        "Connected"
                    </span>
                    <span class="text-foreground/30">"|"</span>
                    <span class="text-foreground/40 font-mono">"P2P Mesh"</span>
                    <span class="text-foreground/30">"|"</span>
                    <span class="text-foreground/40 font-mono">"23ms latency"</span>
                </div>
                <div class="flex-1"></div>
                <div class="flex items-center gap-2 text-xs text-foreground/40">
                    <span class="font-mono">{room.member_count}" members"</span>
                    <span>"|"</span>
                    <span class="font-mono text-success/70">{room.online_count}" online"</span>
                </div>
            </div>
        </header>
    }
}

