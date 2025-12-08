// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_router::components::A;

/// Types of notifications
#[derive(Clone)]
pub enum NotificationType {
    Like { post_preview: String },
    Comment { post_preview: String, comment_preview: String },
    Follow,
    Mention { post_preview: String },
    Reply { comment_preview: String },
    SynapseInvite { synapse_name: String },
}

/// Represents a notification
#[derive(Clone)]
pub struct Notification {
    pub id: String,
    pub actor_handle: String,
    pub actor_avatar: Option<String>,
    pub notification_type: NotificationType,
    pub timestamp: String,
    pub is_read: bool,
}

fn get_mock_notifications() -> Vec<Notification> {
    vec![
        Notification {
            id: "notif-001".to_string(),
            actor_handle: "morpheus".to_string(),
            actor_avatar: None,
            notification_type: NotificationType::Like {
                post_preview: "Just discovered something incredible...".to_string(),
            },
            timestamp: "2m".to_string(),
            is_read: false,
        },
        Notification {
            id: "notif-002".to_string(),
            actor_handle: "trinity".to_string(),
            actor_avatar: None,
            notification_type: NotificationType::Comment {
                post_preview: "The Matrix has you...".to_string(),
                comment_preview: "Follow the white rabbit.".to_string(),
            },
            timestamp: "5m".to_string(),
            is_read: false,
        },
        Notification {
            id: "notif-003".to_string(),
            actor_handle: "oracle".to_string(),
            actor_avatar: None,
            notification_type: NotificationType::Follow,
            timestamp: "12m".to_string(),
            is_read: false,
        },
        Notification {
            id: "notif-004".to_string(),
            actor_handle: "niobe".to_string(),
            actor_avatar: None,
            notification_type: NotificationType::Mention {
                post_preview: "Hey @neo, check this out...".to_string(),
            },
            timestamp: "1h".to_string(),
            is_read: true,
        },
        Notification {
            id: "notif-005".to_string(),
            actor_handle: "tank".to_string(),
            actor_avatar: None,
            notification_type: NotificationType::SynapseInvite {
                synapse_name: "Zion Command".to_string(),
            },
            timestamp: "2h".to_string(),
            is_read: true,
        },
    ]
}

