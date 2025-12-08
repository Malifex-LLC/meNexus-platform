// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// Search category for filtering results
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchCategory {
    All,
    Users,
    Synapses,
    Posts,
    Tags,
}

impl SearchCategory {
    pub fn label(&self) -> &'static str {
        match self {
            SearchCategory::All => "All",
            SearchCategory::Users => "Users",
            SearchCategory::Synapses => "Synapses",
            SearchCategory::Posts => "Posts",
            SearchCategory::Tags => "Tags",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            SearchCategory::All => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>"#,
            SearchCategory::Users => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/>"#,
            SearchCategory::Synapses => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>"#,
            SearchCategory::Posts => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M19 20H5a2 2 0 01-2-2V6a2 2 0 012-2h10a2 2 0 012 2v1m2 13a2 2 0 01-2-2V7m2 13a2 2 0 002-2V9a2 2 0 00-2-2h-2m-4-3H9M7 16h6M7 8h6v4H7V8z"/>"#,
            SearchCategory::Tags => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"/>"#,
        }
    }

    pub fn all() -> Vec<SearchCategory> {
        vec![
            SearchCategory::All,
            SearchCategory::Users,
            SearchCategory::Synapses,
            SearchCategory::Posts,
            SearchCategory::Tags,
        ]
    }
}

/// Search scope - where to search
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchScope {
    MyNetwork, // Only joined synapses
    Global,    // Entire network
}

impl SearchScope {
    pub fn label(&self) -> &'static str {
        match self {
            SearchScope::MyNetwork => "My Network",
            SearchScope::Global => "Global",
        }
    }
}

/// Time range filter for posts
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchTimeRange {
    AnyTime,
    PastHour,
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
}

impl SearchTimeRange {
    pub fn label(&self) -> &'static str {
        match self {
            SearchTimeRange::AnyTime => "Any time",
            SearchTimeRange::PastHour => "Past hour",
            SearchTimeRange::Today => "Today",
            SearchTimeRange::ThisWeek => "This week",
            SearchTimeRange::ThisMonth => "This month",
            SearchTimeRange::ThisYear => "This year",
        }
    }

    pub fn all() -> Vec<SearchTimeRange> {
        vec![
            SearchTimeRange::AnyTime,
            SearchTimeRange::PastHour,
            SearchTimeRange::Today,
            SearchTimeRange::ThisWeek,
            SearchTimeRange::ThisMonth,
            SearchTimeRange::ThisYear,
        ]
    }
}

/// Sort options for search results
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchSortBy {
    Relevance,
    Recent,
    Popular,
    Reputation,
}

impl SearchSortBy {
    pub fn label(&self) -> &'static str {
        match self {
            SearchSortBy::Relevance => "Most relevant",
            SearchSortBy::Recent => "Most recent",
            SearchSortBy::Popular => "Most popular",
            SearchSortBy::Reputation => "Highest reputation",
        }
    }

    pub fn all() -> Vec<SearchSortBy> {
        vec![
            SearchSortBy::Relevance,
            SearchSortBy::Recent,
            SearchSortBy::Popular,
            SearchSortBy::Reputation,
        ]
    }
}

/// User search result
#[derive(Clone, Debug)]
pub struct UserResult {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub reputation: u32,
    pub is_verified: bool,
    pub follower_count: u32,
    pub following_count: u32,
    pub mutual_connections: u32,
    pub is_following: bool,
    pub joined_date: String,
}

impl UserResult {
    pub fn mock_results(query: &str) -> Vec<UserResult> {
        vec![
            UserResult {
                handle: "neo".to_string(),
                display_name: "Neo Anderson".to_string(),
                avatar_url: None,
                bio: Some("The One. Building the future of decentralized networks.".to_string()),
                reputation: 4200,
                is_verified: true,
                follower_count: 12450,
                following_count: 234,
                mutual_connections: 8,
                is_following: false,
                joined_date: "March 2024".to_string(),
            },
            UserResult {
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
                avatar_url: None,
                bio: Some("Operator & pilot. Expert in infiltration systems.".to_string()),
                reputation: 3800,
                is_verified: true,
                follower_count: 8920,
                following_count: 156,
                mutual_connections: 12,
                is_following: true,
                joined_date: "February 2024".to_string(),
            },
            UserResult {
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
                avatar_url: None,
                bio: Some("Captain of the Nebuchadnezzar. Believer in the prophecy.".to_string()),
                reputation: 5100,
                is_verified: true,
                follower_count: 23400,
                following_count: 89,
                mutual_connections: 15,
                is_following: true,
                joined_date: "January 2024".to_string(),
            },
            UserResult {
                handle: "oracle".to_string(),
                display_name: "The Oracle".to_string(),
                avatar_url: None,
                bio: Some("I'm not here to tell you how this is going to end. I'm here to tell you how it's going to begin.".to_string()),
                reputation: 8900,
                is_verified: true,
                follower_count: 45600,
                following_count: 12,
                mutual_connections: 6,
                is_following: false,
                joined_date: "December 2023".to_string(),
            },
            UserResult {
                handle: format!("{}_user", query.to_lowercase()),
                display_name: format!("{} Explorer", query),
                avatar_url: None,
                bio: Some("Exploring the network one node at a time.".to_string()),
                reputation: 890,
                is_verified: false,
                follower_count: 234,
                following_count: 567,
                mutual_connections: 3,
                is_following: false,
                joined_date: "November 2024".to_string(),
            },
        ]
    }
}

