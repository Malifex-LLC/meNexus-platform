// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{Member, MemberRole, OnlineStatus};
use leptos::prelude::*;
use leptos_router::components::A;

/// Individual member card component
#[component]
pub fn MemberCard(
    #[prop(into)] member: Member,
    /// Whether to show in compact mode (for sidebar lists)
    #[prop(into, optional)] compact: Option<bool>,
) -> impl IntoView {
    let compact = compact.unwrap_or(false);
    let initials = member.get_initials();
    let handle = member.handle.clone();
    let display_name = member.display_name.clone();
    let display_name_for_alt = display_name.clone();
    let display_name_for_link = display_name.clone();
    let status = member.status.clone();
    let role = member.role.clone();
    let status_message = member.status_message.clone();
    let is_streaming = member.is_streaming;
    let is_verified = member.is_verified;
    let last_seen = member.last_seen.clone();
    let avatar_url = member.avatar_url.clone();
    let avatar_url_expanded = avatar_url.clone();

    if compact {
        // Compact view for sidebars
        view! {
            <A
                href=format!("/profiles/{}", handle)
                attr:class="group flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-foreground/5 transition-all cursor-pointer"
            >
                // Avatar with status indicator
                <div class="relative flex-shrink-0">
                    {if let Some(url) = avatar_url.clone() {
                        view! {
                            <img
                                src=url
                                alt=format!("{}'s avatar", display_name)
                                class="w-8 h-8 rounded-full object-cover ring-1 ring-border/30"
                            />
                        }.into_any()
                    } else {
                        view! {
                            <div class="w-8 h-8 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-1 ring-border/30 flex items-center justify-center">
                                <span class="text-brand font-semibold text-xs">{initials.clone()}</span>
                            </div>
                        }.into_any()
                    }}
                    // Status indicator
                    <div class=format!(
                        "absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full ring-2 ring-panel {}",
                        status.indicator_class()
                    )></div>
                </div>

                // Name and status
                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-1.5">
                        <span class="text-sm font-medium text-foreground/90 truncate group-hover:text-foreground transition-colors">
                            {display_name.clone()}
                        </span>
                        {if is_verified {
                            view! {
                                <svg class="w-3.5 h-3.5 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        {if is_streaming {
                            view! {
                                <span class="flex items-center gap-1 px-1.5 py-0.5 rounded bg-rose-500/20 text-rose-400 text-[10px] font-medium">
                                    <span class="w-1.5 h-1.5 rounded-full bg-rose-400 animate-pulse"></span>
                                    "LIVE"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    {if let Some(msg) = status_message.clone() {
                        view! {
                            <p class="text-xs text-foreground/40 truncate">{msg}</p>
                        }.into_any()
                    } else if !status.is_online() {
                        if let Some(seen) = last_seen.clone() {
                            view! {
                                <p class="text-xs text-foreground/30 truncate">{seen}</p>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Role icon (only for special roles)
                {if role != MemberRole::Member {
                    view! {
                        <div 
                            class=format!("flex-shrink-0 w-5 h-5 rounded flex items-center justify-center {}", role.badge_classes())
                            title=role.label()
                            inner_html=role.icon_svg()
                        ></div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </A>
        }.into_any()
    } else {
        // Expanded view for detailed display
        view! {
            <article class="group relative flex gap-3 p-3 bg-card/50 border border-border/30 rounded-xl transition-all duration-200 hover:bg-foreground/[0.02] hover:border-border/50">
                // Avatar with status
                <div class="relative flex-shrink-0">
                    <A href=format!("/profiles/{}", handle.clone())>
                        {if let Some(url) = avatar_url_expanded {
                            view! {
                                <img
                                    src=url
                                    alt=format!("{}'s avatar", display_name_for_alt)
                                    class="w-11 h-11 rounded-full object-cover ring-2 ring-border/30 hover:ring-brand/50 transition-all"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-11 h-11 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-2 ring-border/30 hover:ring-brand/50 transition-all flex items-center justify-center">
                                    <span class="text-brand font-bold text-sm">{initials}</span>
                                </div>
                            }.into_any()
                        }}
                    </A>
                    // Status indicator
                    <div class=format!(
                        "absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 rounded-full ring-2 ring-card {}",
                        status.indicator_class()
                    )></div>
                    // Streaming indicator
                    {if is_streaming {
                        view! {
                            <div class="absolute -top-1 -right-1 w-4 h-4 rounded-full bg-rose-500 ring-2 ring-card flex items-center justify-center">
                                <div class="w-2 h-2 rounded-full bg-white animate-pulse"></div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Member info
                <div class="flex-1 min-w-0">
                    // Name row
                    <div class="flex items-center gap-2 flex-wrap">
                        <A 
                            href=format!("/profiles/{}", handle.clone())
                            attr:class="font-semibold text-foreground hover:text-brand transition-colors"
                        >
                            {display_name_for_link}
                        </A>
                        <span class="text-brand/60 text-sm font-mono">{"@"}{handle.clone()}</span>
                        
                        // Verified badge
                        {if is_verified {
                            view! {
                                <svg class="w-4 h-4 text-brand" viewBox="0 0 24 24" fill="currentColor" title="Verified">
                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        
                        // Role badge
                        <span class=format!(
                            "px-1.5 py-0.5 text-[10px] font-medium rounded border flex items-center gap-1 {}",
                            role.badge_classes()
                        )>
                            <span inner_html=role.icon_svg()></span>
                            {role.label()}
                        </span>
                    </div>

                    // Status message or last seen
                    <div class="mt-1 flex items-center gap-2 text-xs">
                        <span class=format!(
                            "flex items-center gap-1.5 {}",
                            if status.is_online() { "text-foreground/50" } else { "text-foreground/30" }
                        )>
                            <span class=format!(
                                "w-1.5 h-1.5 rounded-full {}",
                                status.indicator_class()
                            )></span>
                            {status.label()}
                        </span>
                        {if let Some(msg) = status_message {
                            view! {
                                <span class="text-foreground/40">"·"</span>
                                <span class="text-foreground/50 truncate max-w-[200px]">{msg}</span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                        {if !status.is_online() {
                            if let Some(seen) = last_seen {
                                view! {
                                    <span class="text-foreground/30">"·"</span>
                                    <span class="text-foreground/30">"Last seen "{seen}</span>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>

                // Hover actions
                <div class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                    <div class="flex items-center gap-0.5 px-1 py-0.5 rounded-lg bg-panel border border-border/50 shadow-lg">
                        <button 
                            class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" 
                            title="Send message"
                        >
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                            </svg>
                        </button>
                        <button 
                            class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" 
                            title="View profile"
                        >
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"/>
                            </svg>
                        </button>
                        <div class="w-px h-4 bg-border/50 mx-0.5"></div>
                        <button 
                            class="p-1.5 rounded hover:bg-foreground/10 text-foreground/50 hover:text-foreground transition-colors" 
                            title="More options"
                        >
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </article>
        }.into_any()
    }
}
