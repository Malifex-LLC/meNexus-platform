// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};

/// Represents a chat channel/room
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub member_count: u32,
    pub online_count: u32,
    pub unread_count: u32,
    pub last_message_preview: Option<String>,
    pub last_message_time: Option<String>,
}

/// Represents a user in the chat
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatUser {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_online: bool,
    pub role: UserRole,
}

/// User roles in a chat room
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Owner,
    Admin,
    Moderator,
    Member,
}

/// A single chat message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub room_id: String,
    pub author: ChatUser,
    pub content: String,
    pub timestamp: String,
    pub is_edited: bool,
    pub reply_to: Option<Box<ChatMessage>>,
    pub reactions: Vec<Reaction>,
    pub attachments: Vec<Attachment>,
    pub message_type: MessageType,
}

/// Reaction on a message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reaction {
    pub emoji: String,
    pub count: u32,
    pub reacted_by_me: bool,
}

/// File attachment
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub file_name: String,
    pub file_type: AttachmentType,
    pub url: String,
    pub size_bytes: u64,
}

/// Types of attachments
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AttachmentType {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Other,
}

/// Types of messages
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Normal,
    System,    // User joined, left, etc.
    Pinned,
}

/// Typing indicator data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypingUser {
    pub handle: String,
    pub display_name: String,
}

// ============================================================================
// Faux Data Generators
// ============================================================================

impl ChatRoom {
    /// Generate mock chat rooms for UI development
    pub fn mock_rooms() -> Vec<ChatRoom> {
        vec![
            ChatRoom {
                id: "room-1".to_string(),
                name: "general".to_string(),
                description: Some("General discussion for everyone".to_string()),
                is_private: false,
                member_count: 128,
                online_count: 23,
                unread_count: 5,
                last_message_preview: Some("neo: Has anyone seen the latest update?".to_string()),
                last_message_time: Some("2m ago".to_string()),
            },
            ChatRoom {
                id: "room-2".to_string(),
                name: "dev-core".to_string(),
                description: Some("Core development discussions".to_string()),
                is_private: false,
                member_count: 45,
                online_count: 12,
                unread_count: 0,
                last_message_preview: Some("trinity: PR merged, rebasing now".to_string()),
                last_message_time: Some("15m ago".to_string()),
            },
            ChatRoom {
                id: "room-3".to_string(),
                name: "random".to_string(),
                description: Some("Off-topic chaos".to_string()),
                is_private: false,
                member_count: 89,
                online_count: 8,
                unread_count: 42,
                last_message_preview: Some("morpheus: 🔥🔥🔥".to_string()),
                last_message_time: Some("Just now".to_string()),
            },
            ChatRoom {
                id: "room-4".to_string(),
                name: "security".to_string(),
                description: Some("Security team private channel".to_string()),
                is_private: true,
                member_count: 8,
                online_count: 3,
                unread_count: 2,
                last_message_preview: Some("cypher: New CVE detected...".to_string()),
                last_message_time: Some("1h ago".to_string()),
            },
            ChatRoom {
                id: "room-5".to_string(),
                name: "announcements".to_string(),
                description: Some("Official announcements".to_string()),
                is_private: false,
                member_count: 256,
                online_count: 45,
                unread_count: 1,
                last_message_preview: Some("System: v0.3.0-alpha released!".to_string()),
                last_message_time: Some("3h ago".to_string()),
            },
        ]
    }
}

impl ChatUser {
    /// Generate mock users
    pub fn mock_users() -> Vec<ChatUser> {
        vec![
            ChatUser {
                id: "user-1".to_string(),
                handle: "neo".to_string(),
                display_name: "Neo Anderson".to_string(),
                avatar_url: None,
                is_online: true,
                role: UserRole::Owner,
            },
            ChatUser {
                id: "user-2".to_string(),
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
                avatar_url: None,
                is_online: true,
                role: UserRole::Admin,
            },
            ChatUser {
                id: "user-3".to_string(),
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
                avatar_url: None,
                is_online: true,
                role: UserRole::Moderator,
            },
            ChatUser {
                id: "user-4".to_string(),
                handle: "cypher".to_string(),
                display_name: "Cypher Reagan".to_string(),
                avatar_url: None,
                is_online: false,
                role: UserRole::Member,
            },
            ChatUser {
                id: "user-5".to_string(),
                handle: "oracle".to_string(),
                display_name: "The Oracle".to_string(),
                avatar_url: None,
                is_online: true,
                role: UserRole::Member,
            },
        ]
    }

    pub fn mock_me() -> ChatUser {
        ChatUser {
            id: "user-me".to_string(),
            handle: "neo".to_string(),
            display_name: "Neo Anderson".to_string(),
            avatar_url: None,
            is_online: true,
            role: UserRole::Owner,
        }
    }
}

