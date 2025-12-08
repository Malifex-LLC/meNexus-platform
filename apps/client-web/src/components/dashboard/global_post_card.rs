// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::GlobalPost;
use leptos::prelude::*;
use leptos_router::components::A;

/// Format large numbers
fn format_count(count: u32) -> String {
    if count >= 1000 {
        format!("{:.1}K", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}

/// Post card for the global feed - shows source Synapse
#[component]
pub fn GlobalPostCard(#[prop(into)] post: GlobalPost) -> impl IntoView {
    let (is_liked, set_is_liked) = signal(post.is_liked);
    let (like_count, set_like_count) = signal(post.likes);
    let (is_bookmarked, set_is_bookmarked) = signal(post.is_bookmarked);

    let initials: String = post
        .author_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    let synapse_initial = post.synapse_name.chars().next().unwrap_or('S').to_uppercase().to_string();

    // Check if content has code block
    let has_code = post.content.contains("```");

    view! {
        <article class="group relative bg-card border border-border/50 rounded-xl overflow-hidden transition-all duration-200 hover:border-border hover:shadow-lg hover:shadow-black/5">
            // Source indicator - shows which Synapse this is from
            <div class="px-4 py-2 bg-foreground/[0.02] border-b border-border/30 flex items-center gap-2">
                <A 
                    href=format!("/synapses/{}", post.synapse_id)
                    attr:class="flex items-center gap-2 hover:text-brand transition-colors"
                >
                    <div class="w-5 h-5 rounded bg-violet-500/20 flex items-center justify-center">
                        <span class="text-violet-400 font-bold text-[9px]">{synapse_initial}</span>
                    </div>
                    <span class="text-xs font-medium text-foreground/60">{post.synapse_name.clone()}</span>
                </A>
                <span class="text-foreground/20">"/"</span>
                <span class="text-xs text-brand/70 font-mono">"#"{post.channel_name.clone()}</span>
                <div class="flex-1"></div>
                <time class="text-xs text-foreground/40">{post.timestamp.clone()}</time>
            </div>

            <div class="p-4">
                // Author row
                <div class="flex items-start gap-3">
                    // Avatar
                    <A href=format!("/profiles/{}", post.author_handle) attr:class="relative flex-shrink-0">
                        {if let Some(url) = post.author_avatar.clone() {
                            view! {
                                <img
                                    src=url
                                    alt=""
                                    class="w-11 h-11 rounded-full object-cover ring-2 ring-border/30 group-hover:ring-brand/30 transition-all"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-11 h-11 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-2 ring-border/30 group-hover:ring-brand/30 flex items-center justify-center transition-all">
                                    <span class="text-brand font-bold text-sm">{initials.clone()}</span>
                                </div>
                            }.into_any()
                        }}
                        // Online indicator
                        <div class="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-emerald-500 rounded-full border-2 border-card"></div>
                    </A>

                    // Author info
                    <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2 flex-wrap">
                            <A 
                                href=format!("/profiles/{}", post.author_handle)
                                attr:class="font-semibold text-foreground hover:text-brand transition-colors"
                            >
                                {post.author_name.clone()}
                            </A>
                            
                            {if post.author_verified {
                                view! {
                                    <svg class="w-4 h-4 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                        <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                    </svg>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}

                            <span class="text-brand/60 text-sm font-mono">{"@"}{post.author_handle.clone()}</span>

                            // Reputation badge
                            <span class="px-1.5 py-0.5 rounded text-[10px] font-medium bg-foreground/5 text-foreground/40 font-mono">
                                {format_count(post.author_reputation)}" rep"
                            </span>
                        </div>
                    </div>
                </div>

                // Content
                <div class="mt-3 pl-14">
                    {if has_code {
                        view! {
                            <div class="text-foreground/90 leading-relaxed whitespace-pre-wrap break-words">
                                {render_content_with_code(&post.content)}
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <p class="text-foreground/90 leading-relaxed whitespace-pre-wrap break-words">
                                {post.content.clone()}
                            </p>
                        }.into_any()
                    }}

                    // Media indicator
                    {if post.has_media {
                        view! {
                            <div class="mt-3 p-4 bg-foreground/5 rounded-xl border border-border/30 flex items-center gap-3 text-foreground/40">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                                <span class="text-sm">"Media attachment"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Actions bar
                <div class="mt-4 pl-14 flex items-center gap-1">
                    // Like
                    <button
                        class="group/btn flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-all hover:bg-brand/10"
                        on:click=move |_| {
                            if is_liked.get() {
                                set_like_count.update(|c| *c = c.saturating_sub(1));
                            } else {
                                set_like_count.update(|c| *c += 1);
                            }
                            set_is_liked.update(|v| *v = !*v);
                        }
                    >
                        <svg
                            class=move || format!(
                                "w-4 h-4 transition-all {}",
                                if is_liked.get() { "fill-brand text-brand" } else { "fill-none text-foreground/40 group-hover/btn:text-brand" }
                            )
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            stroke-width="2"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                        </svg>
                        <span class=move || format!(
                            "text-sm font-medium {}",
                            if is_liked.get() { "text-brand" } else { "text-foreground/40 group-hover/btn:text-brand" }
                        )>
                            {move || format_count(like_count.get())}
                        </span>
                    </button>

                    // Comment
                    <button class="group/btn flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-all hover:bg-sky-500/10">
                        <svg class="w-4 h-4 text-foreground/40 group-hover/btn:text-sky-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                        </svg>
                        <span class="text-sm font-medium text-foreground/40 group-hover/btn:text-sky-400">{format_count(post.comments)}</span>
                    </button>

                    // Share
                    <button class="group/btn flex items-center gap-1.5 px-3 py-1.5 rounded-lg transition-all hover:bg-emerald-500/10">
                        <svg class="w-4 h-4 text-foreground/40 group-hover/btn:text-emerald-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/>
                        </svg>
                        <span class="text-sm font-medium text-foreground/40 group-hover/btn:text-emerald-400">{format_count(post.shares)}</span>
                    </button>

                    <div class="flex-1"></div>

                    // Bookmark
                    <button
                        class=move || format!(
                            "p-1.5 rounded-lg transition-all {}",
                            if is_bookmarked.get() { "text-amber-400 bg-amber-500/10" } else { "text-foreground/30 hover:text-amber-400 hover:bg-amber-500/10" }
                        )
                        on:click=move |_| set_is_bookmarked.update(|v| *v = !*v)
                    >
                        <svg 
                            class=move || format!("w-4 h-4 {}", if is_bookmarked.get() { "fill-current" } else { "fill-none" })
                            viewBox="0 0 24 24" 
                            stroke="currentColor" 
                            stroke-width="2"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z"/>
                        </svg>
                    </button>

                    // More options
                    <button class="p-1.5 rounded-lg text-foreground/30 hover:text-foreground/60 hover:bg-foreground/5 transition-all">
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"/>
                        </svg>
                    </button>
                </div>
            </div>
        </article>
    }
}

/// Simple code block rendering (basic implementation)
fn render_content_with_code(content: &str) -> impl IntoView {
    let parts: Vec<&str> = content.split("```").collect();
    
    view! {
        <div>
            {parts.iter().enumerate().map(|(i, part)| {
                if i % 2 == 1 {
                    // Code block
                    let lines: Vec<&str> = part.lines().collect();
                    let (lang, code) = if !lines.is_empty() && !lines[0].contains(' ') && lines[0].len() < 20 {
                        (lines[0], lines[1..].join("\n"))
                    } else {
                        ("code", part.to_string())
                    };
                    
                    view! {
                        <div class="my-3 rounded-lg overflow-hidden border border-border/50 bg-background/50">
                            <div class="flex items-center justify-between px-3 py-1.5 bg-foreground/5 border-b border-border/30">
                                <span class="text-xs font-mono text-foreground/50">{lang}</span>
                                <button class="text-xs text-foreground/40 hover:text-foreground transition-colors">"Copy"</button>
                            </div>
                            <pre class="p-3 text-sm font-mono text-foreground/90 overflow-x-auto">
                                <code>{code}</code>
                            </pre>
                        </div>
                    }.into_any()
                } else {
                    // Regular text
                    view! {
                        <span>{part.to_string()}</span>
                    }.into_any()
                }
            }).collect_view()}
        </div>
    }
}
