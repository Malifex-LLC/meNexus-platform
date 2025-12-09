// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// Message status for delivery tracking
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

impl MessageStatus {
    pub fn icon_svg(&self) -> &'static str {
        match self {
            Self::Sending => r#"<circle cx="12" cy="12" r="3" fill="currentColor" class="animate-pulse"/>"#,
            Self::Sent => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>"#,
            Self::Delivered => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7M5 7l4 4"/>"#,
            Self::Read => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7M5 7l4 4"/>"#,
            Self::Failed => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>"#,
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Sending => "text-foreground/30",
            Self::Sent => "text-foreground/40",
            Self::Delivered => "text-foreground/50",
            Self::Read => "text-brand",
            Self::Failed => "text-error",
        }
    }
}

/// Type of message content
#[derive(Clone)]
pub enum MessageContent {
    Text(String),
    Image { url: String, caption: Option<String> },
    File { name: String, size: String, file_type: String },
    Voice { duration: String },
    System(String),
    Reply { original_sender: String, original_text: String, reply_text: String },
}

/// Individual message
#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_avatar: Option<String>,
    pub content: MessageContent,
    pub timestamp: String,
    pub status: MessageStatus,
    pub is_own: bool,
    pub reactions: Vec<MessageReaction>,
    pub is_encrypted: bool,
}

/// Reaction on a message
#[derive(Clone)]
pub struct MessageReaction {
    pub emoji: String,
    pub count: u32,
    pub reacted_by_me: bool,
}

/// Participant in a conversation
#[derive(Clone)]
pub struct Participant {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub is_online: bool,
    pub is_verified: bool,
    pub last_seen: Option<String>,
    pub public_key_fingerprint: Option<String>,
}

impl Participant {
    pub fn mock_participants() -> Vec<Self> {
        vec![
            Self {
                id: "user-001".to_string(),
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
                avatar: None,
                is_online: true,
                is_verified: true,
                last_seen: None,
                public_key_fingerprint: Some("A4F2 B8C1 D3E5".to_string()),
            },
            Self {
                id: "user-002".to_string(),
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
                avatar: None,
                is_online: true,
                is_verified: true,
                last_seen: None,
                public_key_fingerprint: Some("B7D3 E9F1 A2C4".to_string()),
            },
            Self {
                id: "user-003".to_string(),
                handle: "neo".to_string(),
                display_name: "Neo".to_string(),
                avatar: None,
                is_online: false,
                is_verified: true,
                last_seen: Some("2 hours ago".to_string()),
                public_key_fingerprint: Some("C1E5 F2A8 B4D6".to_string()),
            },
        ]
    }
}

/// Conversation type
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConversationType {
    Direct,
    Group,
}

/// Conversation/chat thread
#[derive(Clone)]
pub struct Conversation {
    pub id: String,
    pub conversation_type: ConversationType,
    pub name: Option<String>,
    pub participants: Vec<Participant>,
    pub last_message: Option<String>,
    pub last_message_sender: Option<String>,
    pub last_message_time: String,
    pub unread_count: u32,
    pub is_muted: bool,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_encrypted: bool,
}

