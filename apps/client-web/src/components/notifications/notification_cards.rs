// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::*;
use leptos::prelude::*;
use leptos_router::components::A;

/// Card for network broadcast alerts
#[component]
pub fn BroadcastAlertCard(alert: BroadcastAlert) -> impl IntoView {
    let priority_class = alert.priority.color_class();
    let indicator_class = alert.priority.indicator_class();
    let title = alert.title.clone();
    let message = alert.message.clone();
    let source = alert.source.clone();
    let timestamp = alert.timestamp.clone();
    let action_url = alert.action_url.clone();
    let action_label = alert.action_label.clone();

    view! {
        <article class=format!(
            "relative p-4 rounded-xl border transition-all hover:border-border {}",
            if alert.is_read { "bg-card/50 border-border/30" } else { "bg-card border-border/50" }
        )>
            // Priority indicator
            <div class=format!("absolute left-0 top-0 bottom-0 w-1 rounded-l-xl {}", indicator_class)></div>

            <div class="pl-3">
                // Header with source and timestamp
                <div class="flex items-center justify-between mb-2">
                    <div class="flex items-center gap-2">
                        <div class="w-6 h-6 rounded-lg bg-brand/15 flex items-center justify-center">
                            <svg class="w-3.5 h-3.5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                            </svg>
                        </div>
                        <span class="text-xs font-medium text-foreground/60">{source}</span>
                        <span class=format!("px-2 py-0.5 rounded-full text-[10px] font-semibold border {}", priority_class)>
                            {alert.priority.label()}
                        </span>
                    </div>
                    <time class="text-xs text-foreground/40">{timestamp}</time>
                </div>

                // Title
                <h3 class="font-semibold text-foreground mb-1">{title}</h3>

                // Message
                <p class="text-sm text-foreground/60 leading-relaxed mb-3">{message}</p>

                // Actions
                <div class="flex items-center justify-between">
                    {if let (Some(url), Some(label)) = (action_url, action_label) {
                        view! {
                            <A href=url attr:class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-sm font-medium text-brand hover:bg-brand/10 transition-colors">
                                {label}
                                <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                                </svg>
                            </A>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}

                    {if !alert.is_read {
                        view! {
                            <button class="text-xs text-foreground/40 hover:text-foreground/60 transition-colors">
                                "Mark as read"
                            </button>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>
        </article>
    }
}

/// Card for content notifications
#[component]
pub fn ContentNotificationCard(notification: ContentNotification) -> impl IntoView {
    let initials: String = notification.actor_handle
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let icon_color = notification.notification_type.icon_color();
    let icon_svg = notification.notification_type.icon_svg();
    let action_text = notification.notification_type.label();
    
    let handle = notification.actor_handle.clone();
    let display_name = notification.actor_display_name.clone();
    let timestamp = notification.timestamp.clone();
    let post_preview = notification.post_preview.clone();
    let comment_preview = notification.comment_preview.clone();
    let synapse_name = notification.synapse_name.clone();

    // For reaction type, get the emoji
    let reaction_emoji = if let ContentNotificationType::Reaction { emoji } = &notification.notification_type {
        Some(emoji.clone())
    } else {
        None
    };

    view! {
        <article class=format!(
            "group flex gap-3 p-4 rounded-xl transition-all hover:bg-foreground/[0.02] {}",
            if notification.is_read { "" } else { "bg-brand/[0.03]" }
        )>
            // Avatar with notification type indicator
            <div class="relative flex-shrink-0">
                <div class="w-11 h-11 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30">
                    <span class="text-brand font-bold text-sm">{initials}</span>
                </div>
                // Type indicator
                <div class=format!("absolute -bottom-1 -right-1 w-6 h-6 rounded-full flex items-center justify-center ring-2 ring-panel {}", icon_color)>
                    {if let Some(emoji) = reaction_emoji {
                        view! { <span class="text-sm">{emoji}</span> }.into_any()
                    } else {
                        view! {
                            <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor" inner_html=icon_svg></svg>
                        }.into_any()
                    }}
                </div>
            </div>

            // Content
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                    <div>
                        <p class="text-sm text-foreground leading-snug">
                            <A href=format!("/profiles/{}", handle) attr:class="font-semibold text-foreground hover:text-brand transition-colors">
                                {display_name}
                            </A>
                            {if notification.actor_verified {
                                view! {
                                    <svg class="inline-block w-3.5 h-3.5 ml-0.5 text-brand" viewBox="0 0 24 24" fill="currentColor">
                                        <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                    </svg>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                            " "
                            <span class="text-foreground/60">{action_text}</span>
                        </p>

                        // Context: synapse name
                        {if let Some(synapse) = synapse_name {
                            view! {
                                <p class="text-xs text-foreground/40 mt-0.5 flex items-center gap-1">
                                    <span class="text-violet-400 font-mono">"#"</span>
                                    {synapse}
                                </p>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    <time class="flex-shrink-0 text-xs text-foreground/40">{timestamp}</time>
                </div>

                // Preview content
                {if let Some(preview) = post_preview {
                    view! {
                        <div class="mt-2 p-2.5 rounded-lg bg-foreground/[0.03] border border-border/30">
                            <p class="text-sm text-foreground/50 line-clamp-2">{preview}</p>
                        </div>
                    }.into_any()
                } else if let Some(comment) = comment_preview {
                    view! {
                        <div class="mt-2 p-2.5 rounded-lg bg-foreground/[0.03] border border-border/30">
                            <p class="text-sm text-foreground/60 line-clamp-2">"\""{ comment }"\""</p>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Unread indicator
            {if !notification.is_read {
                view! {
                    <div class="flex-shrink-0 w-2.5 h-2.5 rounded-full bg-brand mt-1.5"></div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </article>
    }
}

/// Card for connection notifications
#[component]
pub fn ConnectionNotificationCard(notification: ConnectionNotification) -> impl IntoView {
    let icon_color = notification.notification_type.icon_color();
    let icon_svg = notification.notification_type.icon_svg();
    let action_text = notification.notification_type.label();
    let has_actions = notification.notification_type.has_actions();
    
    let handle = notification.actor_handle.clone();
    let display_name = notification.actor_display_name.clone();
    let timestamp = notification.timestamp.clone();
    let mutual_count = notification.mutual_count;

    let initials: String = handle.clone()
        .unwrap_or_else(|| "SY".to_string())
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();

    // Check if this is a system-level notification (like credential awarded)
    let is_system = matches!(notification.notification_type, ConnectionNotificationType::CredentialAwarded { .. });

    view! {
        <article class=format!(
            "group flex gap-3 p-4 rounded-xl transition-all hover:bg-foreground/[0.02] {}",
            if notification.is_read { "" } else { "bg-brand/[0.03]" }
        )>
            // Avatar or system icon
            <div class="relative flex-shrink-0">
                {if is_system {
                    view! {
                        <div class=format!("w-11 h-11 rounded-full flex items-center justify-center ring-2 ring-border/30 {}", icon_color)>
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" inner_html=icon_svg></svg>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="w-11 h-11 rounded-full bg-gradient-to-br from-sky-500/20 to-sky-500/5 flex items-center justify-center ring-2 ring-border/30">
                            <span class="text-sky-400 font-bold text-sm">{initials.clone()}</span>
                        </div>
                        // Type indicator
                        <div class=format!("absolute -bottom-1 -right-1 w-6 h-6 rounded-full flex items-center justify-center ring-2 ring-panel {}", icon_color)>
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" inner_html=icon_svg></svg>
                        </div>
                    }.into_any()
                }}
            </div>

            // Content
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                    <div>
                        <p class="text-sm text-foreground leading-snug">
                            {if let Some(name) = display_name.clone() {
                                let h = handle.clone().unwrap_or_default();
                                view! {
                                    <A href=format!("/profiles/{}", h) attr:class="font-semibold text-foreground hover:text-brand transition-colors">
                                        {name}
                                    </A>
                                    {if notification.actor_verified {
                                        view! {
                                            <svg class="inline-block w-3.5 h-3.5 ml-0.5 text-brand" viewBox="0 0 24 24" fill="currentColor">
                                                <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }}
                                    " "
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                            <span class="text-foreground/60">{action_text}</span>
                        </p>

                        // Mutual connections count
                        {if let Some(count) = mutual_count {
                            view! {
                                <p class="text-xs text-foreground/40 mt-0.5 flex items-center gap-1">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>
                                    </svg>
                                    {count}" mutual connections"
                                </p>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>

                    <time class="flex-shrink-0 text-xs text-foreground/40">{timestamp}</time>
                </div>

                // Action buttons for follow requests and invites
                {if has_actions {
                    view! {
                        <div class="flex items-center gap-2 mt-3">
                            <button class="px-3 py-1.5 rounded-lg text-sm font-medium bg-brand text-white hover:bg-brand/90 transition-colors">
                                "Accept"
                            </button>
                            <button class="px-3 py-1.5 rounded-lg text-sm font-medium bg-foreground/5 text-foreground/70 hover:bg-foreground/10 transition-colors">
                                "Decline"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Unread indicator
            {if !notification.is_read {
                view! {
                    <div class="flex-shrink-0 w-2.5 h-2.5 rounded-full bg-brand mt-1.5"></div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </article>
    }
}

/// Card for system notifications
#[component]
pub fn SystemNotificationCard(notification: SystemNotification) -> impl IntoView {
    let icon_color = notification.notification_type.icon_color();
    let icon_svg = notification.notification_type.icon_svg();
    let message = notification.notification_type.label();
    let timestamp = notification.timestamp.clone();
    let action_url = notification.action_url.clone();
    let action_label = notification.action_label.clone();

    view! {
        <article class=format!(
            "group flex gap-3 p-4 rounded-xl transition-all hover:bg-foreground/[0.02] {}",
            if notification.is_read { "" } else { "bg-brand/[0.03]" }
        )>
            // Icon
            <div class=format!("w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0 {}", icon_color)>
                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" inner_html=icon_svg></svg>
            </div>

            // Content
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                    <p class="text-sm text-foreground">{message}</p>
                    <time class="flex-shrink-0 text-xs text-foreground/40">{timestamp}</time>
                </div>

                // Action button
                {if let (Some(url), Some(label)) = (action_url, action_label) {
                    view! {
                        <A href=url attr:class="inline-flex items-center gap-1 mt-2 text-sm font-medium text-brand hover:text-brand/80 transition-colors">
                            {label}
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                            </svg>
                        </A>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Unread indicator
            {if !notification.is_read {
                view! {
                    <div class="flex-shrink-0 w-2.5 h-2.5 rounded-full bg-brand mt-1.5"></div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </article>
    }
}
