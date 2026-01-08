// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{ChatMessage, ChatUser};
use leptos::prelude::*;

/// Chat input bar for composing and sending messages - compact and responsive
#[component]
pub fn ChatInput(
    /// The current room/channel name
    #[prop(into)]
    room_name: Signal<String>,
    /// Optional reply context - message being replied to
    #[prop(into, optional)]
    reply_to: Option<Signal<Option<ChatMessage>>>,
    /// Current user (for avatar)
    #[prop(into, optional)]
    current_user: Option<ChatUser>,
) -> impl IntoView {
    let (content, set_content) = signal(String::new());
    let (is_sending, set_is_sending) = signal(false);
    let (show_emoji_picker, set_show_emoji_picker) = signal(false);
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    // User info for avatar
    let user = current_user.unwrap_or_else(ChatUser::mock_me);
    let initials = user.display_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    // Auto-resize textarea
    let resize_textarea = move || {
        if let Some(textarea) = textarea_ref.get() {
            let _ = textarea.style(("height", "auto"));
            let scroll_height = textarea.scroll_height();
            let max_height = 100; // Smaller max height for compact mode
            let new_height = scroll_height.min(max_height);
            let _ = textarea.style(("height", format!("{}px", new_height)));
        }
    };

    // Send message handler
    let on_send = move |_| {
        let msg = content.get();
        if msg.trim().is_empty() {
            return;
        }

        set_is_sending.set(true);
        leptos::logging::log!("Sending to #{}: {}", room_name.get(), msg);
        set_content.set(String::new());
        set_is_sending.set(false);
        
        if let Some(textarea) = textarea_ref.get() {
            let _ = textarea.style(("height", "auto"));
        }
    };

    let quick_emojis = vec!["👍", "❤️", "😂", "🔥", "👀", "🎉"];

    view! {
        <div class="flex-shrink-0 border-t border-border/50 bg-panel/30">
            // Reply preview bar
            {move || {
                if let Some(reply_signal) = reply_to {
                    if let Some(reply_msg) = reply_signal.get() {
                        let handle = reply_msg.author.handle.clone();
                        let preview = truncate(&reply_msg.content, 30);
                        view! {
                            <div class="flex items-center gap-2 px-2 py-1.5 border-b border-border/30 bg-brand/5 text-xs">
                                <svg class="w-3 h-3 text-brand flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"></path>
                                </svg>
                                <span class="text-foreground/50">"Re: "</span>
                                <span class="font-medium text-brand">{"@"}{handle}</span>
                                <span class="text-foreground/40 truncate">{preview}</span>
                                <button class="ml-auto p-0.5 rounded hover:bg-foreground/10 text-foreground/40">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            <div class="p-2">
                <div class="flex items-end gap-2">
                    // Avatar (hidden on very small screens)
                    <div class="hidden sm:block flex-shrink-0 mb-0.5">
                        {if let Some(avatar) = user.avatar_url.clone() {
                            view! {
                                <img src=avatar alt="You" class="w-7 h-7 rounded-full object-cover ring-1 ring-border/30"/>
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-7 h-7 rounded-full bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center ring-1 ring-border/30">
                                    <span class="text-brand font-bold text-[10px]">{initials.clone()}</span>
                                </div>
                            }.into_any()
                        }}
                    </div>

                    // Input container
                    <div class="flex-1 relative min-w-0">
                        <div class="relative bg-background border border-border/50 rounded-lg focus-within:border-brand/50 transition-all">
                            <textarea
                                node_ref=textarea_ref
                                placeholder=move || format!("Message #{}", room_name.get())
                                class="w-full px-3 py-2 pr-20 bg-transparent text-sm text-foreground placeholder-foreground/30 focus:outline-none resize-none overflow-y-auto leading-relaxed scrollbar-thin"
                                style="min-height: 36px; max-height: 100px;"
                                prop:value=move || content.get()
                                prop:disabled=move || is_sending.get()
                                on:input=move |ev| {
                                    set_content.set(event_target_value(&ev));
                                    resize_textarea();
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" && !ev.shift_key() {
                                        ev.prevent_default();
                                        on_send(());
                                    }
                                }
                            ></textarea>

                            // Action buttons inside input
                            <div class="absolute bottom-1.5 right-1.5 flex items-center gap-0.5">
                                // Attachment
                                <button 
                                    class="p-1.5 rounded text-foreground/30 hover:text-foreground/60 hover:bg-foreground/5 transition-colors"
                                    title="Attach"
                                >
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4"></path>
                                    </svg>
                                </button>

                                // Emoji
                                <div class="relative">
                                    <button 
                                        class=move || format!(
                                            "p-1.5 rounded transition-colors {}",
                                            if show_emoji_picker.get() { "bg-brand/20 text-brand" } else { "text-foreground/30 hover:text-foreground/60 hover:bg-foreground/5" }
                                        )
                                        on:click=move |_| set_show_emoji_picker.update(|v| *v = !*v)
                                        title="Emoji"
                                    >
                                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                    </button>

                                    {move || {
                                        if show_emoji_picker.get() {
                                            view! {
                                                <div class="absolute bottom-full right-0 mb-1 bg-panel border border-border/50 rounded-lg shadow-xl p-1.5 z-50">
                                                    <div class="flex gap-0.5">
                                                        {quick_emojis.iter().map(|emoji| {
                                                            let emoji = emoji.to_string();
                                                            let emoji_for_click = emoji.clone();
                                                            view! {
                                                                <button 
                                                                    class="w-7 h-7 flex items-center justify-center rounded hover:bg-foreground/10 text-base transition-colors"
                                                                    on:click=move |_| {
                                                                        set_content.update(|c| c.push_str(&emoji_for_click));
                                                                        set_show_emoji_picker.set(false);
                                                                    }
                                                                >
                                                                    {emoji}
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </div>

                    // Send button
                    <button 
                        class=move || format!(
                            "flex-shrink-0 mb-0.5 p-2 rounded-lg transition-all active:scale-95 {}",
                            if content.get().trim().is_empty() || is_sending.get() {
                                "bg-brand/30 text-white/50 cursor-not-allowed"
                            } else {
                                "bg-brand hover:bg-brand/90 text-white"
                            }
                        )
                        prop:disabled=move || content.get().trim().is_empty() || is_sending.get()
                        on:click=move |_| on_send(())
                        title="Send (Enter)"
                    >
                        {move || if is_sending.get() {
                            view! {
                                <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8"></path>
                                </svg>
                            }.into_any()
                        }}
                    </button>
                </div>

                // Compact hints (hidden on small screens)
                <div class="hidden sm:flex items-center justify-end mt-1 px-9 text-[10px] text-foreground/25">
                    <span>
                        <kbd class="px-1 py-0.5 bg-foreground/5 rounded font-mono">"Enter"</kbd>
                        " send · "
                        <kbd class="px-1 py-0.5 bg-foreground/5 rounded font-mono">"Shift+Enter"</kbd>
                        " new line"
                    </span>
                </div>
            </div>
        </div>
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}