impl Conversation {
    pub fn display_name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else if self.participants.len() == 1 {
            self.participants[0].display_name.clone()
        } else {
            self.participants
                .iter()
                .map(|p| p.display_name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    pub fn is_online(&self) -> bool {
        match self.conversation_type {
            ConversationType::Direct => self.participants.first().map(|p| p.is_online).unwrap_or(false),
            ConversationType::Group => self.participants.iter().any(|p| p.is_online),
        }
    }

    pub fn online_count(&self) -> usize {
        self.participants.iter().filter(|p| p.is_online).count()
    }

    pub fn mock_conversations() -> Vec<Self> {
        vec![
            Self {
                id: "conv-001".to_string(),
                conversation_type: ConversationType::Direct,
                name: None,
                participants: vec![Participant {
                    id: "user-001".to_string(),
                    handle: "morpheus".to_string(),
                    display_name: "Morpheus".to_string(),
                    avatar: None,
                    is_online: true,
                    is_verified: true,
                    last_seen: None,
                    public_key_fingerprint: Some("A4F2 B8C1 D3E5".to_string()),
                }],
                last_message: Some("The Matrix has you...".to_string()),
                last_message_sender: Some("Morpheus".to_string()),
                last_message_time: "2m".to_string(),
                unread_count: 3,
                is_muted: false,
                is_pinned: true,
                is_archived: false,
                is_encrypted: true,
            },
            Self {
                id: "conv-002".to_string(),
                conversation_type: ConversationType::Direct,
                name: None,
                participants: vec![Participant {
                    id: "user-002".to_string(),
                    handle: "trinity".to_string(),
                    display_name: "Trinity".to_string(),
                    avatar: None,
                    is_online: true,
                    is_verified: true,
                    last_seen: None,
                    public_key_fingerprint: Some("B7D3 E9F1 A2C4".to_string()),
                }],
                last_message: Some("Dodge this.".to_string()),
                last_message_sender: Some("Trinity".to_string()),
                last_message_time: "15m".to_string(),
                unread_count: 0,
                is_muted: false,
                is_pinned: false,
                is_archived: false,
                is_encrypted: true,
            },
            Self {
                id: "conv-003".to_string(),
                conversation_type: ConversationType::Group,
                name: Some("Nebuchadnezzar Crew".to_string()),
                participants: Participant::mock_participants(),
                last_message: Some("Tank: I need an exit!".to_string()),
                last_message_sender: Some("Tank".to_string()),
                last_message_time: "1h".to_string(),
                unread_count: 12,
                is_muted: false,
                is_pinned: true,
                is_archived: false,
                is_encrypted: true,
            },
            Self {
                id: "conv-004".to_string(),
                conversation_type: ConversationType::Direct,
                name: None,
                participants: vec![Participant {
                    id: "user-004".to_string(),
                    handle: "oracle".to_string(),
                    display_name: "The Oracle".to_string(),
                    avatar: None,
                    is_online: false,
                    is_verified: false,
                    last_seen: Some("3 hours ago".to_string()),
                    public_key_fingerprint: Some("D2F6 A1B3 C5E7".to_string()),
                }],
                last_message: Some("You're not here to make a choice...".to_string()),
                last_message_sender: Some("The Oracle".to_string()),
                last_message_time: "3h".to_string(),
                unread_count: 0,
                is_muted: true,
                is_pinned: false,
                is_archived: false,
                is_encrypted: true,
            },
            Self {
                id: "conv-005".to_string(),
                conversation_type: ConversationType::Group,
                name: Some("Zion Council".to_string()),
                participants: vec![
                    Participant {
                        id: "user-005".to_string(),
                        handle: "niobe".to_string(),
                        display_name: "Niobe".to_string(),
                        avatar: None,
                        is_online: false,
                        is_verified: true,
                        last_seen: Some("1 hour ago".to_string()),
                        public_key_fingerprint: None,
                    },
                    Participant {
                        id: "user-006".to_string(),
                        handle: "commander_lock".to_string(),
                        display_name: "Commander Lock".to_string(),
                        avatar: None,
                        is_online: true,
                        is_verified: true,
                        last_seen: None,
                        public_key_fingerprint: None,
                    },
                ],
                last_message: Some("The machines are tunneling...".to_string()),
                last_message_sender: Some("Commander Lock".to_string()),
                last_message_time: "Yesterday".to_string(),
                unread_count: 0,
                is_muted: false,
                is_pinned: false,
                is_archived: false,
                is_encrypted: true,
            },
            Self {
                id: "conv-006".to_string(),
                conversation_type: ConversationType::Direct,
                name: None,
                participants: vec![Participant {
                    id: "user-007".to_string(),
                    handle: "agent_smith".to_string(),
                    display_name: "Agent Smith".to_string(),
                    avatar: None,
                    is_online: true,
                    is_verified: false,
                    last_seen: None,
                    public_key_fingerprint: None,
                }],
                last_message: Some("Mr. Anderson...".to_string()),
                last_message_sender: Some("Agent Smith".to_string()),
                last_message_time: "2d".to_string(),
                unread_count: 1,
                is_muted: true,
                is_pinned: false,
                is_archived: false,
                is_encrypted: false,
            },
        ]
    }
}

impl Message {
    pub fn mock_messages() -> Vec<Self> {
        vec![
            Self {
                id: "msg-001".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("Wake up, Neo...".to_string()),
                timestamp: "10:30 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-002".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("The Matrix has you...".to_string()),
                timestamp: "10:31 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![
                    MessageReaction { emoji: "🔥".to_string(), count: 2, reacted_by_me: true },
                ],
                is_encrypted: true,
            },
            Self {
                id: "msg-003".to_string(),
                sender_id: "me".to_string(),
                sender_name: "Neo".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("What is the Matrix?".to_string()),
                timestamp: "10:32 AM".to_string(),
                status: MessageStatus::Read,
                is_own: true,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-004".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("Unfortunately, no one can be told what the Matrix is. You have to see it for yourself.".to_string()),
                timestamp: "10:33 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![
                    MessageReaction { emoji: "👀".to_string(), count: 1, reacted_by_me: false },
                ],
                is_encrypted: true,
            },
            Self {
                id: "msg-005".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Image { 
                    url: "https://example.com/red-blue-pill.jpg".to_string(),
                    caption: Some("This is your last chance. After this, there is no turning back.".to_string()),
                },
                timestamp: "10:34 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-006".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("You take the blue pill—the story ends, you wake up in your bed and believe whatever you want to believe.".to_string()),
                timestamp: "10:35 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-007".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("You take the red pill—you stay in Wonderland, and I show you how deep the rabbit hole goes.".to_string()),
                timestamp: "10:35 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![
                    MessageReaction { emoji: "💊".to_string(), count: 3, reacted_by_me: true },
                    MessageReaction { emoji: "🐇".to_string(), count: 1, reacted_by_me: false },
                ],
                is_encrypted: true,
            },
            Self {
                id: "msg-008".to_string(),
                sender_id: "me".to_string(),
                sender_name: "Neo".to_string(),
                sender_avatar: None,
                content: MessageContent::Reply {
                    original_sender: "Morpheus".to_string(),
                    original_text: "You take the red pill—you stay in Wonderland...".to_string(),
                    reply_text: "I'll take the red pill.".to_string(),
                },
                timestamp: "10:36 AM".to_string(),
                status: MessageStatus::Read,
                is_own: true,
                reactions: vec![
                    MessageReaction { emoji: "🎉".to_string(), count: 1, reacted_by_me: false },
                ],
                is_encrypted: true,
            },
            Self {
                id: "msg-009".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::System("End-to-end encryption is active. Messages are secured with quantum-resistant cryptography.".to_string()),
                timestamp: "10:36 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-010".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Voice { duration: "0:42".to_string() },
                timestamp: "10:37 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-011".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::File { 
                    name: "construct_blueprints.pdf".to_string(),
                    size: "2.4 MB".to_string(),
                    file_type: "PDF".to_string(),
                },
                timestamp: "10:38 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-012".to_string(),
                sender_id: "me".to_string(),
                sender_name: "Neo".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("I know kung fu.".to_string()),
                timestamp: "10:45 AM".to_string(),
                status: MessageStatus::Delivered,
                is_own: true,
                reactions: vec![],
                is_encrypted: true,
            },
            Self {
                id: "msg-013".to_string(),
                sender_id: "user-001".to_string(),
                sender_name: "Morpheus".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("Show me.".to_string()),
                timestamp: "10:45 AM".to_string(),
                status: MessageStatus::Read,
                is_own: false,
                reactions: vec![
                    MessageReaction { emoji: "⚔️".to_string(), count: 2, reacted_by_me: false },
                ],
                is_encrypted: true,
            },
            Self {
                id: "msg-014".to_string(),
                sender_id: "me".to_string(),
                sender_name: "Neo".to_string(),
                sender_avatar: None,
                content: MessageContent::Text("What are you trying to tell me? That I can dodge bullets?".to_string()),
                timestamp: "Just now".to_string(),
                status: MessageStatus::Sending,
                is_own: true,
                reactions: vec![],
                is_encrypted: true,
            },
        ]
    }
}

/// Filter for conversation list
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ConversationFilter {
    #[default]
    All,
    Unread,
    Direct,
    Groups,
    Archived,
}

impl ConversationFilter {
    pub fn all() -> Vec<Self> {
        vec![Self::All, Self::Unread, Self::Direct, Self::Groups, Self::Archived]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Unread => "Unread",
            Self::Direct => "Direct",
            Self::Groups => "Groups",
            Self::Archived => "Archived",
        }
    }
}

/// Format timestamp for display
pub fn format_message_time(timestamp: &str) -> String {
    timestamp.to_string()
}