/// Synapse search result
#[derive(Clone, Debug)]
pub struct SynapseResult {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub member_count: u32,
    pub online_count: u32,
    pub post_count: u32,
    pub is_joined: bool,
    pub is_verified: bool,
    pub category: String,
    pub tags: Vec<String>,
    pub created_date: String,
}

impl SynapseResult {
    pub fn mock_results(_query: &str) -> Vec<SynapseResult> {
        vec![
            SynapseResult {
                id: "syn-1".to_string(),
                name: "Zion HQ".to_string(),
                description: "The last human city. Central hub for resistance operations and coordination across the network.".to_string(),
                icon_url: None,
                banner_url: None,
                member_count: 12450,
                online_count: 1234,
                post_count: 89500,
                is_joined: true,
                is_verified: true,
                category: "Community".to_string(),
                tags: vec!["official".to_string(), "community".to_string(), "headquarters".to_string()],
                created_date: "January 2024".to_string(),
            },
            SynapseResult {
                id: "syn-2".to_string(),
                name: "Rust Developers".to_string(),
                description: "A community for Rust programmers. Share code, get help, and discuss the language we love.".to_string(),
                icon_url: None,
                banner_url: None,
                member_count: 8900,
                online_count: 456,
                post_count: 45600,
                is_joined: false,
                is_verified: true,
                category: "Technology".to_string(),
                tags: vec!["rust".to_string(), "programming".to_string(), "systems".to_string()],
                created_date: "February 2024".to_string(),
            },
            SynapseResult {
                id: "syn-3".to_string(),
                name: "Oracle's Temple".to_string(),
                description: "Seek guidance and wisdom. A place for reflection, discussion, and discovering your path.".to_string(),
                icon_url: None,
                banner_url: None,
                member_count: 5600,
                online_count: 234,
                post_count: 23400,
                is_joined: true,
                is_verified: true,
                category: "Philosophy".to_string(),
                tags: vec!["wisdom".to_string(), "guidance".to_string(), "philosophy".to_string()],
                created_date: "March 2024".to_string(),
            },
            SynapseResult {
                id: "syn-4".to_string(),
                name: "P2P Protocol Labs".to_string(),
                description: "Research and development of peer-to-peer networking protocols. Building the decentralized future.".to_string(),
                icon_url: None,
                banner_url: None,
                member_count: 3400,
                online_count: 189,
                post_count: 12300,
                is_joined: false,
                is_verified: false,
                category: "Technology".to_string(),
                tags: vec!["p2p".to_string(), "protocols".to_string(), "research".to_string()],
                created_date: "April 2024".to_string(),
            },
            SynapseResult {
                id: "syn-5".to_string(),
                name: "Local-First Builders".to_string(),
                description: "Building software that works offline-first. CRDTs, sync engines, and collaborative systems.".to_string(),
                icon_url: None,
                banner_url: None,
                member_count: 2100,
                online_count: 98,
                post_count: 8900,
                is_joined: false,
                is_verified: false,
                category: "Technology".to_string(),
                tags: vec!["local-first".to_string(), "crdt".to_string(), "sync".to_string()],
                created_date: "May 2024".to_string(),
            },
        ]
    }
}

/// Post search result
#[derive(Clone, Debug)]
pub struct PostResult {
    pub id: String,
    pub author_handle: String,
    pub author_name: String,
    pub author_avatar: Option<String>,
    pub author_verified: bool,
    pub author_reputation: u32,
    pub synapse_id: String,
    pub synapse_name: String,
    pub content: String,
    pub highlight_snippet: String, // Content with search term highlighted
    pub timestamp: String,
    pub likes: u32,
    pub comments: u32,
    pub shares: u32,
    pub has_media: bool,
}

