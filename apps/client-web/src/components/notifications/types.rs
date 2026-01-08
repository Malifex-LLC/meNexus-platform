// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// Categories for filtering notifications
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationCategory {
    #[default]
    All,
    Broadcasts,
    Content,
    Connections,
    System,
}

impl NotificationCategory {
    pub fn all() -> Vec<Self> {
        vec![
            Self::All,
            Self::Broadcasts,
            Self::Content,
            Self::Connections,
            Self::System,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Broadcasts => "Broadcasts",
            Self::Content => "Content",
            Self::Connections => "Connections",
            Self::System => "System",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::All => "All notifications",
            Self::Broadcasts => "Network-wide alerts",
            Self::Content => "Reactions, shares, mentions",
            Self::Connections => "Followers & connections",
            Self::System => "Account & security",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::All => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>"#,
            Self::Broadcasts => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>"#,
            Self::Content => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>"#,
            Self::Connections => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>"#,
            Self::System => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>"#,
        }
    }
}

/// Priority level for broadcasts and alerts
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlertPriority {
    Critical,
    High,
    Normal,
    Low,
}

impl AlertPriority {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "Critical",
            Self::High => "Important",
            Self::Normal => "Info",
            Self::Low => "Notice",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Critical => "bg-error/20 text-error border-error/30",
            Self::High => "bg-warning/20 text-warning border-warning/30",
            Self::Normal => "bg-brand/20 text-brand border-brand/30",
            Self::Low => "bg-foreground/10 text-foreground/60 border-foreground/20",
        }
    }

    pub fn indicator_class(&self) -> &'static str {
        match self {
            Self::Critical => "bg-error",
            Self::High => "bg-warning",
            Self::Normal => "bg-brand",
            Self::Low => "bg-foreground/40",
        }
    }
}

/// Network broadcast alert from meNexus core
#[derive(Clone)]
pub struct BroadcastAlert {
    pub id: String,
    pub title: String,
    pub message: String,
    pub priority: AlertPriority,
    pub timestamp: String,
    pub source: String,
    pub is_read: bool,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
}

impl BroadcastAlert {
    pub fn mock_alerts() -> Vec<Self> {
        vec![
            Self {
                id: "broadcast-001".to_string(),
                title: "Network Protocol Update v0.3.1".to_string(),
                message: "A critical security patch has been deployed. All nodes will automatically update within 24 hours. No action required.".to_string(),
                priority: AlertPriority::High,
                timestamp: "2 hours ago".to_string(),
                source: "meNexus Core".to_string(),
                is_read: false,
                action_url: Some("/changelog".to_string()),
                action_label: Some("View Changelog".to_string()),
            },
            Self {
                id: "broadcast-002".to_string(),
                title: "Scheduled Maintenance Window".to_string(),
                message: "The network will undergo maintenance on Dec 15th, 02:00-04:00 UTC. Expect brief sync delays.".to_string(),
                priority: AlertPriority::Normal,
                timestamp: "1 day ago".to_string(),
                source: "meNexus Operations".to_string(),
                is_read: true,
                action_url: None,
                action_label: None,
            },
            Self {
                id: "broadcast-003".to_string(),
                title: "New Feature: Encrypted Direct Messages".to_string(),
                message: "End-to-end encrypted DMs are now available! Your messages are secured with quantum-resistant encryption.".to_string(),
                priority: AlertPriority::Normal,
                timestamp: "3 days ago".to_string(),
                source: "meNexus Team".to_string(),
                is_read: true,
                action_url: Some("/messages".to_string()),
                action_label: Some("Try It Now".to_string()),
            },
            Self {
                id: "broadcast-004".to_string(),
                title: "Community Guidelines Update".to_string(),
                message: "We've updated our community guidelines to clarify content policies. Please review the changes.".to_string(),
                priority: AlertPriority::Low,
                timestamp: "1 week ago".to_string(),
                source: "meNexus Trust & Safety".to_string(),
                is_read: true,
                action_url: Some("/guidelines".to_string()),
                action_label: Some("Read Guidelines".to_string()),
            },
        ]
    }
}

