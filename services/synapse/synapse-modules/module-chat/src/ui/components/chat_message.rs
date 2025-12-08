// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{ChatMessage, MessageType, Reaction, UserRole};
use leptos::prelude::*;
use leptos_router::components::A;

/// Props for the ChatMessageCard component
#[component]
pub fn ChatMessageCard(
    #[prop(into)] message: ChatMessage,
    /// Whether this message is from the current user
    #[prop(into, optional)] is_own_message: bool,
    /// Whether to show the author info (false for consecutive messages from same user)
    #[prop(into, optional)] show_author: Option<bool>,
) -> impl IntoView {
    let show_author = show_author.unwrap_or(true);
    let author = message.author.clone();
    let author_for_link = author.handle.clone();
    let author_avatar = author.avatar_url.clone();
    let author_display_name = author.display_name.clone();
    let author_is_online = author.is_online;
    
    // Generate initials for avatar fallback
    let initials = author.display_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    // Handle system messages differently
    if message.message_type == MessageType::System {
        return view! {
            <div class="flex items-center justify-center py-2 px-4">
                <div class="flex items-center gap-2 px-3 py-1 rounded-full bg-foreground/5 text-foreground/40 text-xs">
                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <span>{message.content.clone()}</span>
                    <span class="text-foreground/30 ml-1">{message.timestamp.clone()}</span>
                </div>
            </div>
        }.into_any();
    }

    // Role badge
    let role_badge = match author.role {
        UserRole::Owner => Some(("Owner", "bg-amber-500/20 text-amber-400 border-amber-500/30")),
        UserRole::Admin => Some(("Admin", "bg-rose-500/20 text-rose-400 border-rose-500/30")),
        UserRole::Moderator => Some(("Mod", "bg-violet-500/20 text-violet-400 border-violet-500/30")),
        UserRole::Member => None,
    };

    // Check if content has code block
    let has_code = message.content.contains("```");
    let content_html = if has_code {
        format_code_blocks(&message.content)
    } else {
        message.content.clone()
    };

    // Clone reactions for the view
    let reactions = message.reactions.clone();
    let has_reactions = !reactions.is_empty();

    // Reply preview
    let reply_to = message.reply_to.clone();

    view! {
        <div class=move || format!(
            "group relative flex gap-3 py-1.5 p-4 bg-card border border-border/50 rounded-xl transition-colors hover:bg-foreground/[0.02] {}",
            if is_own_message { "flex-row-reverse" } else { "" }
        )>
            // Avatar column
            <div class="flex-shrink-0 w-10">
                {if show_author {
                    view! {
                        <A href=format!("/profiles/{}", author_for_link) attr:class="block">
                            {if let Some(avatar_url) = author_avatar.clone() {
                                view! {
                                    <img 
                                        src=avatar_url
                                        alt=format!("{}'s avatar", author_display_name)
                                        class="w-10 h-10 rounded-full object-cover ring-2 ring-border/30 hover:ring-brand/50 transition-all"
                                    />
                                }.into_any()
                            } else {
                                view! {
                                    <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center ring-2 ring-border/30 hover:ring-brand/50 transition-all">
                                        <span class="text-brand font-bold text-sm">{initials.clone()}</span>
                                    </div>
                                }.into_any()
                            }}
                            // Online indicator
                            {if author_is_online {
                                view! {
                                    <div class="absolute bottom-0 right-0 w-3 h-3 bg-emerald-500 rounded-full border-2 border-background"></div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </A>
                    }.into_any()
                } else {
                    // Timestamp on hover for consecutive messages
                    view! {
                        <span class="hidden group-hover:flex items-center justify-center h-full text-foreground/30 text-[10px] font-mono">
                            {message.timestamp.clone()}
                        </span>
                    }.into_any()
                }}
            </div>

            // Message content column
            <div class=move || format!(
                "flex-1 min-w-0 {}",
                if is_own_message { "text-right" } else { "" }
            )>
                // Author info row
                {if show_author {
                    let author_display = author.clone();
                    view! {
                        <div class=move || format!(
                            "flex items-center gap-2 mb-1 {}",
                            if is_own_message { "flex-row-reverse" } else { "" }
                        )>
                            // Display name
                            <A href=format!("/profiles/{}", author_display.handle) 
                               attr:class="font-semibold text-foreground hover:text-brand transition-colors">
                                {author_display.display_name.clone()}
                            </A>
                            
                            // Handle
                            <span class="text-brand/60 text-sm font-mono">{"@"}{author_display.handle.clone()}</span>
                            
                            // Role badge
                            {if let Some((label, classes)) = role_badge {
                                view! {
                                    <span class=format!("px-1.5 py-0.5 text-[10px] font-medium rounded border {}", classes)>
                                        {label}
                                    </span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                            
                            // Timestamp
                            <span class="text-foreground/30 text-xs">{message.timestamp.clone()}</span>
                            
                            // Edited indicator
                            {if message.is_edited {
                                view! {
                                    <span class="text-foreground/30 text-xs italic">"(edited)"</span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Reply preview
                {if let Some(reply) = reply_to {
                    let reply_handle = reply.author.handle.clone();
                    let reply_content = reply.content.clone();
                    view! {
                        <div class="flex items-start gap-2 mb-2 pl-3 border-l-2 border-brand/30 text-sm">
                            <div class="flex items-center gap-1.5 text-foreground/50">
                                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"></path>
                                </svg>
                                <A href=format!("/profiles/{}", reply_handle) attr:class="font-medium text-brand/70 hover:text-brand">
                                    {"@"}{reply_handle.clone()}
                                </A>
                            </div>
                            <span class="text-foreground/40 truncate max-w-xs">
                                {truncate_text(&reply_content, 50)}
                            </span>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Message content
                <div class="text-foreground/90 leading-relaxed whitespace-pre-wrap break-words">
                    {if has_code {
                        view! {
                            <div inner_html=content_html></div>
                        }.into_any()
                    } else {
                        view! {
                            <span>{content_html}</span>
                        }.into_any()
                    }}
                </div>

                // Attachments
                {if !message.attachments.is_empty() {
                    view! {
                        <div class="mt-2 flex flex-wrap gap-2">
                            {message.attachments.iter().map(|attachment| {
                                let file_name = attachment.file_name.clone();
                                let file_size = format_file_size(attachment.size_bytes);
                                view! {
                                    <div class="inline-flex items-center gap-2 px-3 py-2 rounded-lg bg-panel border border-border/50 hover:border-brand/30 transition-colors cursor-pointer group/att">
                                        <div class="w-8 h-8 rounded bg-brand/10 flex items-center justify-center">
                                            <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                            </svg>
                                        </div>
                                        <div class="flex flex-col">
                                            <span class="text-sm font-medium text-foreground/90 group-hover/att:text-brand transition-colors">{file_name}</span>
                                            <span class="text-xs text-foreground/40">{file_size}</span>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}

                // Reactions
                {if has_reactions {
                    view! {
                        <div class="mt-2 flex flex-wrap gap-1.5">
                            {reactions.iter().map(|reaction| {
                                let reaction = reaction.clone();
                                view! {
                                    <ReactionBadge reaction=reaction />
                                }
                            }).collect_view()}
                            // Add reaction button
                            <button class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-foreground/5 hover:bg-foreground/10 text-foreground/40 hover:text-foreground/60 text-sm transition-colors border border-transparent hover:border-border/50">
                                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Hover actions
            <div class="absolute top-0 right-4 opacity-0 group-hover:opacity-100 transition-opacity">
                <div class="flex items-center gap-0.5 -mt-3 px-1 py-0.5 rounded-lg bg-panel border border-border/50 shadow-lg">
                    <button class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" title="Add reaction">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                    </button>
                    <button class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" title="Reply">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"></path>
                        </svg>
                    </button>
                    <button class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" title="Pin message">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"></path>
                        </svg>
                    </button>
                    <div class="w-px h-4 bg-border/50 mx-0.5"></div>
                    <button class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" title="More options">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    }.into_any()
}

/// Reaction badge component
#[component]
fn ReactionBadge(#[prop(into)] reaction: Reaction) -> impl IntoView {
    view! {
        <button class=move || format!(
            "inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-sm transition-all {}",
            if reaction.reacted_by_me {
                "bg-brand/20 border border-brand/40 text-brand"
            } else {
                "bg-foreground/5 border border-transparent hover:border-border/50 text-foreground/70 hover:text-foreground"
            }
        )>
            <span>{reaction.emoji.clone()}</span>
            <span class="text-xs font-medium">{reaction.count}</span>
        </button>
    }
}

// Helper functions

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    }
}

fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn format_code_blocks(content: &str) -> String {
    // Simple code block formatting - wrap ```...``` in styled divs
    let mut result = String::new();
    let mut in_code = false;
    let mut code_lang = String::new();
    let mut code_content = String::new();

    for line in content.lines() {
        if line.starts_with("```") && !in_code {
            in_code = true;
            code_lang = line.trim_start_matches("```").to_string();
            code_content.clear();
        } else if line.starts_with("```") && in_code {
            in_code = false;
            let lang_label = if code_lang.is_empty() { "code" } else { &code_lang };
            result.push_str(&format!(
                r#"<div class="my-2 rounded-lg overflow-hidden border border-border/50 bg-background/50">
                    <div class="flex items-center justify-between px-3 py-1.5 bg-foreground/5 border-b border-border/30">
                        <span class="text-xs font-mono text-foreground/50">{}</span>
                        <button class="text-xs text-foreground/40 hover:text-foreground transition-colors">Copy</button>
                    </div>
                    <pre class="p-3 text-sm font-mono text-foreground/90 overflow-x-auto"><code>{}</code></pre>
                </div>"#,
                lang_label,
                html_escape(&code_content)
            ));
        } else if in_code {
            if !code_content.is_empty() {
                code_content.push('\n');
            }
            code_content.push_str(line);
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result.trim_end().to_string()
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

