// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::*;
use leptos::prelude::*;

/// Individual message bubble
#[component]
pub fn MessageBubble(message: Message) -> impl IntoView {
    let is_own = message.is_own;
    let sender_name = message.sender_name.clone();
    let timestamp = message.timestamp.clone();
    let status = message.status;
    let reactions = message.reactions.clone();
    let is_encrypted = message.is_encrypted;

    let initials: String = sender_name
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    view! {
        <div class=format!(
            "flex gap-2 group {}",
            if is_own { "flex-row-reverse" } else { "" }
        )>
            // Avatar (only for others' messages)
            {if !is_own {
                view! {
                    <div class="flex-shrink-0 w-8 h-8 rounded-lg bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center self-end mb-1">
                        <span class="text-brand font-bold text-[10px]">{initials}</span>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Message content
            <div class=format!(
                "max-w-[70%] {}",
                if is_own { "items-end" } else { "items-start" }
            )>
                // Sender name (only for others)
                {if !is_own {
                    view! {
                        <p class="text-[10px] font-medium text-foreground/40 mb-1 ml-1">{sender_name.clone()}</p>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Message bubble
                {match message.content {
                    MessageContent::Text(text) => {
                        view! {
                            <div class=format!(
                                "px-4 py-2.5 rounded-2xl {}",
                                if is_own {
                                    "bg-brand text-white rounded-br-md"
                                } else {
                                    "bg-card border border-border/50 text-foreground rounded-bl-md"
                                }
                            )>
                                <p class="text-sm leading-relaxed whitespace-pre-wrap">{text}</p>
                            </div>
                        }.into_any()
                    },
                    MessageContent::Image { url, caption } => {
                        let cap = caption.clone();
                        view! {
                            <div class=format!(
                                "rounded-2xl overflow-hidden {}",
                                if is_own { "rounded-br-md" } else { "rounded-bl-md" }
                            )>
                                <div class="w-64 h-48 bg-foreground/10 flex items-center justify-center">
                                    <svg class="w-12 h-12 text-foreground/20" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                    </svg>
                                </div>
                                {if let Some(c) = cap {
                                    view! {
                                        <div class=format!(
                                            "px-4 py-2 {}",
                                            if is_own { "bg-brand/90 text-white/90" } else { "bg-card border-x border-b border-border/50 text-foreground/70" }
                                        )>
                                            <p class="text-sm">{c}</p>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }.into_any()
                    },
                    MessageContent::File { name, size, file_type } => {
                        view! {
                            <div class=format!(
                                "flex items-center gap-3 px-4 py-3 rounded-2xl {}",
                                if is_own {
                                    "bg-brand text-white rounded-br-md"
                                } else {
                                    "bg-card border border-border/50 rounded-bl-md"
                                }
                            )>
                                <div class=format!(
                                    "w-10 h-10 rounded-lg flex items-center justify-center {}",
                                    if is_own { "bg-white/20" } else { "bg-foreground/10" }
                                )>
                                    <svg class=format!("w-5 h-5 {}", if is_own { "text-white" } else { "text-foreground/60" }) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                    </svg>
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class=format!("text-sm font-medium truncate {}", if is_own { "" } else { "text-foreground" })>{name}</p>
                                    <p class=format!("text-xs {}", if is_own { "text-white/70" } else { "text-foreground/50" })>
                                        {file_type}" · "{size}
                                    </p>
                                </div>
                                <button class=format!(
                                    "p-2 rounded-lg transition-colors {}",
                                    if is_own { "hover:bg-white/20" } else { "hover:bg-foreground/10" }
                                )>
                                    <svg class=format!("w-5 h-5 {}", if is_own { "text-white" } else { "text-foreground/60" }) fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                                    </svg>
                                </button>
                            </div>
                        }.into_any()
                    },
                    MessageContent::Voice { duration } => {
                        view! {
                            <div class=format!(
                                "flex items-center gap-3 px-4 py-3 rounded-2xl min-w-[200px] {}",
                                if is_own {
                                    "bg-brand text-white rounded-br-md"
                                } else {
                                    "bg-card border border-border/50 rounded-bl-md"
                                }
                            )>
                                <button class=format!(
                                    "w-10 h-10 rounded-full flex items-center justify-center {}",
                                    if is_own { "bg-white/20 hover:bg-white/30" } else { "bg-foreground/10 hover:bg-foreground/20" }
                                )>
                                    <svg class=format!("w-5 h-5 {}", if is_own { "text-white" } else { "text-foreground" }) fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M8 5v14l11-7z"/>
                                    </svg>
                                </button>
                                <div class="flex-1">
                                    <div class=format!(
                                        "h-1 rounded-full {}",
                                        if is_own { "bg-white/30" } else { "bg-foreground/20" }
                                    )>
                                        <div class=format!(
                                            "h-full w-1/3 rounded-full {}",
                                            if is_own { "bg-white" } else { "bg-brand" }
                                        )></div>
                                    </div>
                                </div>
                                <span class=format!(
                                    "text-xs font-mono {}",
                                    if is_own { "text-white/70" } else { "text-foreground/50" }
                                )>
                                    {duration}
                                </span>
                            </div>
                        }.into_any()
                    },
                    MessageContent::System(text) => {
                        view! {
                            <div class="flex items-center justify-center gap-2 py-2 px-4 mx-auto">
                                <svg class="w-4 h-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                </svg>
                                <p class="text-xs text-foreground/40 text-center">{text}</p>
                            </div>
                        }.into_any()
                    },
                    MessageContent::Reply { original_sender, original_text, reply_text } => {
                        view! {
                            <div class=format!(
                                "rounded-2xl overflow-hidden {}",
                                if is_own { "rounded-br-md" } else { "rounded-bl-md" }
                            )>
                                // Original message preview
                                <div class=format!(
                                    "px-4 py-2 border-l-2 {}",
                                    if is_own {
                                        "bg-brand/80 border-white/50"
                                    } else {
                                        "bg-foreground/5 border-brand/50"
                                    }
                                )>
                                    <p class=format!(
                                        "text-[10px] font-medium {}",
                                        if is_own { "text-white/70" } else { "text-brand" }
                                    )>
                                        {original_sender}
                                    </p>
                                    <p class=format!(
                                        "text-xs truncate {}",
                                        if is_own { "text-white/60" } else { "text-foreground/50" }
                                    )>
                                        {original_text}
                                    </p>
                                </div>
                                // Reply text
                                <div class=format!(
                                    "px-4 py-2.5 {}",
                                    if is_own { "bg-brand text-white" } else { "bg-card border border-t-0 border-border/50 text-foreground" }
                                )>
                                    <p class="text-sm leading-relaxed">{reply_text}</p>
                                </div>
                            </div>
                        }.into_any()
                    },
                }}

                // Reactions
                {if !reactions.is_empty() {
                    view! {
                        <div class=format!(
                            "flex items-center gap-1 mt-1 {}",
                            if is_own { "justify-end mr-1" } else { "ml-1" }
                        )>
                            {reactions.iter().map(|r| {
                                view! {
                                    <button class=format!(
                                        "flex items-center gap-1 px-2 py-0.5 rounded-full text-xs {}",
                                        if r.reacted_by_me {
                                            "bg-brand/20 border border-brand/30"
                                        } else {
                                            "bg-foreground/10 border border-border/30"
                                        }
                                    )>
                                        <span>{r.emoji.clone()}</span>
                                        <span class="text-foreground/60">{r.count}</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Timestamp and status
                <div class=format!(
                    "flex items-center gap-1.5 mt-1 {}",
                    if is_own { "justify-end mr-1" } else { "ml-1" }
                )>
                    // Encryption indicator
                    {if is_encrypted {
                        view! {
                            <svg class="w-3 h-3 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                            </svg>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    <time class="text-[10px] text-foreground/40">{timestamp}</time>
                    
                    // Status (only for own messages)
                    {if is_own {
                        view! {
                            <svg 
                                class=format!("w-4 h-4 {}", status.color_class())
                                fill="none" 
                                viewBox="0 0 24 24" 
                                stroke="currentColor" 
                                stroke-width="2"
                                inner_html=status.icon_svg()
                            ></svg>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>

            // Message actions (shown on hover)
            <div class=format!(
                "flex-shrink-0 self-center opacity-0 group-hover:opacity-100 transition-opacity {}",
                if is_own { "order-first" } else { "" }
            )>
                <div class="flex items-center gap-0.5 p-1 rounded-lg bg-panel border border-border/50 shadow-lg">
                    <button class="p-1.5 rounded hover:bg-foreground/10 transition-colors" title="React">
                        <svg class="w-4 h-4 text-foreground/50" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </button>
                    <button class="p-1.5 rounded hover:bg-foreground/10 transition-colors" title="Reply">
                        <svg class="w-4 h-4 text-foreground/50" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/>
                        </svg>
                    </button>
                    <button class="p-1.5 rounded hover:bg-foreground/10 transition-colors" title="More">
                        <svg class="w-4 h-4 text-foreground/50" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"/>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Chat header with recipient info
#[component]
pub fn ChatHeader(
    conversation: Conversation,
    #[prop(into)] on_info_click: Callback<()>,
) -> impl IntoView {
    let display_name = conversation.display_name();
    let is_group = conversation.conversation_type == ConversationType::Group;
    let is_online = conversation.is_online();
    let online_count = conversation.online_count();
    let participant_count = conversation.participants.len();
    let is_encrypted = conversation.is_encrypted;

    let initials: String = display_name
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();

    let status_text = if is_group {
        format!("{} members · {} online", participant_count, online_count)
    } else if is_online {
        "Online".to_string()
    } else {
        conversation.participants.first()
            .and_then(|p| p.last_seen.clone())
            .unwrap_or_else(|| "Offline".to_string())
    };

    view! {
        <header class="flex-shrink-0 flex items-center justify-between px-4 py-3 bg-panel border-b border-border/50">
            // Left - Avatar and info
            <div class="flex items-center gap-3">
                // Back button (mobile)
                <button class="lg:hidden p-2 -ml-2 rounded-lg hover:bg-foreground/10 transition-colors">
                    <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"/>
                    </svg>
                </button>

                // Avatar
                <div class="relative">
                    {if is_group {
                        view! {
                            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-secondary/20 to-secondary/5 flex items-center justify-center ring-2 ring-border/30">
                                <svg class="w-5 h-5 text-secondary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                                </svg>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30">
                                <span class="text-brand font-bold text-sm">{initials}</span>
                            </div>
                        }.into_any()
                    }}
                    
                    {if is_online && !is_group {
                        view! {
                            <span class="absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full status-online ring-2 ring-panel"></span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Name and status
                <div>
                    <div class="flex items-center gap-1.5">
                        <h3 class="font-semibold text-foreground">{display_name}</h3>
                        {if is_encrypted {
                            view! {
                                <svg class="w-4 h-4 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    <p class=format!(
                        "text-xs {}",
                        if is_online { "text-success" } else { "text-foreground/50" }
                    )>
                        {status_text}
                    </p>
                </div>
            </div>

            // Right - Actions
            <div class="flex items-center gap-1">
                <button class="p-2 rounded-lg hover:bg-foreground/10 transition-colors" title="Voice call">
                    <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"/>
                    </svg>
                </button>
                <button class="p-2 rounded-lg hover:bg-foreground/10 transition-colors" title="Video call">
                    <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                    </svg>
                </button>
                <button class="p-2 rounded-lg hover:bg-foreground/10 transition-colors" title="Search">
                    <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                    </svg>
                </button>
                <button 
                    class="p-2 rounded-lg hover:bg-foreground/10 transition-colors" 
                    title="Info"
                    on:click=move |_| on_info_click.run(())
                >
                    <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                    </svg>
                </button>
            </div>
        </header>
    }
}

/// Message thread component
#[component]
pub fn MessageThread(
    conversation: Conversation,
) -> impl IntoView {
    let messages = Message::mock_messages();
    let (show_info, set_show_info) = signal(false);

    let on_info_click = Callback::new(move |_: ()| {
        set_show_info.update(|v| *v = !*v);
    });

    view! {
        <div class="flex h-full">
            // Main chat area
            <div class="flex-1 flex flex-col min-w-0">
                // Header
                <ChatHeader 
                    conversation=conversation.clone() 
                    on_info_click=on_info_click
                />

                // Messages
                <div class="flex-1 overflow-y-auto scrollbar-thin p-4 space-y-4 bg-background">
                    // Date separator
                    <div class="flex items-center gap-4 py-2">
                        <span class="flex-1 h-px bg-border/30"></span>
                        <span class="text-xs text-foreground/40 font-medium">"Today"</span>
                        <span class="flex-1 h-px bg-border/30"></span>
                    </div>

                    {messages.into_iter().map(|msg| {
                        view! { <MessageBubble message=msg /> }
                    }).collect_view()}

                    // Typing indicator
                    <div class="flex items-center gap-2">
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                            <span class="text-brand font-bold text-[10px]">"MO"</span>
                        </div>
                        <div class="px-4 py-2.5 rounded-2xl rounded-bl-md bg-card border border-border/50">
                            <div class="flex items-center gap-1">
                                <span class="w-2 h-2 rounded-full bg-foreground/30 animate-bounce" style="animation-delay: 0ms"></span>
                                <span class="w-2 h-2 rounded-full bg-foreground/30 animate-bounce" style="animation-delay: 150ms"></span>
                                <span class="w-2 h-2 rounded-full bg-foreground/30 animate-bounce" style="animation-delay: 300ms"></span>
                            </div>
                        </div>
                    </div>
                </div>

                // Composer
                <MessageComposer />
            </div>

            // Info panel (conditionally shown)
            {move || {
                if show_info.get() {
                    view! {
                        <ConversationInfo conversation=conversation.clone() />
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Message composer input
#[component]
pub fn MessageComposer() -> impl IntoView {
    let (message, set_message) = signal(String::new());
    let (is_recording, set_is_recording) = signal(false);

    view! {
        <div class="flex-shrink-0 p-4 border-t border-border/50 bg-panel">
            // Reply preview (would be shown when replying)
            // <div class="mb-3 p-2 rounded-lg bg-foreground/5 border-l-2 border-brand">...</div>

            <div class="flex items-end gap-2">
                // Attachment button
                <button class="p-2.5 rounded-xl hover:bg-foreground/10 transition-colors text-foreground/50 hover:text-foreground">
                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"/>
                    </svg>
                </button>

                // Input area
                <div class="flex-1 relative">
                    <textarea
                        class="w-full px-4 py-3 pr-24 rounded-2xl bg-foreground/5 border border-border/50 text-sm text-foreground placeholder-foreground/30 resize-none focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all max-h-32"
                        placeholder="Type a message..."
                        rows="1"
                        prop:value=move || message.get()
                        on:input=move |ev| set_message.set(event_target_value(&ev))
                    ></textarea>

                    // Inline actions
                    <div class="absolute right-2 bottom-2 flex items-center gap-1">
                        <button class="p-1.5 rounded-lg hover:bg-foreground/10 transition-colors text-foreground/40 hover:text-foreground">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                            </svg>
                        </button>
                        <button class="p-1.5 rounded-lg hover:bg-foreground/10 transition-colors text-foreground/40 hover:text-foreground">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>

                // Send/Voice button
                {move || {
                    if message.get().trim().is_empty() {
                        view! {
                            <button 
                                class=format!(
                                    "p-2.5 rounded-xl transition-all {}",
                                    if is_recording.get() {
                                        "bg-error text-white animate-pulse"
                                    } else {
                                        "hover:bg-foreground/10 text-foreground/50 hover:text-foreground"
                                    }
                                )
                                on:click=move |_| set_is_recording.update(|v| *v = !*v)
                            >
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"/>
                                </svg>
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <button class="p-2.5 rounded-xl bg-brand text-white hover:bg-brand/90 transition-colors">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"/>
                                </svg>
                            </button>
                        }.into_any()
                    }
                }}
            </div>

            // Encryption status
            <div class="flex items-center justify-center gap-1.5 mt-2">
                <svg class="w-3 h-3 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                </svg>
                <span class="text-[10px] text-foreground/40">"Messages are end-to-end encrypted"</span>
            </div>
        </div>
    }
}

/// Conversation info sidebar
#[component]
pub fn ConversationInfo(conversation: Conversation) -> impl IntoView {
    let display_name = conversation.display_name();
    let is_group = conversation.conversation_type == ConversationType::Group;
    let participants = conversation.participants.clone();

    let initials: String = display_name
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .collect::<String>()
        .to_uppercase();

    view! {
        <aside class="w-80 flex-shrink-0 border-l border-border/50 bg-panel overflow-y-auto scrollbar-thin">
            // Profile section
            <div class="p-6 text-center border-b border-border/30">
                {if is_group {
                    view! {
                        <div class="w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-secondary/20 to-secondary/5 flex items-center justify-center ring-4 ring-border/30 mb-4">
                            <svg class="w-10 h-10 text-secondary" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="w-20 h-20 mx-auto rounded-2xl bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-4 ring-border/30 mb-4">
                            <span class="text-brand font-bold text-2xl">{initials}</span>
                        </div>
                    }.into_any()
                }}
                <h3 class="text-lg font-bold text-foreground mb-1">{display_name}</h3>
                {if !is_group {
                    let handle = participants.first().map(|p| format!("@{}", p.handle)).unwrap_or_default();
                    view! {
                        <p class="text-sm text-foreground/50">{handle}</p>
                    }.into_any()
                } else {
                    view! {
                        <p class="text-sm text-foreground/50">{participants.len()}" members"</p>
                    }.into_any()
                }}
            </div>

            // Quick actions
            <div class="p-4 border-b border-border/30">
                <div class="grid grid-cols-4 gap-2">
                    <button class="flex flex-col items-center gap-1 p-3 rounded-xl hover:bg-foreground/5 transition-colors">
                        <div class="w-10 h-10 rounded-xl bg-brand/10 flex items-center justify-center">
                            <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                            </svg>
                        </div>
                        <span class="text-[10px] text-foreground/50">"Search"</span>
                    </button>
                    <button class="flex flex-col items-center gap-1 p-3 rounded-xl hover:bg-foreground/5 transition-colors">
                        <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                            <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"/>
                            </svg>
                        </div>
                        <span class="text-[10px] text-foreground/50">"Mute"</span>
                    </button>
                    <button class="flex flex-col items-center gap-1 p-3 rounded-xl hover:bg-foreground/5 transition-colors">
                        <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                            <svg class="w-5 h-5 text-foreground/60" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
                            </svg>
                        </div>
                        <span class="text-[10px] text-foreground/50">"Pin"</span>
                    </button>
                    <button class="flex flex-col items-center gap-1 p-3 rounded-xl hover:bg-foreground/5 transition-colors">
                        <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                            <svg class="w-5 h-5 text-foreground/60" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z"/>
                            </svg>
                        </div>
                        <span class="text-[10px] text-foreground/50">"More"</span>
                    </button>
                </div>
            </div>

            // Encryption info
            <div class="p-4 border-b border-border/30">
                <div class="p-3 rounded-xl bg-success/10 border border-success/20">
                    <div class="flex items-center gap-2 mb-2">
                        <svg class="w-5 h-5 text-success" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                        </svg>
                        <span class="text-sm font-medium text-success">"Encrypted"</span>
                    </div>
                    <p class="text-xs text-foreground/50">"Messages are secured with end-to-end encryption using quantum-resistant cryptography."</p>
                    {if !is_group {
                        let fingerprint = participants.first().and_then(|p| p.public_key_fingerprint.clone()).unwrap_or_else(|| "Not available".to_string());
                        view! {
                            <p class="text-xs font-mono text-foreground/30 mt-2">"Fingerprint: "{fingerprint}</p>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>

            // Members (for groups)
            {if is_group {
                view! {
                    <div class="p-4 border-b border-border/30">
                        <div class="flex items-center justify-between mb-3">
                            <h4 class="text-sm font-semibold text-foreground">"Members"</h4>
                            <span class="text-xs text-foreground/40">{participants.len()}</span>
                        </div>
                        <div class="space-y-2">
                            {participants.into_iter().map(|p| {
                                let initials: String = p.display_name.chars().take(2).collect::<String>().to_uppercase();
                                view! {
                                    <div class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 transition-colors">
                                        <div class="relative">
                                            <div class="w-9 h-9 rounded-lg bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                                <span class="text-brand font-bold text-xs">{initials}</span>
                                            </div>
                                            {if p.is_online {
                                                view! {
                                                    <span class="absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full status-online ring-2 ring-panel"></span>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <div class="flex items-center gap-1">
                                                <span class="text-sm font-medium text-foreground truncate">{p.display_name.clone()}</span>
                                                {if p.is_verified {
                                                    view! {
                                                        <svg class="w-3.5 h-3.5 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                                            <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                        </svg>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                }}
                                            </div>
                                            <p class="text-xs text-foreground/40">{"@"}{p.handle.clone()}</p>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Shared media preview
            <div class="p-4">
                <div class="flex items-center justify-between mb-3">
                    <h4 class="text-sm font-semibold text-foreground">"Shared Media"</h4>
                    <button class="text-xs text-brand hover:underline">"See all"</button>
                </div>
                <div class="grid grid-cols-3 gap-1">
                    {(0..6).map(|_| {
                        view! {
                            <div class="aspect-square rounded-lg bg-foreground/10 flex items-center justify-center">
                                <svg class="w-6 h-6 text-foreground/20" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </aside>
    }
}