/// Types of content-related notifications
#[derive(Clone)]
pub enum ContentNotificationType {
    Like,
    Reaction { emoji: String },
    Comment,
    Reply,
    Share,
    Repost,
    Mention,
    Quote,
    PollVote,
    ThreadReply,
}

impl ContentNotificationType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Like => "liked your post",
            Self::Reaction { .. } => "reacted to your post",
            Self::Comment => "commented on your post",
            Self::Reply => "replied to your comment",
            Self::Share => "shared your post",
            Self::Repost => "reposted your post",
            Self::Mention => "mentioned you",
            Self::Quote => "quoted your post",
            Self::PollVote => "voted on your poll",
            Self::ThreadReply => "replied in your thread",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::Like => r#"<path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>"#,
            Self::Reaction { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>"#,
            Self::Comment => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>"#,
            Self::Reply => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M3 10h10a8 8 0 018 8v2M3 10l6 6m-6-6l6-6"/>"#,
            Self::Share => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/>"#,
            Self::Repost => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>"#,
            Self::Mention => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>"#,
            Self::Quote => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"/>"#,
            Self::PollVote => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>"#,
            Self::ThreadReply => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M17 8h2a2 2 0 012 2v6a2 2 0 01-2 2h-2v4l-4-4H9a1.994 1.994 0 01-1.414-.586m0 0L11 14h4a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2v4l.586-.586z"/>"#,
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            Self::Like => "text-notif-like bg-notif-like/15",
            Self::Reaction { .. } => "text-warning bg-warning/15",
            Self::Comment | Self::Reply | Self::ThreadReply => "text-notif-reply bg-notif-reply/15",
            Self::Share | Self::Repost => "text-success bg-success/15",
            Self::Mention | Self::Quote => "text-notif-mention bg-notif-mention/15",
            Self::PollVote => "text-secondary bg-secondary/15",
        }
    }
}

/// Content notification
#[derive(Clone)]
pub struct ContentNotification {
    pub id: String,
    pub actor_handle: String,
    pub actor_display_name: String,
    pub actor_avatar: Option<String>,
    pub actor_verified: bool,
    pub notification_type: ContentNotificationType,
    pub post_preview: Option<String>,
    pub comment_preview: Option<String>,
    pub synapse_name: Option<String>,
    pub timestamp: String,
    pub is_read: bool,
}

