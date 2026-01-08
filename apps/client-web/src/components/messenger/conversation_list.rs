// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::*;
use leptos::prelude::*;

/// Individual conversation item in the list
#[component]
pub fn ConversationItem(
    conversation: Conversation,
    #[prop(into)] is_selected: Signal<bool>,
    #[prop(into)] on_select: Callback<String>,
) -> impl IntoView {
    let conv_id = conversation.id.clone();
    let display_name = conversation.display_name();
    let is_group = conversation.conversation_type == ConversationType::Group;
    let is_online = conversation.is_online();
    let online_count = conversation.online_count();
    let participant_count = conversation.participants.len();
    
    // Get initials for avatar
    let initials: String = display_name
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();

    let last_message = conversation.last_message.clone().unwrap_or_default();
    let last_time = conversation.last_message_time.clone();
    let unread = conversation.unread_count;
    let is_muted = conversation.is_muted;
    let is_pinned = conversation.is_pinned;
    let is_encrypted = conversation.is_encrypted;

    view! {
        <button
            class=move || format!(
                "w-full flex items-center gap-3 p-3 rounded-xl transition-all {}",
                if is_selected.get() {
                    "bg-brand/15 border border-brand/30"
                } else {
                    "hover:bg-foreground/5 border border-transparent"
                }
            )
            on:click=move |_| on_select.run(conv_id.clone())
        >
            // Avatar
            <div class="relative flex-shrink-0">
                {if is_group {
                    view! {
                        <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-secondary/20 to-secondary/5 flex items-center justify-center ring-2 ring-border/30">
                            <svg class="w-6 h-6 text-secondary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30">
                            <span class="text-brand font-bold text-sm">{initials.clone()}</span>
                        </div>
                    }.into_any()
                }}
                
                // Online indicator
                {if is_online && !is_group {
                    view! {
                        <span class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full status-online ring-2 ring-panel"></span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Pinned indicator
                {if is_pinned {
                    view! {
                        <span class="absolute -top-1 -left-1 w-5 h-5 rounded-full bg-warning/20 flex items-center justify-center">
                            <svg class="w-2.5 h-2.5 text-warning" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
                            </svg>
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Content
            <div class="flex-1 min-w-0 text-left">
                <div class="flex items-center justify-between gap-2 mb-0.5">
                    <div class="flex items-center gap-1.5 min-w-0">
                        <span class=move || format!(
                            "font-semibold truncate {}",
                            if unread > 0 { "text-foreground" } else { "text-foreground/80" }
                        )>
                            {display_name.clone()}
                        </span>
                        
                        // Verified badge
                        {if !is_group && conversation.participants.first().map(|p| p.is_verified).unwrap_or(false) {
                            view! {
                                <svg class="w-4 h-4 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Encryption indicator
                        {if is_encrypted {
                            view! {
                                <svg class="w-3.5 h-3.5 text-success flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Muted indicator
                        {if is_muted {
                            view! {
                                <svg class="w-3.5 h-3.5 text-foreground/30 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" clip-rule="evenodd"/>
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    <time class="text-xs text-foreground/40 flex-shrink-0">{last_time}</time>
                </div>

                <div class="flex items-center justify-between gap-2">
                    <p class=move || format!(
                        "text-sm truncate {}",
                        if unread > 0 { "text-foreground/70" } else { "text-foreground/50" }
                    )>
                        {last_message}
                    </p>
                    
                    <div class="flex items-center gap-1.5 flex-shrink-0">
                        // Group online count
                        {if is_group {
                            view! {
                                <span class="text-[10px] text-success font-medium">
                                    {online_count}"/"{ participant_count }
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Unread badge
                        {if unread > 0 {
                            view! {
                                <span class="min-w-5 h-5 px-1.5 rounded-full bg-brand flex items-center justify-center text-[10px] font-bold text-white">
                                    {if unread > 99 { "99+".to_string() } else { unread.to_string() }}
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>
            </div>
        </button>
    }
}

/// Conversation list sidebar
#[component]
pub fn ConversationList(
    #[prop(into)] selected_id: RwSignal<Option<String>>,
    #[prop(into)] on_new_chat: Callback<()>,
) -> impl IntoView {
    let conversations = Conversation::mock_conversations();
    let (filter, set_filter) = signal(ConversationFilter::All);
    let (search_query, set_search_query) = signal(String::new());

    // Filter conversations
    let filtered_conversations = {
        let convs = conversations.clone();
        move || {
            let query = search_query.get().to_lowercase();
            let f = filter.get();
            
            convs.iter()
                .filter(|c| {
                    // Filter by type
                    let type_match = match f {
                        ConversationFilter::All => !c.is_archived,
                        ConversationFilter::Unread => c.unread_count > 0,
                        ConversationFilter::Direct => c.conversation_type == ConversationType::Direct && !c.is_archived,
                        ConversationFilter::Groups => c.conversation_type == ConversationType::Group && !c.is_archived,
                        ConversationFilter::Archived => c.is_archived,
                    };
                    
                    // Filter by search query
                    let search_match = if query.is_empty() {
                        true
                    } else {
                        c.display_name().to_lowercase().contains(&query)
                    };
                    
                    type_match && search_match
                })
                .cloned()
                .collect::<Vec<_>>()
        }
    };

    let on_select = Callback::new(move |id: String| {
        selected_id.set(Some(id));
    });

    view! {
        <div class="flex flex-col h-full bg-panel border-r border-border/50">
            // Header
            <div class="flex-shrink-0 p-4 border-b border-border/30">
                <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center gap-2">
                        <h2 class="text-lg font-bold text-foreground">"Messages"</h2>
                        <span class="px-2 py-0.5 rounded-full bg-brand/15 text-brand text-xs font-semibold">
                            "16"
                        </span>
                    </div>
                    <button 
                        class="p-2 rounded-lg bg-brand text-white hover:bg-brand/90 transition-colors"
                        on:click=move |_| on_new_chat.run(())
                        title="New message"
                    >
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
                        </svg>
                    </button>
                </div>

                // Search
                <div class="relative">
                    <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                    </svg>
                    <input
                        type="text"
                        placeholder="Search conversations..."
                        class="w-full pl-10 pr-4 py-2.5 rounded-xl bg-foreground/5 border border-border/50 text-sm text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                        prop:value=move || search_query.get()
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                    />
                </div>
            </div>

            // Filter tabs
            <div class="flex-shrink-0 px-4 py-2 border-b border-border/30 overflow-x-auto scrollbar-thin">
                <div class="flex gap-1">
                    {ConversationFilter::all().into_iter().map(|f| {
                        view! {
                            <button
                                class=move || format!(
                                    "px-3 py-1.5 rounded-lg text-xs font-medium whitespace-nowrap transition-colors {}",
                                    if filter.get() == f {
                                        "bg-brand/15 text-brand"
                                    } else {
                                        "text-foreground/50 hover:text-foreground hover:bg-foreground/5"
                                    }
                                )
                                on:click=move |_| set_filter.set(f)
                            >
                                {f.label()}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Conversation list
            <div class="flex-1 overflow-y-auto scrollbar-thin p-2 space-y-1">
                {move || {
                    let convs = filtered_conversations();
                    if convs.is_empty() {
                        view! {
                            <div class="flex flex-col items-center justify-center py-12 text-center">
                                <div class="w-16 h-16 rounded-2xl bg-foreground/5 flex items-center justify-center mb-4">
                                    <svg class="w-8 h-8 text-foreground/20" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                                    </svg>
                                </div>
                                <p class="text-sm text-foreground/40">"No conversations found"</p>
                            </div>
                        }.into_any()
                    } else {
                        // Separate pinned and regular conversations
                        let pinned: Vec<_> = convs.iter().filter(|c| c.is_pinned).cloned().collect();
                        let regular: Vec<_> = convs.iter().filter(|c| !c.is_pinned).cloned().collect();
                        let has_pinned = !pinned.is_empty();

                        view! {
                            <div class="space-y-1">
                                // Pinned section
                                {if has_pinned {
                                    view! {
                                        <div class="mb-2">
                                            <p class="px-3 py-1 text-[10px] font-semibold text-foreground/30 uppercase tracking-wider">"Pinned"</p>
                                            {pinned.into_iter().map(|conv| {
                                                let id = conv.id.clone();
                                                let is_selected = Signal::derive(move || selected_id.get() == Some(id.clone()));
                                                view! {
                                                    <ConversationItem 
                                                        conversation=conv 
                                                        is_selected=is_selected
                                                        on_select=on_select
                                                    />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}

                                // Regular conversations
                                {if !regular.is_empty() {
                                    view! {
                                        <div>
                                            {if has_pinned {
                                                view! {
                                                    <p class="px-3 py-1 text-[10px] font-semibold text-foreground/30 uppercase tracking-wider">"Recent"</p>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                            {regular.into_iter().map(|conv| {
                                                let id = conv.id.clone();
                                                let is_selected = Signal::derive(move || selected_id.get() == Some(id.clone()));
                                                view! {
                                                    <ConversationItem 
                                                        conversation=conv 
                                                        is_selected=is_selected
                                                        on_select=on_select
                                                    />
                                                }
                                            }).collect_view()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Footer with encryption status
            <div class="flex-shrink-0 p-3 border-t border-border/30 bg-success/5">
                <div class="flex items-center gap-2 text-xs">
                    <svg class="w-4 h-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                    </svg>
                    <span class="text-success font-medium">"End-to-end encrypted"</span>
                </div>
            </div>
        </div>
    }
}