impl PostResult {
    pub fn mock_results(query: &str) -> Vec<PostResult> {
        let query_lower = query.to_lowercase();
        vec![
            PostResult {
                id: "post-1".to_string(),
                author_handle: "neo".to_string(),
                author_name: "Neo Anderson".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 4200,
                synapse_id: "syn-1".to_string(),
                synapse_name: "Zion HQ".to_string(),
                content: format!("Just deployed a major update related to {}. The network is stronger than ever.", query),
                highlight_snippet: format!("Just deployed a major update related to <mark>{}</mark>. The network is stronger...", query),
                timestamp: "2 hours ago".to_string(),
                likes: 156,
                comments: 34,
                shares: 23,
                has_media: false,
            },
            PostResult {
                id: "post-2".to_string(),
                author_handle: "trinity".to_string(),
                author_name: "Trinity".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 3800,
                synapse_id: "syn-2".to_string(),
                synapse_name: "Rust Developers".to_string(),
                content: format!("Been exploring {} implementations in Rust. The type system makes everything so much safer.", query),
                highlight_snippet: format!("Been exploring <mark>{}</mark> implementations in Rust. The type system makes...", query),
                timestamp: "5 hours ago".to_string(),
                likes: 89,
                comments: 21,
                shares: 12,
                has_media: false,
            },
            PostResult {
                id: "post-3".to_string(),
                author_handle: "morpheus".to_string(),
                author_name: "Morpheus".to_string(),
                author_avatar: None,
                author_verified: true,
                author_reputation: 5100,
                synapse_id: "syn-3".to_string(),
                synapse_name: "Oracle's Temple".to_string(),
                content: format!("The path to understanding {} begins with questioning everything you think you know.", query),
                highlight_snippet: format!("The path to understanding <mark>{}</mark> begins with questioning everything...", query),
                timestamp: "1 day ago".to_string(),
                likes: 234,
                comments: 56,
                shares: 45,
                has_media: false,
            },
            PostResult {
                id: "post-4".to_string(),
                author_handle: "tank".to_string(),
                author_name: "Tank".to_string(),
                author_avatar: None,
                author_verified: false,
                author_reputation: 1200,
                synapse_id: "syn-4".to_string(),
                synapse_name: "P2P Protocol Labs".to_string(),
                content: format!("New research paper on {} just dropped. This could change how we think about distributed systems.", query),
                highlight_snippet: format!("New research paper on <mark>{}</mark> just dropped. This could change...", query),
                timestamp: "2 days ago".to_string(),
                likes: 67,
                comments: 18,
                shares: 9,
                has_media: true,
            },
        ]
    }
}

/// Tag search result
#[derive(Clone, Debug)]
pub struct TagResult {
    pub tag: String,
    pub post_count: u32,
    pub follower_count: u32,
    pub trending_rank: Option<u32>,
    pub growth_percentage: i32,
    pub top_synapses: Vec<String>,
    pub is_following: bool,
}

impl TagResult {
    pub fn mock_results(query: &str) -> Vec<TagResult> {
        vec![
            TagResult {
                tag: query.to_string(),
                post_count: 4567,
                follower_count: 2340,
                trending_rank: Some(3),
                growth_percentage: 24,
                top_synapses: vec!["Zion HQ".to_string(), "Rust Developers".to_string()],
                is_following: false,
            },
            TagResult {
                tag: format!("{}Dev", query),
                post_count: 2345,
                follower_count: 1200,
                trending_rank: Some(12),
                growth_percentage: 15,
                top_synapses: vec!["P2P Protocol Labs".to_string()],
                is_following: true,
            },
            TagResult {
                tag: format!("{}Network", query),
                post_count: 1890,
                follower_count: 890,
                trending_rank: None,
                growth_percentage: -5,
                top_synapses: vec!["Local-First Builders".to_string()],
                is_following: false,
            },
            TagResult {
                tag: "LocalFirst".to_string(),
                post_count: 8900,
                follower_count: 4500,
                trending_rank: Some(1),
                growth_percentage: 45,
                top_synapses: vec!["Zion HQ".to_string(), "Local-First Builders".to_string()],
                is_following: true,
            },
            TagResult {
                tag: "RustLang".to_string(),
                post_count: 12300,
                follower_count: 6700,
                trending_rank: Some(2),
                growth_percentage: 32,
                top_synapses: vec!["Rust Developers".to_string()],
                is_following: true,
            },
        ]
    }
}

