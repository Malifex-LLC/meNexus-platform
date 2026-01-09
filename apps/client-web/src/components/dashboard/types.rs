// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// A Synapse with its channels for the compose dropdown
#[derive(Clone, Debug)]
pub struct SynapseWithChannels {
    pub id: String,
    pub name: String,
    pub icon_url: Option<String>,
    pub channels: Vec<Channel>,
}

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

impl SynapseWithChannels {
    pub fn mock_synapses() -> Vec<SynapseWithChannels> {
        vec![
            SynapseWithChannels {
                id: "syn-1".to_string(),
                name: "Zion HQ".to_string(),
                icon_url: None,
                channels: vec![
                    Channel { id: "ch-1".to_string(), name: "general".to_string(), is_default: true },
                    Channel { id: "ch-2".to_string(), name: "announcements".to_string(), is_default: false },
                    Channel { id: "ch-3".to_string(), name: "random".to_string(), is_default: false },
                ],
            },
            SynapseWithChannels {
                id: "syn-2".to_string(),
                name: "Nebuchadnezzar".to_string(),
                icon_url: None,
                channels: vec![
                    Channel { id: "ch-4".to_string(), name: "crew".to_string(), is_default: true },
                    Channel { id: "ch-5".to_string(), name: "operations".to_string(), is_default: false },
                ],
            },
            SynapseWithChannels {
                id: "syn-3".to_string(),
                name: "Oracle's Temple".to_string(),
                icon_url: None,
                channels: vec![
                    Channel { id: "ch-6".to_string(), name: "prophecies".to_string(), is_default: true },
                    Channel { id: "ch-7".to_string(), name: "guidance".to_string(), is_default: false },
                ],
            },
            SynapseWithChannels {
                id: "syn-4".to_string(),
                name: "Rust Devs".to_string(),
                icon_url: None,
                channels: vec![
                    Channel { id: "ch-8".to_string(), name: "general".to_string(), is_default: true },
                    Channel { id: "ch-9".to_string(), name: "help".to_string(), is_default: false },
                    Channel { id: "ch-10".to_string(), name: "showcase".to_string(), is_default: false },
                ],
            },
        ]
    }
}

/// Filter content type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentType {
    All,
    Posts,
    Media,
    Links,
    Polls,
}

impl ContentType {
    pub fn label(&self) -> &'static str {
        match self {
            ContentType::All => "All",
            ContentType::Posts => "Posts",
            ContentType::Media => "Media",
            ContentType::Links => "Links",
            ContentType::Polls => "Polls",
        }
    }
}

/// Time range filter
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeRange {
    LastHour,
    Today,
    ThisWeek,
    ThisMonth,
    AllTime,
}

impl TimeRange {
    pub fn label(&self) -> &'static str {
        match self {
            TimeRange::LastHour => "Last hour",
            TimeRange::Today => "Today",
            TimeRange::ThisWeek => "This week",
            TimeRange::ThisMonth => "This month",
            TimeRange::AllTime => "All time",
        }
    }
}

/// Sort order
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SortOrder {
    Latest,
    Trending,
    MostLiked,
    MostCommented,
    Following,
}

impl SortOrder {
    pub fn label(&self) -> &'static str {
        match self {
            SortOrder::Latest => "Latest",
            SortOrder::Trending => "Trending",
            SortOrder::MostLiked => "Most liked",
            SortOrder::MostCommented => "Most discussed",
            SortOrder::Following => "Following",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            SortOrder::Latest => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#,
            SortOrder::Trending => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>"#,
            SortOrder::MostLiked => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>"#,
            SortOrder::MostCommented => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/></svg>"#,
            SortOrder::Following => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/></svg>"#,
        }
    }
}

/// Feed filter state
#[derive(Clone, Debug)]
pub struct FeedFilters {
    pub sort_order: SortOrder,
    pub content_type: ContentType,
    pub time_range: TimeRange,
    pub synapse_filter: Option<String>, // None = all synapses
    pub hide_seen: bool,
    pub only_verified: bool,
    pub min_reputation: Option<u32>,
}

impl Default for FeedFilters {
    fn default() -> Self {
        Self {
            sort_order: SortOrder::Latest,
            content_type: ContentType::All,
            time_range: TimeRange::AllTime,
            synapse_filter: None,
            hide_seen: false,
            only_verified: false,
            min_reputation: None,
        }
    }
}

/// A global post from any Synapse
#[derive(Clone, Debug)]
pub struct GlobalPost {
    pub id: String,
    pub author_handle: String,
    pub author_name: String,
    pub author_avatar: Option<String>,
    pub author_verified: bool,
    pub author_reputation: u32,
    pub synapse_id: String,
    pub synapse_name: String,
    pub channel_name: String,
    pub content: String,
    pub timestamp: String,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub is_liked: bool,
    pub is_bookmarked: bool,
    pub has_media: bool,
}

