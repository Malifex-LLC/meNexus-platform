// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::ui::components::activity_card::{Activity, ActivityCard, ActivityType};
use leptos::prelude::*;

/// Generate mock activities for development/preview
fn get_mock_activities() -> Vec<Activity> {
    vec![
        Activity {
            id: "act-001".to_string(),
            actor_handle: "neo".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::NewPost {
                post_id: "post-123".to_string(),
                post_preview: "Just discovered something incredible about the Matrix. The code is everywhere, you just have to see it...".to_string(),
            },
            timestamp: "2 minutes ago".to_string(),
            is_new: true,
        },
        Activity {
            id: "act-002".to_string(),
            actor_handle: "trinity".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Comment {
                post_id: "post-456".to_string(),
                post_owner_handle: "morpheus".to_string(),
                comment_preview: "The answer is out there, Neo. It's looking for you.".to_string(),
            },
            timestamp: "5 minutes ago".to_string(),
            is_new: true,
        },
        Activity {
            id: "act-003".to_string(),
            actor_handle: "cipher".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::UserJoined,
            timestamp: "12 minutes ago".to_string(),
            is_new: true,
        },
        Activity {
            id: "act-004".to_string(),
            actor_handle: "morpheus".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Like {
                post_id: "post-789".to_string(),
                post_owner_handle: "neo".to_string(),
            },
            timestamp: "18 minutes ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-005".to_string(),
            actor_handle: "oracle".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Follow {
                followed_handle: "neo".to_string(),
            },
            timestamp: "25 minutes ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-006".to_string(),
            actor_handle: "tank".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Share {
                post_id: "post-101".to_string(),
                original_author_handle: "morpheus".to_string(),
            },
            timestamp: "32 minutes ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-007".to_string(),
            actor_handle: "niobe".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Mention {
                post_id: "post-202".to_string(),
                mentioned_handle: "morpheus".to_string(),
            },
            timestamp: "45 minutes ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-008".to_string(),
            actor_handle: "smith".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::Reply {
                post_id: "post-303".to_string(),
                replied_to_handle: "neo".to_string(),
            },
            timestamp: "1 hour ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-009".to_string(),
            actor_handle: "seraph".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::NewPost {
                post_id: "post-404".to_string(),
                post_preview: "You do not truly know someone until you fight them.".to_string(),
            },
            timestamp: "1 hour ago".to_string(),
            is_new: false,
        },
        Activity {
            id: "act-010".to_string(),
            actor_handle: "keymaker".to_string(),
            actor_avatar: None,
            activity_type: ActivityType::UserJoined,
            timestamp: "2 hours ago".to_string(),
            is_new: false,
        },
    ]
}

#[component]
pub fn ActivityFeed() -> impl IntoView {
    let activities = get_mock_activities();
    let new_count = activities.iter().filter(|a| a.is_new).count();

    view! {
        <div class="flex flex-col h-full bg-panel border border-border/30 rounded-xl overflow-hidden">
            // Header
            <header class="flex-shrink-0 px-4 py-3 border-b border-border/30 bg-panel/50 backdrop-blur-sm">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        // Icon
                        <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-brand to-brand/60 flex items-center justify-center">
                            <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                            </svg>
                        </div>
                        <div>
                            <h2 class="text-foreground font-semibold tracking-tight">"Activity"</h2>
                            <p class="text-foreground/40 text-xs">"What's happening in the Synapse"</p>
                        </div>
                    </div>

                    // New activity count badge
                    {if new_count > 0 {
                        view! {
                            <span class="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-brand/15 text-brand text-xs font-medium">
                                <span class="w-1.5 h-1.5 rounded-full bg-brand animate-pulse"></span>
                                {new_count}" new"
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </header>

            // Filter tabs
            <div class="flex-shrink-0 px-4 py-2 border-b border-border/20 bg-background/30">
                <div class="flex items-center gap-1">
                    <FilterTab label="All" active=true />
                    <FilterTab label="Posts" active=false />
                    <FilterTab label="Comments" active=false />
                    <FilterTab label="Follows" active=false />
                </div>
            </div>

            // Activity list
            <div class="flex-1 overflow-y-auto scrollbar-styled">
                <div class="divide-y divide-border/10 space-y-4 p-4">
                    {activities.into_iter().map(|activity| {
                        view! {
                            <ActivityCard activity=activity />
                        }
                    }).collect_view()}
                </div>

                // Load more button
                <div class="p-4">
                    <button class="w-full py-2.5 px-4 rounded-lg border border-border/30 text-foreground/50 text-sm font-medium hover:bg-foreground/5 hover:text-foreground/70 hover:border-border/50 transition-all">
                        "Load more activity"
                    </button>
                </div>
            </div>

            // Footer with live indicator
            <footer class="flex-shrink-0 px-4 py-2 border-t border-border/20 bg-panel/30">
                <div class="flex items-center justify-between text-xs">
                    <div class="flex items-center gap-2 text-foreground/40">
                        <span class="flex items-center gap-1.5">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse"></span>
                            "Live updates"
                        </span>
                    </div>
                    <button class="text-foreground/40 hover:text-foreground/70 transition-colors">
                        "Mark all as read"
                    </button>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn FilterTab(label: &'static str, active: bool) -> impl IntoView {
    view! {
        <button class=format!(
            "px-3 py-1.5 rounded-md text-xs font-medium transition-all {}",
            if active {
                "bg-brand/15 text-brand"
            } else {
                "text-foreground/50 hover:text-foreground/70 hover:bg-foreground/5"
            }
        )>
            {label}
        </button>
    }
}