#[component]
pub fn NotificationsDropdown() -> impl IntoView {
    let notifications = get_mock_notifications();
    let (is_open, set_is_open) = signal(false);
    
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();
    let display_notifications = notifications.clone();

    view! {
        <div class="relative">
            // Notification bell button
            <button 
                class="relative p-2.5 rounded-xl bg-panel/50 hover:bg-panel border border-border/30 hover:border-border/50 transition-all"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <svg class="w-5 h-5 text-foreground/70" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                </svg>
                // Unread badge
                {if unread_count > 0 {
                    view! {
                        <span class="absolute -top-1 -right-1 min-w-5 h-5 px-1 rounded-full bg-brand flex items-center justify-center text-xs font-bold text-white">
                            {unread_count}
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </button>

            // Dropdown panel
            <div class=move || format!(
                "absolute right-0 top-full mt-2 w-80 rounded-xl border border-border/50 bg-panel shadow-xl shadow-black/20 z-50 transition-all duration-200 origin-top-right {}",
                if is_open.get() { "opacity-100 scale-100" } else { "opacity-0 scale-95 pointer-events-none" }
            )>
                // Header
                <div class="flex items-center justify-between p-3 border-b border-border/30">
                    <div class="flex items-center gap-2">
                        <span class="font-semibold text-foreground">"Notifications"</span>
                        {if unread_count > 0 {
                            view! {
                                <span class="px-1.5 py-0.5 rounded-full bg-brand/20 text-brand text-xs font-medium">
                                    {unread_count}" new"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                    <button class="text-xs text-foreground/40 hover:text-foreground/70 transition-colors">
                        "Mark all read"
                    </button>
                </div>

                // Notifications list
                <div class="max-h-80 overflow-y-auto scrollbar-thin">
                    {if display_notifications.is_empty() {
                        view! {
                            <div class="p-6 text-center">
                                <div class="w-12 h-12 mx-auto mb-3 rounded-full bg-foreground/5 flex items-center justify-center">
                                    <svg class="w-6 h-6 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                                    </svg>
                                </div>
                                <p class="text-foreground/40 text-sm">"No notifications yet"</p>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="divide-y divide-border/10">
                                {display_notifications.into_iter().map(|notification| {
                                    view! { <NotificationItem notification=notification /> }
                                }).collect_view()}
                            </div>
                        }.into_any()
                    }}
                </div>

                // Footer
                <div class="p-2 border-t border-border/30">
                    <A href="/notifications" attr:class="flex items-center justify-center gap-2 p-2 rounded-lg text-foreground/60 text-sm font-medium hover:text-foreground hover:bg-foreground/5 transition-colors">
                        "View all notifications"
                        <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"></path>
                        </svg>
                    </A>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NotificationItem(notification: Notification) -> impl IntoView {
    let initials: String = notification.actor_handle
        .chars()
        .take(2)
        .collect::<String>()
        .to_uppercase();
    
    let (icon_svg, icon_bg) = get_notification_icon(&notification.notification_type);
    let message = get_notification_message(&notification.actor_handle, &notification.notification_type);

    view! {
        <div class=format!(
            "flex gap-3 p-3 hover:bg-foreground/[0.02] transition-colors cursor-pointer {}",
            if !notification.is_read { "bg-brand/[0.03]" } else { "" }
        )>
            // Avatar with type indicator
            <div class="relative flex-shrink-0">
                {if let Some(avatar_url) = notification.actor_avatar.clone() {
                    view! {
                        <img 
                            src=avatar_url
                            alt=format!("{}'s avatar", notification.actor_handle)
                            class="w-10 h-10 rounded-full object-cover"
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                            <span class="text-brand font-bold text-xs">{initials}</span>
                        </div>
                    }.into_any()
                }}
                // Type indicator
                <div class=format!(
                    "absolute -bottom-1 -right-1 w-5 h-5 rounded-full flex items-center justify-center ring-2 ring-panel {}",
                    icon_bg
                )>
                    <div class="w-3 h-3 text-white" inner_html=icon_svg></div>
                </div>
            </div>

            // Content
            <div class="flex-1 min-w-0">
                <p class="text-sm text-foreground/90 leading-snug">
                    {message}
                </p>
                <time class="text-xs text-foreground/40 mt-1 block">{notification.timestamp}</time>
            </div>

            // Unread indicator
            {if !notification.is_read {
                view! {
                    <div class="flex-shrink-0 w-2 h-2 rounded-full bg-brand mt-2"></div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

fn get_notification_icon(notification_type: &NotificationType) -> (&'static str, &'static str) {
    match notification_type {
        NotificationType::Like { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>"#,
            "bg-rose-500"
        ),
        NotificationType::Comment { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/></svg>"#,
            "bg-violet-500"
        ),
        NotificationType::Follow => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/></svg>"#,
            "bg-sky-500"
        ),
        NotificationType::Mention { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/></svg>"#,
            "bg-amber-500"
        ),
        NotificationType::Reply { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/></svg>"#,
            "bg-indigo-500"
        ),
        NotificationType::SynapseInvite { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/></svg>"#,
            "bg-violet-500"
        ),
    }
}

fn get_notification_message(actor: &str, notification_type: &NotificationType) -> impl IntoView {
    let actor = actor.to_string();
    
    match notification_type.clone() {
        NotificationType::Like { post_preview } => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " liked your post: \""
                <span class="text-foreground/60">{truncate(&post_preview, 30)}</span>
                "\""
            </span>
        }.into_any(),
        NotificationType::Comment { comment_preview, .. } => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " commented: \""
                <span class="text-foreground/60">{truncate(&comment_preview, 30)}</span>
                "\""
            </span>
        }.into_any(),
        NotificationType::Follow => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " started following you"
            </span>
        }.into_any(),
        NotificationType::Mention { post_preview } => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " mentioned you: \""
                <span class="text-foreground/60">{truncate(&post_preview, 30)}</span>
                "\""
            </span>
        }.into_any(),
        NotificationType::Reply { comment_preview } => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " replied: \""
                <span class="text-foreground/60">{truncate(&comment_preview, 30)}</span>
                "\""
            </span>
        }.into_any(),
        NotificationType::SynapseInvite { synapse_name } => view! {
            <span>
                <A href=format!("/profiles/{}", actor) attr:class="font-semibold text-brand hover:underline">{"@"}{actor.clone()}</A>
                " invited you to join "
                <span class="font-medium text-violet-400">{synapse_name.clone()}</span>
            </span>
        }.into_any(),
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len])
    }
}