/// Recent search entry
#[derive(Clone, Debug)]
pub struct RecentSearch {
    pub query: String,
    pub category: SearchCategory,
    pub timestamp: String,
}

impl RecentSearch {
    pub fn mock_recent() -> Vec<RecentSearch> {
        vec![
            RecentSearch { query: "rust async".to_string(), category: SearchCategory::Posts, timestamp: "2 hours ago".to_string() },
            RecentSearch { query: "neo".to_string(), category: SearchCategory::Users, timestamp: "Yesterday".to_string() },
            RecentSearch { query: "P2P".to_string(), category: SearchCategory::Tags, timestamp: "2 days ago".to_string() },
            RecentSearch { query: "local-first".to_string(), category: SearchCategory::Synapses, timestamp: "3 days ago".to_string() },
        ]
    }
}

/// Featured/Trending Synapse for discovery
#[derive(Clone, Debug)]
pub struct DiscoverySynapse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub member_count: u32,
    pub online_count: u32,
    pub growth_percentage: i32,
    pub is_verified: bool,
    pub is_joined: bool,
    pub featured_reason: String,
    pub category: String,
}

impl DiscoverySynapse {
    pub fn mock_featured() -> Vec<DiscoverySynapse> {
        vec![
            DiscoverySynapse {
                id: "syn-featured-1".to_string(),
                name: "WebAssembly Wizards".to_string(),
                description: "Push the boundaries of web performance with WASM. Tutorials, discussions, and cutting-edge projects.".to_string(),
                icon_url: None,
                member_count: 4500,
                online_count: 234,
                growth_percentage: 156,
                is_verified: true,
                is_joined: false,
                featured_reason: "Trending this week".to_string(),
                category: "Technology".to_string(),
            },
            DiscoverySynapse {
                id: "syn-featured-2".to_string(),
                name: "Cryptography Corner".to_string(),
                description: "Deep dives into modern cryptographic systems. From basics to zero-knowledge proofs.".to_string(),
                icon_url: None,
                member_count: 2800,
                online_count: 145,
                growth_percentage: 89,
                is_verified: true,
                is_joined: false,
                featured_reason: "Editor's pick".to_string(),
                category: "Security".to_string(),
            },
            DiscoverySynapse {
                id: "syn-featured-3".to_string(),
                name: "Distributed Systems Daily".to_string(),
                description: "Daily discussions on distributed computing, consensus algorithms, and system design.".to_string(),
                icon_url: None,
                member_count: 6700,
                online_count: 312,
                growth_percentage: 45,
                is_verified: false,
                is_joined: true,
                featured_reason: "Popular in your network".to_string(),
                category: "Technology".to_string(),
            },
        ]
    }

    pub fn mock_by_category() -> Vec<(String, Vec<DiscoverySynapse>)> {
        vec![
            ("Technology".to_string(), vec![
                DiscoverySynapse {
                    id: "cat-tech-1".to_string(),
                    name: "Systems Programming".to_string(),
                    description: "Low-level programming, OS development, and performance optimization.".to_string(),
                    icon_url: None,
                    member_count: 3200,
                    online_count: 156,
                    growth_percentage: 23,
                    is_verified: true,
                    is_joined: false,
                    featured_reason: "".to_string(),
                    category: "Technology".to_string(),
                },
                DiscoverySynapse {
                    id: "cat-tech-2".to_string(),
                    name: "Frontend Frontier".to_string(),
                    description: "Modern frontend development. React, Vue, Svelte, and beyond.".to_string(),
                    icon_url: None,
                    member_count: 8900,
                    online_count: 445,
                    growth_percentage: 34,
                    is_verified: true,
                    is_joined: false,
                    featured_reason: "".to_string(),
                    category: "Technology".to_string(),
                },
            ]),
            ("Gaming".to_string(), vec![
                DiscoverySynapse {
                    id: "cat-gaming-1".to_string(),
                    name: "Indie Game Devs".to_string(),
                    description: "A supportive community for independent game developers.".to_string(),
                    icon_url: None,
                    member_count: 5600,
                    online_count: 278,
                    growth_percentage: 56,
                    is_verified: false,
                    is_joined: false,
                    featured_reason: "".to_string(),
                    category: "Gaming".to_string(),
                },
            ]),
            ("Science".to_string(), vec![
                DiscoverySynapse {
                    id: "cat-science-1".to_string(),
                    name: "Quantum Computing".to_string(),
                    description: "Exploring the frontier of quantum computation and algorithms.".to_string(),
                    icon_url: None,
                    member_count: 2100,
                    online_count: 89,
                    growth_percentage: 78,
                    is_verified: true,
                    is_joined: false,
                    featured_reason: "".to_string(),
                    category: "Science".to_string(),
                },
            ]),
        ]
    }
}

