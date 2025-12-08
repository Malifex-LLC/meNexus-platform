// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};

/// User roles in a Synapse
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemberRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

impl MemberRole {
    /// Get display label for the role
    pub fn label(&self) -> &'static str {
        match self {
            MemberRole::Owner => "Owner",
            MemberRole::Admin => "Admin",
            MemberRole::Moderator => "Moderator",
            MemberRole::Member => "Member",
        }
    }

    /// Get badge styling classes for the role
    pub fn badge_classes(&self) -> &'static str {
        match self {
            MemberRole::Owner => "bg-amber-500/20 text-amber-400 border-amber-500/30",
            MemberRole::Admin => "bg-rose-500/20 text-rose-400 border-rose-500/30",
            MemberRole::Moderator => "bg-violet-500/20 text-violet-400 border-violet-500/30",
            MemberRole::Member => "bg-foreground/10 text-foreground/50 border-foreground/20",
        }
    }

    /// Get the icon SVG for the role
    pub fn icon_svg(&self) -> &'static str {
        match self {
            MemberRole::Owner => r#"<svg viewBox="0 0 24 24" fill="currentColor" class="w-3 h-3"><path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z"/></svg>"#,
            MemberRole::Admin => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-3 h-3"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"/></svg>"#,
            MemberRole::Moderator => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-3 h-3"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"/></svg>"#,
            MemberRole::Member => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" class="w-3 h-3"><path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"/></svg>"#,
        }
    }
}

/// Online status of a member
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum OnlineStatus {
    Online,
    Away,
    DoNotDisturb,
    Offline,
}

impl OnlineStatus {
    /// Get display label for the status
    pub fn label(&self) -> &'static str {
        match self {
            OnlineStatus::Online => "Online",
            OnlineStatus::Away => "Away",
            OnlineStatus::DoNotDisturb => "Do Not Disturb",
            OnlineStatus::Offline => "Offline",
        }
    }

    /// Get the status indicator color class
    pub fn indicator_class(&self) -> &'static str {
        match self {
            OnlineStatus::Online => "bg-emerald-400",
            OnlineStatus::Away => "bg-amber-400",
            OnlineStatus::DoNotDisturb => "bg-rose-400",
            OnlineStatus::Offline => "bg-foreground/30",
        }
    }

    /// Check if user is considered online (for grouping)
    pub fn is_online(&self) -> bool {
        matches!(self, OnlineStatus::Online | OnlineStatus::Away | OnlineStatus::DoNotDisturb)
    }
}

/// Represents a member of a Synapse
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: MemberRole,
    pub status: OnlineStatus,
    pub status_message: Option<String>,
    pub joined_at: String,
    pub last_seen: Option<String>,
    pub is_streaming: bool,
    pub is_verified: bool,
}

impl Member {
    /// Get initials from display name for avatar fallback
    pub fn get_initials(&self) -> String {
        self.display_name
            .split_whitespace()
            .take(2)
            .filter_map(|s| s.chars().next())
            .collect::<String>()
            .to_uppercase()
    }