impl GlobalPost {
    pub fn mock_posts() -> Vec<GlobalPost> {
        vec![
            GlobalPost {
                id: "gp-1".to_string(),
                author_handle: "neo".to_string(),
                author_name: "Neo Anderson".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 1247,
                synapse_id: "syn-1".to_string(),
                synapse_name: "Zion HQ".to_string(),
                channel_name: "general".to_string(),
                content: "Just deployed a major update to the P2P sync protocol. Latency is down 40% and we're seeing much better consistency across nodes. The local-first future is here! 🚀".to_string(),
                timestamp: "5 min ago".to_string(),
                likes: 42,
                comments: 8,
                shares: 12,
                is_liked: false,
                is_bookmarked: false,
                has_media: false,
            },
            GlobalPost {
                id: "gp-2".to_string(),
                author_handle: "trinity".to_string(),
                author_name: "Trinity".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 2100,
                synapse_id: "syn-2".to_string(),
                synapse_name: "Nebuchadnezzar".to_string(),
                channel_name: "operations".to_string(),
                content: "Mission briefing at 1800 UTC. All operators check your gear. This one's going to be interesting.".to_string(),
                timestamp: "12 min ago".to_string(),
                likes: 18,
                comments: 5,
                shares: 2,
                is_liked: true,
                is_bookmarked: true,
                has_media: false,
            },
            GlobalPost {
                id: "gp-3".to_string(),
                author_handle: "morpheus".to_string(),
                author_name: "Morpheus".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 3200,
                synapse_id: "syn-3".to_string(),
                synapse_name: "Oracle's Temple".to_string(),
                channel_name: "prophecies".to_string(),
                content: "\"You take the blue pill, the story ends. You take the red pill, you stay in Wonderland, and I show you how deep the rabbit hole goes.\"\n\nThe choice has always been yours. Choose wisely.".to_string(),
                timestamp: "28 min ago".to_string(),
                likes: 156,
                comments: 34,
                shares: 45,
                is_liked: false,
                is_bookmarked: false,
                has_media: false,
            },
            GlobalPost {
                id: "gp-4".to_string(),
                author_handle: "tank".to_string(),
                author_name: "Tank".to_string(),
                author_avatar: None,
                author_verified: false,
                author_reputation: 890,
                synapse_id: "syn-4".to_string(),
                synapse_name: "Rust Devs".to_string(),
                channel_name: "help".to_string(),
                content: "Anyone else fighting the borrow checker today? I've been staring at this lifetime error for an hour. Send help (and coffee). ☕\n\n```rust\nfn process<'a>(&'a self) -> &'a str\n```".to_string(),
                timestamp: "45 min ago".to_string(),
                likes: 67,
                comments: 23,
                shares: 8,
                is_liked: true,
                is_bookmarked: false,
                has_media: false,
            },
            GlobalPost {
                id: "gp-5".to_string(),
                author_handle: "oracle".to_string(),
                author_name: "The Oracle".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 4500,
                synapse_id: "syn-3".to_string(),
                synapse_name: "Oracle's Temple".to_string(),
                channel_name: "guidance".to_string(),
                content: "Here, take a cookie. I promise, by the time you're done eating it, you'll feel right as rain. 🍪\n\nRemember: knowing the path and walking the path are two different things.".to_string(),
                timestamp: "1 hour ago".to_string(),
                likes: 234,
                comments: 56,
                shares: 78,
                is_liked: false,
                is_bookmarked: true,
                has_media: false,
            },
            GlobalPost {
                id: "gp-6".to_string(),
                author_handle: "niobe".to_string(),
                author_name: "Niobe".to_string(),
                author_avatar: None,
                author_verified: false,
                author_reputation: 1100,
                synapse_id: "syn-2".to_string(),
                synapse_name: "Nebuchadnezzar".to_string(),
                channel_name: "crew".to_string(),
                content: "Just finished upgrading the Logos' navigation systems. The new quantum entanglement relay is going to change everything. Can't wait to test it in the tunnels.".to_string(),
                timestamp: "1.5 hours ago".to_string(),
                likes: 29,
                comments: 11,
                shares: 4,
                is_liked: false,
                is_bookmarked: false,
                has_media: true,
            },
            GlobalPost {
                id: "gp-7".to_string(),
                author_handle: "seraph".to_string(),
                author_name: "Seraph".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 1800,
                synapse_id: "syn-3".to_string(),
                synapse_name: "Oracle's Temple".to_string(),
                channel_name: "general".to_string(),
                content: "You do not truly know someone until you fight them.\n\nNew security protocols are now in effect. All visitors must authenticate through the new verification system.".to_string(),
                timestamp: "2 hours ago".to_string(),
                likes: 45,
                comments: 12,
                shares: 6,
                is_liked: false,
                is_bookmarked: false,
                has_media: false,
            },
            GlobalPost {
                id: "gp-8".to_string(),
                author_handle: "keymaker".to_string(),
                author_name: "The Keymaker".to_string(),
                author_avatar: None,
                author_verified: false,
                author_reputation: 750,
                synapse_id: "syn-1".to_string(),
                synapse_name: "Zion HQ".to_string(),
                channel_name: "announcements".to_string(),
                content: "There is always another way. The door is never truly locked if you know where to look.\n\nNew key rotation schedule posted. Check your credentials.".to_string(),
                timestamp: "3 hours ago".to_string(),
                likes: 89,
                comments: 7,
                shares: 15,
                is_liked: true,
                is_bookmarked: false,
                has_media: false,
            },
        ]
    }
}