impl ContentNotification {
    pub fn mock_notifications() -> Vec<Self> {
        vec![
            Self {
                id: "content-001".to_string(),
                actor_handle: "morpheus".to_string(),
                actor_display_name: "Morpheus".to_string(),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ContentNotificationType::Like,
                post_preview: Some("The future of decentralized identity is here...".to_string()),
                comment_preview: None,
                synapse_name: Some("Tech Pioneers".to_string()),
                timestamp: "2 minutes ago".to_string(),
                is_read: false,
            },
            Self {
                id: "content-002".to_string(),
                actor_handle: "trinity".to_string(),
                actor_display_name: "Trinity".to_string(),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ContentNotificationType::Comment,
                post_preview: Some("Check out this new cryptographic protocol...".to_string()),
                comment_preview: Some("This is exactly what we've been waiting for! The implementation looks solid.".to_string()),
                synapse_name: Some("Cryptography".to_string()),
                timestamp: "15 minutes ago".to_string(),
                is_read: false,
            },
            Self {
                id: "content-003".to_string(),
                actor_handle: "oracle".to_string(),
                actor_display_name: "The Oracle".to_string(),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ContentNotificationType::Reaction { emoji: "🔥".to_string() },
                post_preview: Some("Just deployed my first smart contract...".to_string()),
                comment_preview: None,
                synapse_name: Some("Web3 Builders".to_string()),
                timestamp: "1 hour ago".to_string(),
                is_read: false,
            },
            Self {
                id: "content-004".to_string(),
                actor_handle: "niobe".to_string(),
                actor_display_name: "Niobe".to_string(),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ContentNotificationType::Share,
                post_preview: Some("Why P2P networks will replace centralized platforms...".to_string()),
                comment_preview: None,
                synapse_name: None,
                timestamp: "2 hours ago".to_string(),
                is_read: true,
            },
            Self {
                id: "content-005".to_string(),
                actor_handle: "tank".to_string(),
                actor_display_name: "Tank".to_string(),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ContentNotificationType::Mention,
                post_preview: Some("Hey @neo, what do you think about this approach?".to_string()),
                comment_preview: None,
                synapse_name: Some("Dev Talk".to_string()),
                timestamp: "3 hours ago".to_string(),
                is_read: true,
            },
            Self {
                id: "content-006".to_string(),
                actor_handle: "keymaker".to_string(),
                actor_display_name: "The Keymaker".to_string(),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ContentNotificationType::Reply,
                post_preview: None,
                comment_preview: Some("Great insight! I've been thinking the same thing about key management.".to_string()),
                synapse_name: Some("Security".to_string()),
                timestamp: "5 hours ago".to_string(),
                is_read: true,
            },
            Self {
                id: "content-007".to_string(),
                actor_handle: "merovingian".to_string(),
                actor_display_name: "Merovingian".to_string(),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ContentNotificationType::Quote,
                post_preview: Some("This is what happens when you understand causality...".to_string()),
                comment_preview: None,
                synapse_name: None,
                timestamp: "1 day ago".to_string(),
                is_read: true,
            },
            Self {
                id: "content-008".to_string(),
                actor_handle: "seraph".to_string(),
                actor_display_name: "Seraph".to_string(),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ContentNotificationType::PollVote,
                post_preview: Some("What's the best consensus mechanism?".to_string()),
                comment_preview: None,
                synapse_name: Some("Blockchain".to_string()),
                timestamp: "2 days ago".to_string(),
                is_read: true,
            },
        ]
    }
}

/// Types of connection-related notifications  
#[derive(Clone)]
pub enum ConnectionNotificationType {
    NewFollower,
    FollowBack,
    FollowRequest,
    FollowAccepted,
    SynapseInvite { synapse_name: String },
    SynapseJoined { synapse_name: String },
    SynapseRoleChange { synapse_name: String, new_role: String },
    EndorsementReceived,
    CredentialAwarded { credential_name: String },
}