    /// Generate mock members for UI development
    pub fn mock_members() -> Vec<Member> {
        vec![
            // Online members
            Member {
                id: "user-1".to_string(),
                handle: "neo".to_string(),
                display_name: "Neo Anderson".to_string(),
                avatar_url: None,
                role: MemberRole::Owner,
                status: OnlineStatus::Online,
                status_message: Some("Taking the red pill 💊".to_string()),
                joined_at: "Dec 2024".to_string(),
                last_seen: None,
                is_streaming: true,
                is_verified: true,
            },
            Member {
                id: "user-2".to_string(),
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
                avatar_url: None,
                role: MemberRole::Admin,
                status: OnlineStatus::Online,
                status_message: Some("Debugging the Matrix".to_string()),
                joined_at: "Dec 2024".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: true,
            },
            Member {
                id: "user-3".to_string(),
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
                avatar_url: None,
                role: MemberRole::Admin,
                status: OnlineStatus::DoNotDisturb,
                status_message: Some("In a meeting".to_string()),
                joined_at: "Dec 2024".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: true,
            },
            Member {
                id: "user-4".to_string(),
                handle: "tank".to_string(),
                display_name: "Tank".to_string(),
                avatar_url: None,
                role: MemberRole::Moderator,
                status: OnlineStatus::Online,
                status_message: None,
                joined_at: "Jan 2025".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-5".to_string(),
                handle: "niobe".to_string(),
                display_name: "Niobe".to_string(),
                avatar_url: None,
                role: MemberRole::Moderator,
                status: OnlineStatus::Away,
                status_message: Some("brb".to_string()),
                joined_at: "Jan 2025".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-6".to_string(),
                handle: "oracle".to_string(),
                display_name: "The Oracle".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Online,
                status_message: Some("🍪".to_string()),
                joined_at: "Jan 2025".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: true,
            },
            Member {
                id: "user-7".to_string(),
                handle: "keymaker".to_string(),
                display_name: "The Keymaker".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Online,
                status_message: None,
                joined_at: "Feb 2025".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-8".to_string(),
                handle: "seraph".to_string(),
                display_name: "Seraph".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Online,
                status_message: Some("Guarding the Oracle".to_string()),
                joined_at: "Feb 2025".to_string(),
                last_seen: None,
                is_streaming: false,
                is_verified: false,
            },
            // Offline members
            Member {
                id: "user-9".to_string(),
                handle: "cypher".to_string(),
                display_name: "Cypher Reagan".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Offline,
                status_message: Some("Ignorance is bliss".to_string()),
                joined_at: "Dec 2024".to_string(),
                last_seen: Some("2 hours ago".to_string()),
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-10".to_string(),
                handle: "dozer".to_string(),
                display_name: "Dozer".to_string(),
                avatar_url: None,
                role: MemberRole::Moderator,
                status: OnlineStatus::Offline,
                status_message: None,
                joined_at: "Jan 2025".to_string(),
                last_seen: Some("5 hours ago".to_string()),
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-11".to_string(),
                handle: "apoc".to_string(),
                display_name: "Apoc".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Offline,
                status_message: None,
                joined_at: "Jan 2025".to_string(),
                last_seen: Some("1 day ago".to_string()),
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-12".to_string(),
                handle: "switch".to_string(),
                display_name: "Switch".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Offline,
                status_message: Some("Not the residual self-image".to_string()),
                joined_at: "Jan 2025".to_string(),
                last_seen: Some("3 days ago".to_string()),
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-13".to_string(),
                handle: "mouse".to_string(),
                display_name: "Mouse".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Offline,
                status_message: Some("Working on new simulations".to_string()),
                joined_at: "Feb 2025".to_string(),
                last_seen: Some("1 week ago".to_string()),
                is_streaming: false,
                is_verified: false,
            },
            Member {
                id: "user-14".to_string(),
                handle: "merovingian".to_string(),
                display_name: "The Merovingian".to_string(),
                avatar_url: None,
                role: MemberRole::Member,
                status: OnlineStatus::Offline,
                status_message: Some("Causality is the only true constant".to_string()),
                joined_at: "Mar 2025".to_string(),
                last_seen: Some("2 weeks ago".to_string()),
                is_streaming: false,
                is_verified: true,
            },
        ]
    }

    /// Get online members grouped by role
    pub fn get_online_by_role(members: &[Member]) -> Vec<(MemberRole, Vec<Member>)> {
        let mut result: Vec<(MemberRole, Vec<Member>)> = vec![
            (MemberRole::Owner, vec![]),
            (MemberRole::Admin, vec![]),
            (MemberRole::Moderator, vec![]),
            (MemberRole::Member, vec![]),
        ];

        for member in members.iter().filter(|m| m.status.is_online()) {
            match member.role {
                MemberRole::Owner => result[0].1.push(member.clone()),
                MemberRole::Admin => result[1].1.push(member.clone()),
                MemberRole::Moderator => result[2].1.push(member.clone()),
                MemberRole::Member => result[3].1.push(member.clone()),
            }
        }

        // Filter out empty groups
        result.into_iter().filter(|(_, members)| !members.is_empty()).collect()
    }

    /// Get offline members grouped by role
    pub fn get_offline_by_role(members: &[Member]) -> Vec<(MemberRole, Vec<Member>)> {
        let mut result: Vec<(MemberRole, Vec<Member>)> = vec![
            (MemberRole::Owner, vec![]),
            (MemberRole::Admin, vec![]),
            (MemberRole::Moderator, vec![]),
            (MemberRole::Member, vec![]),
        ];

        for member in members.iter().filter(|m| !m.status.is_online()) {
            match member.role {
                MemberRole::Owner => result[0].1.push(member.clone()),
                MemberRole::Admin => result[1].1.push(member.clone()),
                MemberRole::Moderator => result[2].1.push(member.clone()),
                MemberRole::Member => result[3].1.push(member.clone()),
            }
        }

        // Filter out empty groups
        result.into_iter().filter(|(_, members)| !members.is_empty()).collect()
    }
}
