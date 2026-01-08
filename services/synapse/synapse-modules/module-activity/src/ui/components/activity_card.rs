// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

/// Types of activities that can occur in the Synapse
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    /// User created a new post
    NewPost {
        post_id: String,
        post_preview: String,
    },
    /// User commented on another user's post
    Comment {
        post_id: String,
        post_owner_handle: String,
        comment_preview: String,
    },
    /// User liked a post
    Like {
        post_id: String,
        post_owner_handle: String,
    },
    /// User joined the Synapse
    UserJoined,
    /// User followed another user
    Follow { followed_handle: String },
    /// User shared/reposted content
    Share {
        post_id: String,
        original_author_handle: String,
    },
    /// User mentioned another user
    Mention {
        post_id: String,
        mentioned_handle: String,
    },
    /// User replied to a comment
    Reply {
        post_id: String,
        replied_to_handle: String,
    },
}

/// Represents a single activity in the feed
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub actor_handle: String,
    pub actor_avatar: Option<String>,
    pub activity_type: ActivityType,
    pub timestamp: String,
    pub is_new: bool,
}

impl Activity {
    /// Get initials from handle for avatar fallback
    fn get_initials(&self) -> String {
        self.actor_handle
            .chars()
            .take(2)
            .collect::<String>()
            .to_uppercase()
    }
}

#[component]
pub fn ActivityCard(activity: Activity) -> impl IntoView {
    let initials = activity.get_initials();
    let actor_handle = activity.actor_handle.clone();
    let actor_handle_for_link = activity.actor_handle.clone();
    let is_new = activity.is_new;

    // Get activity-specific icon and color
    let (icon_svg, icon_bg, icon_color) = get_activity_icon(&activity.activity_type);

    view! {
        <article class=move || format!(
            "group relative flex gap-4 p-4 bg-card/50 border border-border rounded-xl transition-all duration-200 hover:bg-foreground/[0.02] {}",
            if is_new { "bg-brand/[0.03] border border-brand" } else { "" }
        )>
            // Avatar with activity type indicator
            <div class="relative flex-shrink-0">
                {if let Some(avatar_url) = activity.actor_avatar.clone() {
                    view! {
                        <img
                            src=avatar_url
                            alt=format!("{}'s avatar", actor_handle)
                            class="w-10 h-10 rounded-full object-cover ring-2 ring-border/30"
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-2 ring-border/30 flex items-center justify-center">
                            <span class="text-brand font-bold text-xs">{initials}</span>
                        </div>
                    }.into_any()
                }}

                // Activity type icon badge
                <div class=format!(
                    "absolute -bottom-1 -right-1 w-5 h-5 rounded-full flex items-center justify-center ring-2 ring-panel {}",
                    icon_bg
                )>
                    <div class=format!("w-3 h-3 {}", icon_color) inner_html=icon_svg></div>
                </div>
            </div>

            // Activity content
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between gap-2">
                    <p class="text-sm text-foreground/90 leading-relaxed">
                        {render_activity_content(activity.clone())}
                    </p>

                    // New indicator
                    {if is_new {
                        view! {
                            <span class="flex-shrink-0 w-2 h-2 rounded-full bg-brand animate-pulse"></span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>

                // Timestamp
                <time class="text-xs text-foreground/40 mt-1 block">{activity.timestamp}</time>
            </div>
        </article>
    }
}

/// Renders the activity content with proper links
fn render_activity_content(activity: Activity) -> impl IntoView {
    let actor = activity.actor_handle.clone();

    match activity.activity_type {
        ActivityType::NewPost {
            post_id,
            post_preview,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " created a new "
                <PostLink post_id=post_id label="post".to_string() />
                <span class="text-foreground/50">" — \""</span>
                <span class="text-foreground/70 italic">{truncate_text(&post_preview, 60)}</span>
                <span class="text-foreground/50">"\""</span>
            </span>
        }
        .into_any(),
        ActivityType::Comment {
            post_id,
            post_owner_handle,
            comment_preview,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " commented on "
                <HandleLink handle=post_owner_handle />
                "'s "
                <PostLink post_id=post_id label="post".to_string() />
                <span class="text-foreground/50">" — \""</span>
                <span class="text-foreground/70 italic">{truncate_text(&comment_preview, 50)}</span>
                <span class="text-foreground/50">"\""</span>
            </span>
        }
        .into_any(),
        ActivityType::Like {
            post_id,
            post_owner_handle,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " liked "
                <HandleLink handle=post_owner_handle />
                "'s "
                <PostLink post_id=post_id label="post".to_string() />
            </span>
        }
        .into_any(),
        ActivityType::UserJoined => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " just joined the Synapse! "
                <span class="text-foreground/50">"🎉"</span>
            </span>
        }
        .into_any(),
        ActivityType::Follow { followed_handle } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " started following "
                <HandleLink handle=followed_handle />
            </span>
        }
        .into_any(),
        ActivityType::Share {
            post_id,
            original_author_handle,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " shared "
                <HandleLink handle=original_author_handle />
                "'s "
                <PostLink post_id=post_id label="post".to_string() />
            </span>
        }
        .into_any(),
        ActivityType::Mention {
            post_id,
            mentioned_handle,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " mentioned "
                <HandleLink handle=mentioned_handle />
                " in a "
                <PostLink post_id=post_id label="post".to_string() />
            </span>
        }
        .into_any(),
        ActivityType::Reply {
            post_id,
            replied_to_handle,
        } => view! {
            <span>
                <HandleLink handle=actor.clone() />
                " replied to "
                <HandleLink handle=replied_to_handle />
                " in a "
                <PostLink post_id=post_id label="thread".to_string() />
            </span>
        }
        .into_any(),
    }
}

/// A styled link to a user's profile
#[component]
fn HandleLink(handle: String) -> impl IntoView {
    let href = format!("/profiles/{}", handle);
    view! {
        <A
            href=href
            attr:class="font-semibold text-brand hover:text-brand/80 hover:underline transition-colors"
        >
            "@"{handle}
        </A>
    }
}

/// A styled link to a post
#[component]
fn PostLink(post_id: String, label: String) -> impl IntoView {
    let href = format!("/posts/{}", post_id);
    view! {
        <A
            href=href
            attr:class="font-medium text-foreground/80 hover:text-brand hover:underline transition-colors"
        >
            {label}
        </A>
    }
}

/// Get the appropriate icon for each activity type
fn get_activity_icon(activity_type: &ActivityType) -> (&'static str, &'static str, &'static str) {
    match activity_type {
        ActivityType::NewPost { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/></svg>"#,
            "bg-info",
            "text-white",
        ),
        ActivityType::Comment { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/></svg>"#,
            "bg-secondary",
            "text-white",
        ),
        ActivityType::Like { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="currentColor" stroke="none"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>"#,
            "bg-error",
            "text-white",
        ),
        ActivityType::UserJoined => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/></svg>"#,
            "bg-success",
            "text-white",
        ),
        ActivityType::Follow { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/></svg>"#,
            "bg-info",
            "text-white",
        ),
        ActivityType::Share { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/></svg>"#,
            "bg-success",
            "text-white",
        ),
        ActivityType::Mention { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/></svg>"#,
            "bg-warning",
            "text-white",
        ),
        ActivityType::Reply { .. } => (
            r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/></svg>"#,
            "bg-secondary",
            "text-white",
        ),
    }
}

/// Truncate text with ellipsis
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}…", &text[..max_len])
    }
}