impl ConnectionNotificationType {
    pub fn label(&self) -> String {
        match self {
            Self::NewFollower => "started following you".to_string(),
            Self::FollowBack => "followed you back".to_string(),
            Self::FollowRequest => "requested to follow you".to_string(),
            Self::FollowAccepted => "accepted your follow request".to_string(),
            Self::SynapseInvite { synapse_name } => format!("invited you to join {}", synapse_name),
            Self::SynapseJoined { synapse_name } => format!("joined {} too", synapse_name),
            Self::SynapseRoleChange { synapse_name, new_role } => format!("promoted you to {} in {}", new_role, synapse_name),
            Self::EndorsementReceived => "endorsed you".to_string(),
            Self::CredentialAwarded { credential_name } => format!("You earned the {} credential", credential_name),
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::NewFollower | Self::FollowBack => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>"#,
            Self::FollowRequest | Self::FollowAccepted => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"/>"#,
            Self::SynapseInvite { .. } | Self::SynapseJoined { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"/>"#,
            Self::SynapseRoleChange { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>"#,
            Self::EndorsementReceived => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M14 10h4.764a2 2 0 011.789 2.894l-3.5 7A2 2 0 0115.263 21h-4.017c-.163 0-.326-.02-.485-.06L7 20m7-10V5a2 2 0 00-2-2h-.095c-.5 0-.905.405-.905.905 0 .714-.211 1.412-.608 2.006L7 11v9m7-10h-2M7 20H5a2 2 0 01-2-2v-6a2 2 0 012-2h2.5"/>"#,
            Self::CredentialAwarded { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>"#,
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            Self::NewFollower | Self::FollowBack => "text-notif-follow bg-notif-follow/15",
            Self::FollowRequest | Self::FollowAccepted => "text-secondary bg-secondary/15",
            Self::SynapseInvite { .. } | Self::SynapseJoined { .. } => "text-secondary bg-secondary/15",
            Self::SynapseRoleChange { .. } => "text-warning bg-warning/15",
            Self::EndorsementReceived => "text-success bg-success/15",
            Self::CredentialAwarded { .. } => "text-brand bg-brand/15",
        }
    }

    pub fn has_actions(&self) -> bool {
        matches!(self, Self::FollowRequest | Self::SynapseInvite { .. })
    }
}

/// Connection notification
#[derive(Clone)]
pub struct ConnectionNotification {
    pub id: String,
    pub actor_handle: Option<String>,
    pub actor_display_name: Option<String>,
    pub actor_avatar: Option<String>,
    pub actor_verified: bool,
    pub notification_type: ConnectionNotificationType,
    pub mutual_count: Option<u32>,
    pub timestamp: String,
    pub is_read: bool,
}

impl ConnectionNotification {
    pub fn mock_notifications() -> Vec<Self> {
        vec![
            Self {
                id: "conn-001".to_string(),
                actor_handle: Some("agent_smith".to_string()),
                actor_display_name: Some("Agent Smith".to_string()),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ConnectionNotificationType::NewFollower,
                mutual_count: Some(12),
                timestamp: "5 minutes ago".to_string(),
                is_read: false,
            },
            Self {
                id: "conn-002".to_string(),
                actor_handle: Some("neo_official".to_string()),
                actor_display_name: Some("Neo".to_string()),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ConnectionNotificationType::FollowBack,
                mutual_count: Some(47),
                timestamp: "30 minutes ago".to_string(),
                is_read: false,
            },
            Self {
                id: "conn-003".to_string(),
                actor_handle: Some("architect".to_string()),
                actor_display_name: Some("The Architect".to_string()),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ConnectionNotificationType::SynapseInvite { 
                    synapse_name: "Matrix Core".to_string() 
                },
                mutual_count: None,
                timestamp: "1 hour ago".to_string(),
                is_read: false,
            },
            Self {
                id: "conn-004".to_string(),
                actor_handle: Some("switch_matrix".to_string()),
                actor_display_name: Some("Switch".to_string()),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ConnectionNotificationType::FollowRequest,
                mutual_count: Some(3),
                timestamp: "2 hours ago".to_string(),
                is_read: true,
            },
            Self {
                id: "conn-005".to_string(),
                actor_handle: Some("dozer".to_string()),
                actor_display_name: Some("Dozer".to_string()),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ConnectionNotificationType::SynapseJoined { 
                    synapse_name: "Tech Pioneers".to_string() 
                },
                mutual_count: Some(8),
                timestamp: "4 hours ago".to_string(),
                is_read: true,
            },
            Self {
                id: "conn-006".to_string(),
                actor_handle: Some("morpheus".to_string()),
                actor_display_name: Some("Morpheus".to_string()),
                actor_avatar: None,
                actor_verified: true,
                notification_type: ConnectionNotificationType::EndorsementReceived,
                mutual_count: None,
                timestamp: "1 day ago".to_string(),
                is_read: true,
            },
            Self {
                id: "conn-007".to_string(),
                actor_handle: None,
                actor_display_name: None,
                actor_avatar: None,
                actor_verified: false,
                notification_type: ConnectionNotificationType::CredentialAwarded { 
                    credential_name: "Early Adopter".to_string() 
                },
                mutual_count: None,
                timestamp: "3 days ago".to_string(),
                is_read: true,
            },
            Self {
                id: "conn-008".to_string(),
                actor_handle: Some("cypher".to_string()),
                actor_display_name: Some("Cypher".to_string()),
                actor_avatar: None,
                actor_verified: false,
                notification_type: ConnectionNotificationType::SynapseRoleChange { 
                    synapse_name: "Cryptography".to_string(),
                    new_role: "Moderator".to_string(),
                },
                mutual_count: None,
                timestamp: "1 week ago".to_string(),
                is_read: true,
            },
        ]
    }
}

/// System notification types
#[derive(Clone)]
pub enum SystemNotificationType {
    SecurityAlert { description: String },
    LoginDetected { location: String, device: String },
    PasswordChanged,
    TwoFactorEnabled,
    EmailVerified,
    AccountRecoverySet,
    ApiKeyCreated { key_name: String },
    StorageWarning { percent_used: u8 },
    ExportReady,
    SyncCompleted { records: u32 },
}

impl SystemNotificationType {
    pub fn label(&self) -> String {
        match self {
            Self::SecurityAlert { description } => description.clone(),
            Self::LoginDetected { location, device } => format!("New login from {} on {}", location, device),
            Self::PasswordChanged => "Your password was changed".to_string(),
            Self::TwoFactorEnabled => "Two-factor authentication enabled".to_string(),
            Self::EmailVerified => "Email address verified".to_string(),
            Self::AccountRecoverySet => "Account recovery options updated".to_string(),
            Self::ApiKeyCreated { key_name } => format!("API key '{}' created", key_name),
            Self::StorageWarning { percent_used } => format!("Storage {}% full", percent_used),
            Self::ExportReady => "Your data export is ready".to_string(),
            Self::SyncCompleted { records } => format!("Synced {} records", records),
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::SecurityAlert { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>"#,
            Self::LoginDetected { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>"#,
            Self::PasswordChanged | Self::TwoFactorEnabled | Self::AccountRecoverySet => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>"#,
            Self::EmailVerified => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>"#,
            Self::ApiKeyCreated { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>"#,
            Self::StorageWarning { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"/>"#,
            Self::ExportReady => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>"#,
            Self::SyncCompleted { .. } => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>"#,
        }
    }

    pub fn icon_color(&self) -> &'static str {
        match self {
            Self::SecurityAlert { .. } => "text-error bg-error/15",
            Self::LoginDetected { .. } => "text-warning bg-warning/15",
            Self::PasswordChanged | Self::TwoFactorEnabled | Self::AccountRecoverySet | Self::EmailVerified => "text-success bg-success/15",
            Self::ApiKeyCreated { .. } => "text-secondary bg-secondary/15",
            Self::StorageWarning { .. } => "text-warning bg-warning/15",
            Self::ExportReady | Self::SyncCompleted { .. } => "text-info bg-info/15",
        }
    }
}

/// System notification
#[derive(Clone)]
pub struct SystemNotification {
    pub id: String,
    pub notification_type: SystemNotificationType,
    pub timestamp: String,
    pub is_read: bool,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
}

impl SystemNotification {
    pub fn mock_notifications() -> Vec<Self> {
        vec![
            Self {
                id: "sys-001".to_string(),
                notification_type: SystemNotificationType::LoginDetected { 
                    location: "San Francisco, CA".to_string(),
                    device: "Chrome/MacOS".to_string(),
                },
                timestamp: "10 minutes ago".to_string(),
                is_read: false,
                action_url: Some("/settings/security".to_string()),
                action_label: Some("Review".to_string()),
            },
            Self {
                id: "sys-002".to_string(),
                notification_type: SystemNotificationType::SyncCompleted { records: 1247 },
                timestamp: "1 hour ago".to_string(),
                is_read: true,
                action_url: None,
                action_label: None,
            },
            Self {
                id: "sys-003".to_string(),
                notification_type: SystemNotificationType::TwoFactorEnabled,
                timestamp: "2 days ago".to_string(),
                is_read: true,
                action_url: None,
                action_label: None,
            },
            Self {
                id: "sys-004".to_string(),
                notification_type: SystemNotificationType::ExportReady,
                timestamp: "1 week ago".to_string(),
                is_read: true,
                action_url: Some("/settings/data".to_string()),
                action_label: Some("Download".to_string()),
            },
        ]
    }
}

/// Helper to format counts
pub fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}
