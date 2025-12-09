// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::messenger::{Conversation, ConversationList, MessageThread};
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;

#[component]
pub fn MessengerLayout() -> impl IntoView {
    let selected_conversation_id = RwSignal::new(Some("conv-001".to_string()));
    let (show_new_chat_modal, set_show_new_chat_modal) = signal(false);

    let conversations = Conversation::mock_conversations();

    let on_new_chat = Callback::new(move |_: ()| {
        set_show_new_chat_modal.set(true);
    });

    view! {
        <MainLayout>
            <div class="h-full w-full flex overflow-hidden bg-background">
                // Conversation list sidebar
                <div class="w-80 lg:w-96 flex-shrink-0 hidden md:block">
                    <ConversationList
                        selected_id=selected_conversation_id
                        on_new_chat=on_new_chat
                    />
                </div>

                // Main chat area
                <div class="flex-1 min-w-0">
                    {move || {
                        let selected_id = selected_conversation_id.get();
                        if let Some(id) = selected_id {
                            let convs = conversations.clone();
                            if let Some(conv) = convs.into_iter().find(|c| c.id == id) {
                                view! {
                                    <MessageThread conversation=conv />
                                }.into_any()
                            } else {
                                view! {
                                    <EmptyState />
                                }.into_any()
                            }
                        } else {
                            view! {
                                <EmptyState />
                            }.into_any()
                        }
                    }}
                </div>

                // New chat modal
                {move || {
                    if show_new_chat_modal.get() {
                        view! {
                            <NewChatModal on_close=move || set_show_new_chat_modal.set(false) />
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>
        </MainLayout>
    }
}

/// Empty state when no conversation is selected
#[component]
fn EmptyState() -> impl IntoView {
    view! {
        <div class="h-full flex flex-col items-center justify-center bg-panel/50 p-8">
            <div class="w-24 h-24 rounded-3xl bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center mb-6 ring-4 ring-border/20">
                <svg class="w-12 h-12 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                </svg>
            </div>
            <h2 class="text-xl font-bold text-foreground mb-2">"Welcome to Messages"</h2>
            <p class="text-foreground/50 text-center max-w-sm mb-6">
                "Send private, encrypted messages to friends and create group conversations."
            </p>

            // Features list
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 max-w-2xl mb-8">
                <div class="flex flex-col items-center text-center p-4 rounded-xl bg-card border border-border/50">
                    <div class="w-12 h-12 rounded-xl bg-emerald-500/15 flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                        </svg>
                    </div>
                    <h3 class="font-semibold text-foreground text-sm mb-1">"End-to-End Encrypted"</h3>
                    <p class="text-xs text-foreground/50">"Quantum-resistant encryption keeps your messages private"</p>
                </div>

                <div class="flex flex-col items-center text-center p-4 rounded-xl bg-card border border-border/50">
                    <div class="w-12 h-12 rounded-xl bg-violet-500/15 flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                    </div>
                    <h3 class="font-semibold text-foreground text-sm mb-1">"Group Chats"</h3>
                    <p class="text-xs text-foreground/50">"Create groups with multiple participants"</p>
                </div>

                <div class="flex flex-col items-center text-center p-4 rounded-xl bg-card border border-border/50">
                    <div class="w-12 h-12 rounded-xl bg-sky-500/15 flex items-center justify-center mb-3">
                        <svg class="w-6 h-6 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                        </svg>
                    </div>
                    <h3 class="font-semibold text-foreground text-sm mb-1">"Voice & Video"</h3>
                    <p class="text-xs text-foreground/50">"Secure voice and video calls"</p>
                </div>
            </div>

            <button class="px-6 py-3 rounded-xl bg-brand text-white font-semibold hover:bg-brand/90 transition-colors flex items-center gap-2">
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"/>
                </svg>
                "Start a Conversation"
            </button>
        </div>
    }
}

/// Modal for creating a new chat
#[component]
fn NewChatModal(
    #[prop(into)] on_close: Callback<()>,
) -> impl IntoView {
    let (search_query, set_search_query) = signal(String::new());
    let (selected_users, set_selected_users) = signal(Vec::<String>::new());
    let (is_group, set_is_group) = signal(false);
    let (group_name, set_group_name) = signal(String::new());

    // Mock users for selection - stored for multiple accesses
    let users = StoredValue::new(vec![
        ("user-001", "Morpheus", "morpheus", true),
        ("user-002", "Trinity", "trinity", true),
        ("user-003", "Neo", "neo", false),
        ("user-004", "The Oracle", "oracle", false),
        ("user-005", "Tank", "tank", true),
        ("user-006", "Niobe", "niobe", false),
        ("user-007", "Cypher", "cypher", false),
    ]);

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center p-4">
            // Backdrop
            <div 
                class="absolute inset-0 bg-black/60 backdrop-blur-sm"
                on:click=move |_| on_close.run(())
            ></div>

            // Modal
            <div class="relative w-full max-w-md bg-panel border border-border/50 rounded-2xl shadow-2xl overflow-hidden">
                // Header
                <div class="flex items-center justify-between p-4 border-b border-border/30">
                    <h2 class="text-lg font-bold text-foreground">"New Message"</h2>
                    <button 
                        class="p-2 rounded-lg hover:bg-foreground/10 transition-colors"
                        on:click=move |_| on_close.run(())
                    >
                        <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                // Group toggle
                <div class="p-4 border-b border-border/30">
                    <div class="flex items-center gap-3">
                        <button
                            class=move || format!(
                                "flex-1 py-2.5 rounded-xl text-sm font-medium transition-colors {}",
                                if !is_group.get() { "bg-brand text-white" } else { "bg-foreground/5 text-foreground/60 hover:text-foreground" }
                            )
                            on:click=move |_| set_is_group.set(false)
                        >
                            "Direct Message"
                        </button>
                        <button
                            class=move || format!(
                                "flex-1 py-2.5 rounded-xl text-sm font-medium transition-colors {}",
                                if is_group.get() { "bg-brand text-white" } else { "bg-foreground/5 text-foreground/60 hover:text-foreground" }
                            )
                            on:click=move |_| set_is_group.set(true)
                        >
                            "Group Chat"
                        </button>
                    </div>

                    // Group name input
                    {move || {
                        if is_group.get() {
                            view! {
                                <input
                                    type="text"
                                    placeholder="Group name"
                                    class="w-full mt-3 px-4 py-2.5 rounded-xl bg-foreground/5 border border-border/50 text-sm text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                                    prop:value=move || group_name.get()
                                    on:input=move |ev| set_group_name.set(event_target_value(&ev))
                                />
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>

                // Search
                <div class="p-4 border-b border-border/30">
                    <div class="relative">
                        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                        </svg>
                        <input
                            type="text"
                            placeholder="Search users..."
                            class="w-full pl-10 pr-4 py-2.5 rounded-xl bg-foreground/5 border border-border/50 text-sm text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        />
                    </div>

                    // Selected users chips
                    {move || {
                        let selected = selected_users.get();
                        let users_list = users.get_value();
                        if !selected.is_empty() {
                            view! {
                                <div class="flex flex-wrap gap-2 mt-3">
                                    {selected.into_iter().map(|id| {
                                        let user = users_list.iter().find(|(uid, _, _, _)| *uid == id);
                                        if let Some((_, name, _, _)) = user {
                                            let id_clone = id.clone();
                                            view! {
                                                <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-brand/15 text-brand text-sm">
                                                    {*name}
                                                    <button 
                                                        class="p-0.5 rounded-full hover:bg-brand/20"
                                                        on:click=move |_| {
                                                            set_selected_users.update(|v| v.retain(|x| x != &id_clone));
                                                        }
                                                    >
                                                        <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                                        </svg>
                                                    </button>
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    }}
                </div>

                // User list
                <div class="max-h-64 overflow-y-auto scrollbar-thin">
                    {users.get_value().into_iter().map(|(id, name, handle, is_online)| {
                        let id_str = id.to_string();
                        let id_for_check = id_str.clone();
                        let id_for_toggle = id_str.clone();
                        let initials: String = name.chars().take(2).collect::<String>().to_uppercase();
                        
                        view! {
                            <button
                                class=move || format!(
                                    "w-full flex items-center gap-3 p-3 transition-colors {}",
                                    if selected_users.get().contains(&id_for_check) {
                                        "bg-brand/10"
                                    } else {
                                        "hover:bg-foreground/5"
                                    }
                                )
                                on:click=move |_| {
                                    let id = id_for_toggle.clone();
                                    set_selected_users.update(|v| {
                                        if v.contains(&id) {
                                            v.retain(|x| x != &id);
                                        } else {
                                            if !is_group.get() {
                                                v.clear();
                                            }
                                            v.push(id);
                                        }
                                    });
                                }
                            >
                                <div class="relative">
                                    <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                        <span class="text-brand font-bold text-sm">{initials}</span>
                                    </div>
                                    {if is_online {
                                        view! {
                                            <span class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full bg-emerald-500 ring-2 ring-panel"></span>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                </div>
                                <div class="flex-1 text-left">
                                    <p class="font-medium text-foreground">{name}</p>
                                    <p class="text-xs text-foreground/50">{"@"}{handle}</p>
                                </div>
                                {move || {
                                    if selected_users.get().contains(&id_str) {
                                        view! {
                                            <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Footer
                <div class="p-4 border-t border-border/30 bg-card/50">
                    <button
                        class=move || format!(
                            "w-full py-3 rounded-xl font-semibold transition-colors {}",
                            if selected_users.get().is_empty() {
                                "bg-brand/30 text-white/50 cursor-not-allowed"
                            } else {
                                "bg-brand text-white hover:bg-brand/90"
                            }
                        )
                        prop:disabled=move || selected_users.get().is_empty()
                    >
                        {move || {
                            if is_group.get() {
                                format!("Create Group ({})", selected_users.get().len())
                            } else {
                                "Start Conversation".to_string()
                            }
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}