/// Suggested user for discovery
#[derive(Clone, Debug)]
pub struct DiscoveryUser {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub reputation: u32,
    pub is_verified: bool,
    pub follower_count: u32,
    pub mutual_count: u32,
    pub reason: String, // Why we're suggesting this user
    pub top_tags: Vec<String>,
}

impl DiscoveryUser {
    pub fn mock_suggested() -> Vec<DiscoveryUser> {
        vec![
            DiscoveryUser {
                handle: "architect".to_string(),
                display_name: "The Architect".to_string(),
                avatar_url: None,
                bio: "Creator of systems. Designer of matrices.".to_string(),
                reputation: 7800,
                is_verified: true,
                follower_count: 34500,
                mutual_count: 12,
                reason: "Followed by people you follow".to_string(),
                top_tags: vec!["systems".to_string(), "architecture".to_string()],
            },
            DiscoveryUser {
                handle: "merovingian".to_string(),
                display_name: "The Merovingian".to_string(),
                avatar_url: None,
                bio: "Cause and effect. That is the way of things.".to_string(),
                reputation: 5600,
                is_verified: true,
                follower_count: 18900,
                mutual_count: 8,
                reason: "Popular in Zion HQ".to_string(),
                top_tags: vec!["philosophy".to_string(), "power".to_string()],
            },
            DiscoveryUser {
                handle: "switch".to_string(),
                display_name: "Switch".to_string(),
                avatar_url: None,
                bio: "Not everyone believes what you believe.".to_string(),
                reputation: 2300,
                is_verified: false,
                follower_count: 4500,
                mutual_count: 15,
                reason: "Active in synapses you follow".to_string(),
                top_tags: vec!["operations".to_string(), "security".to_string()],
            },
            DiscoveryUser {
                handle: "dozer".to_string(),
                display_name: "Dozer".to_string(),
                avatar_url: None,
                bio: "Born in the real world. Pure bred human.".to_string(),
                reputation: 1800,
                is_verified: false,
                follower_count: 3200,
                mutual_count: 18,
                reason: "Similar interests to you".to_string(),
                top_tags: vec!["rust".to_string(), "engineering".to_string()],
            },
        ]
    }
}

/// Trending post for discovery
#[derive(Clone, Debug)]
pub struct DiscoveryPost {
    pub id: String,
    pub author_handle: String,
    pub author_name: String,
    pub author_avatar: Option<String>,
    pub author_verified: bool,
    pub synapse_name: String,
    pub content_preview: String,
    pub likes: u32,
    pub comments: u32,
    pub timestamp: String,
    pub trending_reason: String,
}

impl DiscoveryPost {
    pub fn mock_trending() -> Vec<DiscoveryPost> {
        vec![
            DiscoveryPost {
                id: "trending-1".to_string(),
                author_handle: "neo".to_string(),
                author_name: "Neo Anderson".to_string(),
                author_avatar: None,
                author_verified: true,
                synapse_name: "Zion HQ".to_string(),
                content_preview: "The revolution starts not with a single act, but with a single thought that spreads across the network...".to_string(),
                likes: 2340,
                comments: 456,
                timestamp: "3 hours ago".to_string(),
                trending_reason: "🔥 Viral in your network".to_string(),
            },
            DiscoveryPost {
                id: "trending-2".to_string(),
                author_handle: "oracle".to_string(),
                author_name: "The Oracle".to_string(),
                author_avatar: None,
                author_verified: true,
                synapse_name: "Oracle's Temple".to_string(),
                content_preview: "You already know what I'm going to say, don't you? The answer isn't out there, it's in here...".to_string(),
                likes: 1890,
                comments: 234,
                timestamp: "6 hours ago".to_string(),
                trending_reason: "📈 Highly engaging".to_string(),
            },
            DiscoveryPost {
                id: "trending-3".to_string(),
                author_handle: "tank".to_string(),
                author_name: "Tank".to_string(),
                author_avatar: None,
                author_verified: false,
                synapse_name: "Rust Developers".to_string(),
                content_preview: "Just open-sourced our entire CRDT implementation. 100% safe Rust, no unsafe blocks. Check it out...".to_string(),
                likes: 1234,
                comments: 189,
                timestamp: "12 hours ago".to_string(),
                trending_reason: "⭐ Editor's choice".to_string(),
            },
        ]
    }
}

/// Helper function to format counts
pub fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1000 {
        format!("{:.1}K", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}