impl ChatMessage {
    /// Generate mock messages for a room
    pub fn mock_messages() -> Vec<ChatMessage> {
        let users = ChatUser::mock_users();
        
        vec![
            ChatMessage {
                id: "msg-1".to_string(),
                room_id: "room-1".to_string(),
                author: users[2].clone(), // morpheus
                content: "Welcome to the construct. This is a loading program.".to_string(),
                timestamp: "10:30 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![
                    Reaction { emoji: "👍".to_string(), count: 3, reacted_by_me: false },
                    Reaction { emoji: "🔥".to_string(), count: 1, reacted_by_me: true },
                ],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-2".to_string(),
                room_id: "room-1".to_string(),
                author: users[0].clone(), // neo
                content: "Can we load anything into this program?".to_string(),
                timestamp: "10:31 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-3".to_string(),
                room_id: "room-1".to_string(),
                author: users[2].clone(), // morpheus
                content: "Anything we need. We're running P2P decentralized infrastructure. Each synapse node contributes to the network. It's completely distributed - no central servers, no single point of failure.".to_string(),
                timestamp: "10:32 AM".to_string(),
                is_edited: true,
                reply_to: None,
                reactions: vec![
                    Reaction { emoji: "🧠".to_string(), count: 5, reacted_by_me: false },
                ],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-4".to_string(),
                room_id: "room-1".to_string(),
                author: users[1].clone(), // trinity
                content: "The latency stats from the new protocol look promising. Check this out:".to_string(),
                timestamp: "10:35 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![],
                attachments: vec![
                    Attachment {
                        id: "att-1".to_string(),
                        file_name: "latency_benchmark.png".to_string(),
                        file_type: AttachmentType::Image,
                        url: "/attachments/latency_benchmark.png".to_string(),
                        size_bytes: 245760,
                    }
                ],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-5".to_string(),
                room_id: "room-1".to_string(),
                author: ChatUser {
                    id: "system".to_string(),
                    handle: "system".to_string(),
                    display_name: "System".to_string(),
                    avatar_url: None,
                    is_online: true,
                    role: UserRole::Member,
                },
                content: "oracle joined the channel".to_string(),
                timestamp: "10:40 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![],
                attachments: vec![],
                message_type: MessageType::System,
            },
            ChatMessage {
                id: "msg-6".to_string(),
                room_id: "room-1".to_string(),
                author: users[4].clone(), // oracle
                content: "I've been expecting you. 🍪".to_string(),
                timestamp: "10:41 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![
                    Reaction { emoji: "😂".to_string(), count: 8, reacted_by_me: true },
                    Reaction { emoji: "🍪".to_string(), count: 12, reacted_by_me: false },
                ],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-7".to_string(),
                room_id: "room-1".to_string(),
                author: users[0].clone(), // neo
                content: "Perfect timing! We were just discussing the new sync protocol.".to_string(),
                timestamp: "10:42 AM".to_string(),
                is_edited: false,
                reply_to: Some(Box::new(ChatMessage {
                    id: "msg-6".to_string(),
                    room_id: "room-1".to_string(),
                    author: users[4].clone(),
                    content: "I've been expecting you. 🍪".to_string(),
                    timestamp: "10:41 AM".to_string(),
                    is_edited: false,
                    reply_to: None,
                    reactions: vec![],
                    attachments: vec![],
                    message_type: MessageType::Normal,
                })),
                reactions: vec![],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-8".to_string(),
                room_id: "room-1".to_string(),
                author: users[1].clone(), // trinity
                content: "```rust\n// New sync implementation\nasync fn sync_documents(\n    local: &DocStore,\n    remote: &Peer,\n) -> Result<SyncState, SyncError> {\n    let heads = local.get_heads().await?;\n    let changes = remote.get_changes_since(heads).await?;\n    local.apply_changes(changes).await\n}\n```".to_string(),
                timestamp: "10:45 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![
                    Reaction { emoji: "👀".to_string(), count: 3, reacted_by_me: false },
                ],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-9".to_string(),
                room_id: "room-1".to_string(),
                author: users[2].clone(), // morpheus
                content: "Clean implementation. The error handling looks solid. 💪".to_string(),
                timestamp: "10:47 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
            ChatMessage {
                id: "msg-10".to_string(),
                room_id: "room-1".to_string(),
                author: users[0].clone(), // neo
                content: "Has anyone seen the latest update from the mainnet?".to_string(),
                timestamp: "10:52 AM".to_string(),
                is_edited: false,
                reply_to: None,
                reactions: vec![],
                attachments: vec![],
                message_type: MessageType::Normal,
            },
        ]
    }
}

impl TypingUser {
    pub fn mock_typing() -> Vec<TypingUser> {
        vec![
            TypingUser {
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
            },
            TypingUser {
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
            },
        ]
    }
}

