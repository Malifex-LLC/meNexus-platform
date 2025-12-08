// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{ChatMessage, ChatRoom, ChatUser, TypingUser};
use crate::ui::components::{
    chat_input::ChatInput, chat_message::ChatMessageCard, typing_indicator::TypingIndicator,
};
use leptos::prelude::*;

/// Main chat feed component - responsive with collapsible channel selector
#[component]
pub fn ChatFeed() -> impl IntoView {
    // Mock data
    let rooms = ChatRoom::mock_rooms();
    let rooms_for_dropdown = rooms.clone();
    let rooms_for_sidebar = rooms.clone();
    let (active_room_id, set_active_room_id) = signal(rooms[0].id.clone());
    let (show_channel_dropdown, set_show_channel_dropdown) = signal(false);
    let (show_sidebar, set_show_sidebar) = signal(false);

    // Get active room
    let rooms_for_active = rooms.clone();
    let active_room = Memo::new(move |_| {
        let id = active_room_id.get();
        rooms_for_active
            .iter()
            .find(|r| r.id == id)
            .cloned()
            .unwrap_or_else(|| rooms_for_active[0].clone())
    });

    // Messages (would be fetched based on active room)
    let messages = ChatMessage::mock_messages();

    // Typing users (mock - would be from real-time updates)
    let (typing_users, _set_typing_users) = signal(TypingUser::mock_typing());

    // Room name signal for input
    let room_name = Signal::derive(move || active_room.get().name.clone());

    // Count total unread
    let total_unread: u32 = rooms.iter().map(|r| r.unread_count).sum();

    view! {
        <div class="flex h-full w-full bg-panel relative">
            // Mobile sidebar overlay
            {move || {
                if show_sidebar.get() {
                    view! {
                        <div
                            class="fixed inset-0 bg-black/50 z-40 lg:hidden"
                            on:click=move |_| set_show_sidebar.set(false)
                        ></div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Sidebar - Channel List (hidden on small screens, slide-in on mobile when toggled)
            <aside class=move || format!(
                "fixed lg:relative inset-y-0 left-0 z-50 w-64 flex-shrink-0 border-r border-border/50 bg-panel flex flex-col transform transition-transform duration-300 lg:translate-x-0 {}",
                if show_sidebar.get() { "translate-x-0" } else { "-translate-x-full" }
            )>
                // Sidebar header
                <div class="p-3 border-b border-border/50">
                    <div class="flex items-center justify-between mb-3">
                        <div class="flex items-center gap-2">
                            <div class="w-7 h-7 rounded-lg bg-gradient-to-br from-brand to-brand/60 flex items-center justify-center">
                                <svg class="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                                </svg>
                            </div>
                            <h2 class="font-semibold text-sm text-foreground">"Rooms"</h2>
                        </div>
                        <div class="flex items-center gap-1">
                            <button class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors" title="Create channel">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"></path>
                                </svg>
                            </button>
                            // Close button (mobile only)
                            <button
                                class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors lg:hidden"
                                on:click=move |_| set_show_sidebar.set(false)
                            >
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                    </div>

                    // Search channels
                    <div class="relative">
                        <svg class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                        </svg>
                        <input
                            type="text"
                            placeholder="Search..."
                            class="w-full pl-8 pr-2 py-1.5 bg-background/50 border border-border/50 rounded-lg text-xs text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 transition-all"
                        />
                    </div>
                </div>

                // Channel list
                <nav class="flex-1 overflow-y-auto scrollbar-thin p-1.5 space-y-0.5">
                    {rooms_for_sidebar.iter().map(|room| {
                        let room_id_for_button = room.id.clone();
                        let room_id_for_icon = room.id.clone();
                        let room_id_for_click = room.id.clone();
                        let room_name = room.name.clone();
                        let unread = room.unread_count;
                        let is_private = room.is_private;

                        view! {
                            <button
                                class=move || {
                                    let is_active = active_room_id.get() == room_id_for_button;
                                    format!(
                                        "w-full group flex items-center gap-2 px-2 py-1.5 rounded-lg text-left transition-all text-sm {}",
                                        if is_active {
                                            if is_private { "bg-amber-500/15 text-amber-400" } else { "bg-brand/15 text-brand" }
                                        } else {
                                            "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                                        }
                                    )
                                }
                                on:click=move |_| {
                                    set_active_room_id.set(room_id_for_click.clone());
                                    set_show_sidebar.set(false);
                                }
                            >
                                <span class=move || format!(
                                    "flex-shrink-0 w-5 h-5 rounded flex items-center justify-center text-xs font-mono transition-colors {}",
                                    if active_room_id.get() == room_id_for_icon {
                                        if is_private { "text-amber-400" } else { "text-brand" }
                                    } else {
                                        "text-foreground/40"
                                    }
                                )>
                                    {if is_private { "🔒" } else { "#" }}
                                </span>
                                <span class="flex-1 truncate font-medium">{room_name}</span>
                                {if unread > 0 {
                                    view! {
                                        <span class=format!(
                                            "min-w-4 h-4 px-1 flex items-center justify-center rounded-full text-[10px] font-bold text-white {}",
                                            if is_private { "bg-amber-500" } else { "bg-brand" }
                                        )>
                                            {unread}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </button>
                        }
                    }).collect_view()}
                </nav>

                // Sidebar footer
                <div class="p-2 border-t border-border/50 bg-background/30">
                    <div class="flex items-center justify-between text-[10px]">
                        <span class="flex items-center gap-1 text-emerald-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            "Online"
                        </span>
                        <span class="text-foreground/30 font-mono">{rooms.len()}" rooms"</span>
                    </div>
                </div>
            </aside>

            // Main Content - Chat Area
            <main class="flex-1 flex flex-col overflow-hidden min-w-0">
                // Compact header with channel selector (replaces full header on small screens)
                <header class="flex-shrink-0 flex items-center gap-2 px-2 py-1.5 border-b border-border/50 bg-panel/30">
                    // Hamburger menu (mobile only)
                    <button
                        class="p-1.5 rounded-lg text-foreground/50 hover:text-foreground hover:bg-foreground/5 transition-colors lg:hidden"
                        on:click=move |_| set_show_sidebar.set(true)
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path>
                        </svg>
                        {if total_unread > 0 {
                            view! {
                                <span class="absolute -top-1 -right-1 min-w-4 h-4 px-1 flex items-center justify-center rounded-full bg-brand text-white text-[10px] font-bold">
                                    {total_unread}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </button>

                    // Channel selector dropdown
                    <div class="relative flex-1 min-w-0">
                        <button
                            class="flex items-center gap-2 px-2 py-1 rounded-lg hover:bg-foreground/5 transition-colors w-full"
                            on:click=move |_| set_show_channel_dropdown.update(|v| *v = !*v)
                        >
                            <span class=move || format!(
                                "font-semibold text-sm {}",
                                if active_room.get().is_private { "text-amber-400" } else { "text-brand" }
                            )>
                                {move || if active_room.get().is_private { "🔒 " } else { "# " }}
                                {move || active_room.get().name}
                            </span>
                            <svg class=move || format!(
                                "w-4 h-4 text-foreground/40 transition-transform {}",
                                if show_channel_dropdown.get() { "rotate-180" } else { "" }
                            ) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                            </svg>
                        </button>

                        // Dropdown menu
                        {move || {
                            if show_channel_dropdown.get() {
                                view! {
                                    <div class="absolute top-full left-0 mt-1 w-56 max-h-64 overflow-y-auto bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-1 scrollbar-thin">
                                        {rooms_for_dropdown.iter().map(|room| {
                                            let room_id_for_class = room.id.clone();
                                            let room_id_for_click = room.id.clone();
                                            let room_name = room.name.clone();
                                            let unread = room.unread_count;
                                            let is_private = room.is_private;
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "w-full flex items-center gap-2 px-3 py-2 text-sm transition-colors {}",
                                                        if active_room_id.get() == room_id_for_class {
                                                            "bg-brand/10 text-brand"
                                                        } else {
                                                            "text-foreground/70 hover:bg-foreground/5 hover:text-foreground"
                                                        }
                                                    )
                                                    on:click=move |_| {
                                                        set_active_room_id.set(room_id_for_click.clone());
                                                        set_show_channel_dropdown.set(false);
                                                    }
                                                >
                                                    <span class="text-foreground/40">{if is_private { "🔒" } else { "#" }}</span>
                                                    <span class="flex-1 truncate text-left">{room_name}</span>
                                                    {if unread > 0 {
                                                        view! {
                                                            <span class="min-w-5 h-5 px-1.5 flex items-center justify-center rounded-full bg-brand text-white text-xs font-bold">
                                                                {unread}
                                                            </span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span></span> }.into_any()
                                                    }}
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

                    // Viewer/member count
                    <div class="hidden sm:flex items-center gap-1.5 px-2 py-1 rounded-lg bg-foreground/5 text-xs">
                        <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                        <span class="text-foreground/60">{move || active_room.get().online_count}" online"</span>
                    </div>

                    // Settings button
                    <button class="p-1.5 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                        </svg>
                    </button>
                </header>

                // Messages area
                <div class="flex-1 overflow-y-auto scrollbar-styled">
                    // Compact welcome message
                    <div class="p-3 sm:p-4 border-b border-border/30">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 sm:w-12 sm:h-12 rounded-xl bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center flex-shrink-0">
                                <span class="text-xl sm:text-2xl font-mono font-bold text-brand">"#"</span>
                            </div>
                            <div class="min-w-0">
                                <h2 class="text-base sm:text-lg font-bold text-foreground truncate">
                                    {move || active_room.get().name}
                                </h2>
                                <p class="text-xs sm:text-sm text-foreground/50 truncate">
                                    {move || active_room.get().description.unwrap_or_else(|| "Start the conversation!".to_string())}
                                </p>
                            </div>
                        </div>
                    </div>

                    // Date separator
                    <div class="flex items-center gap-2 px-3 py-2">
                        <div class="flex-1 h-px bg-border/30"></div>
                        <span class="text-[10px] font-medium text-foreground/40 px-2 py-0.5 rounded-full bg-foreground/5">
                            "Today"
                        </span>
                        <div class="flex-1 h-px bg-border/30"></div>
                    </div>

                    // Messages list
                    <div class="p-2 m-4 space-y-8">
                        {messages.iter().enumerate().map(|(i, message)| {
                            let show_author = if i == 0 {
                                true
                            } else {
                                messages[i - 1].author.id != message.author.id ||
                                messages[i - 1].message_type != message.message_type
                            };
                            let message = message.clone();
                            let is_own = message.author.handle == "neo";

                            view! {
                                <ChatMessageCard
                                    message=message
                                    is_own_message=is_own
                                    show_author=show_author
                                />
                            }
                        }).collect_view()}
                    </div>

                    <div id="chat-bottom"></div>
                </div>

                // Typing indicator
                <TypingIndicator typing_users=typing_users />

                // Chat input
                <ChatInput room_name=room_name />
            </main>
        </div>
    }
}