/// Trending topic
#[derive(Clone, Debug)]
pub struct TrendingTopic {
    pub tag: String,
    pub post_count: u32,
    pub trend_direction: TrendDirection,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TrendDirection {
    Up,
    Down,
    Stable,
}

impl TrendingTopic {
    pub fn mock_topics() -> Vec<TrendingTopic> {
        vec![
            TrendingTopic { tag: "LocalFirst".to_string(), post_count: 234, trend_direction: TrendDirection::Up },
            TrendingTopic { tag: "RustLang".to_string(), post_count: 189, trend_direction: TrendDirection::Up },
            TrendingTopic { tag: "P2P".to_string(), post_count: 156, trend_direction: TrendDirection::Stable },
            TrendingTopic { tag: "Decentralized".to_string(), post_count: 134, trend_direction: TrendDirection::Up },
            TrendingTopic { tag: "CRDT".to_string(), post_count: 98, trend_direction: TrendDirection::Down },
        ]
    }
}

/// Trending user
#[derive(Clone, Debug)]
pub struct TrendingUser {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub reputation: u32,
    pub is_verified: bool,
    pub follower_growth: i32, // can be negative
}

impl TrendingUser {
    pub fn mock_users() -> Vec<TrendingUser> {
        vec![
            TrendingUser { handle: "oracle".to_string(), display_name: "The Oracle".to_string(), avatar_url: None, reputation: 4500, is_verified: true, follower_growth: 156 },
            TrendingUser { handle: "morpheus".to_string(), display_name: "Morpheus".to_string(), avatar_url: None, reputation: 3200, is_verified: true, follower_growth: 89 },
            TrendingUser { handle: "trinity".to_string(), display_name: "Trinity".to_string(), avatar_url: None, reputation: 2100, is_verified: true, follower_growth: 67 },
        ]
    }
}

/// Active Synapse for the sidebar
#[derive(Clone, Debug)]
pub struct ActiveSynapse {
    pub id: String,
    pub name: String,
    pub icon_url: Option<String>,
    pub online_count: u32,
    pub new_posts: u32,
}

impl ActiveSynapse {
    pub fn mock_synapses() -> Vec<ActiveSynapse> {
        vec![
            ActiveSynapse { id: "syn-1".to_string(), name: "Zion HQ".to_string(), icon_url: None, online_count: 234, new_posts: 12 },
            ActiveSynapse { id: "syn-3".to_string(), name: "Oracle's Temple".to_string(), icon_url: None, online_count: 156, new_posts: 8 },
            ActiveSynapse { id: "syn-4".to_string(), name: "Rust Devs".to_string(), icon_url: None, online_count: 89, new_posts: 23 },
            ActiveSynapse { id: "syn-2".to_string(), name: "Nebuchadnezzar".to_string(), icon_url: None, online_count: 12, new_posts: 3 },
        ]
    }
}

/// Network statistics
#[derive(Clone, Debug)]
pub struct NetworkStats {
    pub connected_peers: u32,
    pub total_synapses: u32,
    pub online_users: u32,
    pub posts_today: u32,
    pub latency_ms: u32,
    pub sync_status: SyncStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SyncStatus {
    Synced,
    Syncing,
    Offline,
}

impl NetworkStats {
    pub fn mock() -> Self {
        Self {
            connected_peers: 24,
            total_synapses: 4,
            online_users: 491,
            posts_today: 1_247,
            latency_ms: 24,
            sync_status: SyncStatus::Synced,
        }
    }
}
