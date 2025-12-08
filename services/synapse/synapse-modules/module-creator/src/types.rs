// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Access level for content
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    /// Free for everyone
    Free,
    /// Requires any paid subscription
    Subscribers,
    /// Requires specific tier or higher
    Tier(String),
}

impl AccessLevel {
    pub fn display_name(&self) -> String {
        match self {
            AccessLevel::Free => "Free".to_string(),
            AccessLevel::Subscribers => "Subscribers".to_string(),
            AccessLevel::Tier(name) => name.clone(),
        }
    }
}

/// Type of content
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    Image,
    Gallery,
    Video,
    Audio,
    Text,
    File,
    Poll,
    Livestream,
}

impl ContentType {
    pub fn icon(&self) -> &'static str {
        match self {
            ContentType::Image => "image",
            ContentType::Gallery => "gallery",
            ContentType::Video => "video",
            ContentType::Audio => "audio",
            ContentType::Text => "text",
            ContentType::File => "file",
            ContentType::Poll => "poll",
            ContentType::Livestream => "live",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ContentType::Image => "Image",
            ContentType::Gallery => "Gallery",
            ContentType::Video => "Video",
            ContentType::Audio => "Audio",
            ContentType::Text => "Article",
            ContentType::File => "Download",
            ContentType::Poll => "Poll",
            ContentType::Livestream => "Livestream",
        }
    }
}

/// Media attachment for content
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAttachment {
    pub id: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub media_type: ContentType,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_seconds: Option<u32>,
    pub file_size: Option<String>,
    pub file_name: Option<String>,
}

/// Poll option
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    pub id: String,
    pub text: String,
    pub votes: u32,
    pub percentage: f32,
}

/// A content post by a creator
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPost {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub content_type: ContentType,
    pub access_level: AccessLevel,
    pub media: Vec<MediaAttachment>,
    pub poll_options: Option<Vec<PollOption>>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub likes: u32,
    pub comments: u32,
    pub is_pinned: bool,
    pub is_scheduled: bool,
    pub scheduled_for: Option<OffsetDateTime>,
    pub tags: Vec<String>,
    /// Preview text shown for locked content
    pub preview_text: Option<String>,
    /// Blur level for locked media (0-100)
    pub preview_blur: u8,
}

impl ContentPost {
    pub fn mock_posts() -> Vec<Self> {
        vec![
            // Pinned welcome post
            ContentPost {
                id: "post-1".to_string(),
                title: Some("Welcome to my creative space! 🎨".to_string()),
                content: "Hey everyone! I'm so excited to launch my creator page here on meNexus. This is where I'll be sharing my digital art, behind-the-scenes content, tutorials, and exclusive pieces.\n\nWhat to expect:\n• Weekly art drops\n• Monthly tutorial videos\n• Exclusive wallpapers for supporters\n• Early access to new collections\n\nThank you for being here! 💜".to_string(),
                content_type: ContentType::Text,
                access_level: AccessLevel::Free,
                media: vec![],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 847,
                comments: 156,
                is_pinned: true,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["welcome".to_string(), "announcement".to_string()],
                preview_text: None,
                preview_blur: 0,
            },
            // Free image post
            ContentPost {
                id: "post-2".to_string(),
                title: Some("Neon Dreams - New Collection Preview".to_string()),
                content: "Here's a sneak peek at my upcoming 'Neon Dreams' collection. This piece took about 40 hours to complete. Full collection drops next week for Constellation tier and above!".to_string(),
                content_type: ContentType::Image,
                access_level: AccessLevel::Free,
                media: vec![MediaAttachment {
                    id: "media-1".to_string(),
                    url: "https://images.unsplash.com/photo-1634017839464-5c339bbe3c35?w=800".to_string(),
                    thumbnail_url: Some("https://images.unsplash.com/photo-1634017839464-5c339bbe3c35?w=400".to_string()),
                    media_type: ContentType::Image,
                    width: Some(1920),
                    height: Some(1080),
                    duration_seconds: None,
                    file_size: None,
                    file_name: None,
                }],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 1203,
                comments: 89,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["digital-art".to_string(), "neon".to_string(), "preview".to_string()],
                preview_text: None,
                preview_blur: 0,
            },
            // Subscriber-only gallery
            ContentPost {
                id: "post-3".to_string(),
                title: Some("Full Neon Dreams Collection (12 pieces)".to_string()),
                content: "The complete Neon Dreams collection is here! 12 high-resolution pieces, each available in 4K and 8K. Wallpaper packs included for all devices.".to_string(),
                content_type: ContentType::Gallery,
                access_level: AccessLevel::Tier("Constellation".to_string()),
                media: vec![
                    MediaAttachment {
                        id: "media-2".to_string(),
                        url: "https://images.unsplash.com/photo-1550745165-9bc0b252726f?w=800".to_string(),
                        thumbnail_url: Some("https://images.unsplash.com/photo-1550745165-9bc0b252726f?w=400".to_string()),
                        media_type: ContentType::Image,
                        width: Some(1920),
                        height: Some(1080),
                        duration_seconds: None,
                        file_size: None,
                        file_name: None,
                    },
                    MediaAttachment {
                        id: "media-3".to_string(),
                        url: "https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=800".to_string(),
                        thumbnail_url: Some("https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400".to_string()),
                        media_type: ContentType::Image,
                        width: Some(1920),
                        height: Some(1080),
                        duration_seconds: None,
                        file_size: None,
                        file_name: None,
                    },
                ],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 456,
                comments: 67,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["digital-art".to_string(), "neon".to_string(), "collection".to_string(), "wallpapers".to_string()],
                preview_text: Some("Get access to the full 12-piece collection with wallpaper packs...".to_string()),
                preview_blur: 25,
            },
            // Video tutorial (tier-locked)
            ContentPost {
                id: "post-4".to_string(),
                title: Some("Tutorial: Creating Cyberpunk Lighting Effects".to_string()),
                content: "In this 45-minute deep dive, I'll show you exactly how I create those signature neon glow effects. We'll cover:\n\n• Layer blending modes\n• Custom brush creation\n• Color grading techniques\n• Export settings for maximum quality\n\nProject files included!".to_string(),
                content_type: ContentType::Video,
                access_level: AccessLevel::Tier("Nebula".to_string()),
                media: vec![MediaAttachment {
                    id: "media-4".to_string(),
                    url: "https://example.com/video.mp4".to_string(),
                    thumbnail_url: Some("https://images.unsplash.com/photo-1633356122544-f134324a6cee?w=800".to_string()),
                    media_type: ContentType::Video,
                    width: Some(1920),
                    height: Some(1080),
                    duration_seconds: Some(2700),
                    file_size: Some("1.2 GB".to_string()),
                    file_name: Some("cyberpunk_lighting_tutorial.mp4".to_string()),
                }],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 892,
                comments: 134,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["tutorial".to_string(), "video".to_string(), "cyberpunk".to_string(), "lighting".to_string()],
                preview_text: Some("Learn my signature cyberpunk lighting technique in this comprehensive tutorial...".to_string()),
                preview_blur: 40,
            },
            // Free poll
            ContentPost {
                id: "post-5".to_string(),
                title: Some("What should I create next?".to_string()),
                content: "Help me decide my next major project! I've been thinking about these themes and would love your input. Poll closes in 3 days.".to_string(),
                content_type: ContentType::Poll,
                access_level: AccessLevel::Free,
                media: vec![],
                poll_options: Some(vec![
                    PollOption { id: "opt-1".to_string(), text: "🌊 Underwater Bioluminescence".to_string(), votes: 234, percentage: 38.5 },
                    PollOption { id: "opt-2".to_string(), text: "🏙️ Retro-Futuristic Cityscapes".to_string(), votes: 189, percentage: 31.1 },
                    PollOption { id: "opt-3".to_string(), text: "🌌 Deep Space Exploration".to_string(), votes: 156, percentage: 25.7 },
                    PollOption { id: "opt-4".to_string(), text: "🌿 Overgrown Technology".to_string(), votes: 29, percentage: 4.7 },
                ]),
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 156,
                comments: 45,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["poll".to_string(), "community".to_string()],
                preview_text: None,
                preview_blur: 0,
            },
            // Downloadable file (tier-locked)
            ContentPost {
                id: "post-6".to_string(),
                title: Some("Custom Brush Pack Vol. 3 - Texture Masters".to_string()),
                content: "50+ new custom brushes for Photoshop and Procreate! This pack focuses on organic textures, fabric patterns, and environmental details. Includes:\n\n• 25 Organic texture brushes\n• 15 Fabric & cloth brushes\n• 10 Environmental detail brushes\n• Installation guide\n• Sample project file".to_string(),
                content_type: ContentType::File,
                access_level: AccessLevel::Subscribers,
                media: vec![MediaAttachment {
                    id: "media-5".to_string(),
                    url: "https://example.com/brushpack.zip".to_string(),
                    thumbnail_url: Some("https://images.unsplash.com/photo-1618005182384-a83a8bd57fbe?w=800".to_string()),
                    media_type: ContentType::File,
                    width: None,
                    height: None,
                    duration_seconds: None,
                    file_size: Some("156 MB".to_string()),
                    file_name: Some("TextureMasters_BrushPack_v3.zip".to_string()),
                }],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 678,
                comments: 89,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["brushes".to_string(), "resources".to_string(), "photoshop".to_string(), "procreate".to_string()],
                preview_text: Some("Download 50+ custom brushes for your digital art workflow...".to_string()),
                preview_blur: 0,
            },
            // Audio post
            ContentPost {
                id: "post-7".to_string(),
                title: Some("Artist Commentary: The Making of 'Eclipse'".to_string()),
                content: "Listen to my thought process behind creating the 'Eclipse' piece. I discuss the inspiration, challenges, and happy accidents that shaped the final work.".to_string(),
                content_type: ContentType::Audio,
                access_level: AccessLevel::Tier("Stardust".to_string()),
                media: vec![MediaAttachment {
                    id: "media-6".to_string(),
                    url: "https://example.com/commentary.mp3".to_string(),
                    thumbnail_url: Some("https://images.unsplash.com/photo-1614149162883-504ce4d13909?w=800".to_string()),
                    media_type: ContentType::Audio,
                    width: None,
                    height: None,
                    duration_seconds: Some(1847),
                    file_size: Some("42 MB".to_string()),
                    file_name: Some("eclipse_commentary.mp3".to_string()),
                }],
                poll_options: None,
                created_at: OffsetDateTime::now_utc(),
                updated_at: None,
                likes: 234,
                comments: 28,
                is_pinned: false,
                is_scheduled: false,
                scheduled_for: None,
                tags: vec!["commentary".to_string(), "behind-the-scenes".to_string(), "audio".to_string()],
                preview_text: Some("Hear the story behind 'Eclipse' in this exclusive commentary...".to_string()),
                preview_blur: 0,
            },
        ]
    }
}

/// Subscription tier offered by a creator
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionTier {
    pub id: String,
    pub name: String,
    pub price_monthly: f64,
    pub price_yearly: Option<f64>,
    pub description: String,
    pub benefits: Vec<String>,
    pub subscriber_count: u32,
    pub is_limited: bool,
    pub limit: Option<u32>,
    pub badge_color: String,
    pub badge_icon: Option<String>,
    pub is_featured: bool,
}

impl SubscriptionTier {
    pub fn mock_tiers() -> Vec<Self> {
        vec![
            SubscriptionTier {
                id: "tier-1".to_string(),
                name: "Stardust".to_string(),
                price_monthly: 5.0,
                price_yearly: Some(50.0),
                description: "Support my work and get access to subscriber-only content".to_string(),
                benefits: vec![
                    "Access to all subscriber posts".to_string(),
                    "Monthly wallpaper pack".to_string(),
                    "Supporter badge on comments".to_string(),
                    "Early announcements".to_string(),
                ],
                subscriber_count: 1247,
                is_limited: false,
                limit: None,
                badge_color: "#9333ea".to_string(),
                badge_icon: Some("✦".to_string()),
                is_featured: false,
            },
            SubscriptionTier {
                id: "tier-2".to_string(),
                name: "Constellation".to_string(),
                price_monthly: 15.0,
                price_yearly: Some(150.0),
                description: "Get full access to collections, tutorials, and resources".to_string(),
                benefits: vec![
                    "Everything in Stardust".to_string(),
                    "Full art collections in 4K/8K".to_string(),
                    "Brush packs & resources".to_string(),
                    "Process videos & timelapses".to_string(),
                    "Discord access".to_string(),
                ],
                subscriber_count: 456,
                is_limited: false,
                limit: None,
                badge_color: "#3b82f6".to_string(),
                badge_icon: Some("✧".to_string()),
                is_featured: true,
            },
            SubscriptionTier {
                id: "tier-3".to_string(),
                name: "Nebula".to_string(),
                price_monthly: 35.0,
                price_yearly: Some(350.0),
                description: "The complete experience with tutorials, critiques, and more".to_string(),
                benefits: vec![
                    "Everything in Constellation".to_string(),
                    "Full tutorial library".to_string(),
                    "Monthly art critique session".to_string(),
                    "Project files & source PSDs".to_string(),
                    "Priority commission queue".to_string(),
                    "Name in credits".to_string(),
                ],
                subscriber_count: 89,
                is_limited: false,
                limit: None,
                badge_color: "#f59e0b".to_string(),
                badge_icon: Some("★".to_string()),
                is_featured: false,
            },
            SubscriptionTier {
                id: "tier-4".to_string(),
                name: "Supernova".to_string(),
                price_monthly: 100.0,
                price_yearly: None,
                description: "Ultimate tier with 1-on-1 mentorship and custom work".to_string(),
                benefits: vec![
                    "Everything in Nebula".to_string(),
                    "Monthly 1-on-1 mentorship call".to_string(),
                    "One custom piece per quarter".to_string(),
                    "Physical print annually".to_string(),
                    "Co-create content with me".to_string(),
                ],
                subscriber_count: 12,
                is_limited: true,
                limit: Some(25),
                badge_color: "#ef4444".to_string(),
                badge_icon: Some("◆".to_string()),
                is_featured: false,
            },
        ]
    }
}

/// Creator profile information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub bio: String,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Vec<SocialLink>,
    pub created_at: OffsetDateTime,
    pub is_verified: bool,
    pub is_featured: bool,
    pub category: String,
    pub tags: Vec<String>,
}

impl Creator {
    pub fn mock_creator() -> Self {
        Creator {
            id: "creator-1".to_string(),
            handle: "luminara".to_string(),
            display_name: "Luminara".to_string(),
            avatar_url: Some("https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=200".to_string()),
            banner_url: Some("https://images.unsplash.com/photo-1534796636912-3b95b3ab5986?w=1600".to_string()),
            bio: "Digital artist creating otherworldly landscapes and cosmic dreamscapes. Bringing imagination to life one pixel at a time. ✨\n\n🎨 Commissions: Open\n📧 Business: hello@luminara.art".to_string(),
            location: Some("Los Angeles, CA".to_string()),
            website: Some("https://luminara.art".to_string()),
            social_links: vec![
                SocialLink { platform: "Twitter".to_string(), url: "https://twitter.com/luminara".to_string(), handle: "@luminara".to_string() },
                SocialLink { platform: "Instagram".to_string(), url: "https://instagram.com/luminara.art".to_string(), handle: "@luminara.art".to_string() },
                SocialLink { platform: "ArtStation".to_string(), url: "https://artstation.com/luminara".to_string(), handle: "luminara".to_string() },
            ],
            created_at: OffsetDateTime::now_utc(),
            is_verified: true,
            is_featured: true,
            category: "Digital Art".to_string(),
            tags: vec!["digital-art".to_string(), "fantasy".to_string(), "sci-fi".to_string(), "landscapes".to_string()],
        }
    }
}

/// Social media link
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
    pub handle: String,
}

/// Creator statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatorStats {
    pub total_subscribers: u32,
    pub total_posts: u32,
    pub total_likes: u32,
    pub total_views: u32,
    pub monthly_earnings: Option<f64>,
    pub lifetime_earnings: Option<f64>,
    pub subscriber_growth: f32, // percentage
    pub engagement_rate: f32,   // percentage
}

impl CreatorStats {
    pub fn mock_stats() -> Self {
        CreatorStats {
            total_subscribers: 1804,
            total_posts: 156,
            total_likes: 45_230,
            total_views: 892_450,
            monthly_earnings: Some(8_450.0),
            lifetime_earnings: Some(127_890.0),
            subscriber_growth: 12.5,
            engagement_rate: 8.7,
        }
    }

    /// For public view (hides earnings)
    pub fn mock_public_stats() -> Self {
        CreatorStats {
            total_subscribers: 1804,
            total_posts: 156,
            total_likes: 45_230,
            total_views: 892_450,
            monthly_earnings: None,
            lifetime_earnings: None,
            subscriber_growth: 12.5,
            engagement_rate: 8.7,
        }
    }
}

/// A supporter/subscriber
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Supporter {
    pub id: String,
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub tier: SubscriptionTier,
    pub subscribed_at: OffsetDateTime,
    pub is_new: bool,
    pub total_support: f64,
    pub message: Option<String>,
}

impl Supporter {
    pub fn mock_supporters() -> Vec<Self> {
        let tiers = SubscriptionTier::mock_tiers();
        vec![
            Supporter {
                id: "sup-1".to_string(),
                handle: "artlover99".to_string(),
                display_name: "ArtLover99".to_string(),
                avatar_url: Some("https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=100".to_string()),
                tier: tiers[2].clone(),
                subscribed_at: OffsetDateTime::now_utc(),
                is_new: true,
                total_support: 35.0,
                message: Some("Your work is absolutely stunning! Can't wait for the tutorials! 🎨".to_string()),
            },
            Supporter {
                id: "sup-2".to_string(),
                handle: "creativemind".to_string(),
                display_name: "Creative Mind".to_string(),
                avatar_url: Some("https://images.unsplash.com/photo-1438761681033-6461ffad8d80?w=100".to_string()),
                tier: tiers[1].clone(),
                subscribed_at: OffsetDateTime::now_utc(),
                is_new: true,
                total_support: 150.0,
                message: None,
            },
            Supporter {
                id: "sup-3".to_string(),
                handle: "pixelmaster".to_string(),
                display_name: "Pixel Master".to_string(),
                avatar_url: None,
                tier: tiers[0].clone(),
                subscribed_at: OffsetDateTime::now_utc(),
                is_new: false,
                total_support: 60.0,
                message: Some("Keep up the amazing work!".to_string()),
            },
            Supporter {
                id: "sup-4".to_string(),
                handle: "dreamweaver".to_string(),
                display_name: "Dream Weaver".to_string(),
                avatar_url: Some("https://images.unsplash.com/photo-1527980965255-d3b416303d12?w=100".to_string()),
                tier: tiers[3].clone(),
                subscribed_at: OffsetDateTime::now_utc(),
                is_new: true,
                total_support: 500.0,
                message: Some("Honored to support your journey. The mentorship sessions have been incredible!".to_string()),
            },
            Supporter {
                id: "sup-5".to_string(),
                handle: "synthwave_sam".to_string(),
                display_name: "Synthwave Sam".to_string(),
                avatar_url: Some("https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100".to_string()),
                tier: tiers[0].clone(),
                subscribed_at: OffsetDateTime::now_utc(),
                is_new: false,
                total_support: 25.0,
                message: None,
            },
        ]
    }
}

/// User's subscription status to a creator
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    NotSubscribed,
    Subscribed { tier_id: String },
    Expired { tier_id: String },
}

/// Comment on a post
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author_handle: String,
    pub author_name: String,
    pub author_avatar: Option<String>,
    pub author_tier: Option<String>,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub likes: u32,
    pub replies: Vec<Comment>,
    pub is_creator: bool,
    pub is_pinned: bool,
}

impl Comment {
    pub fn mock_comments() -> Vec<Self> {
        vec![
            Comment {
                id: "comment-1".to_string(),
                author_handle: "artfan42".to_string(),
                author_name: "Art Fan 42".to_string(),
                author_avatar: Some("https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100".to_string()),
                author_tier: Some("Constellation".to_string()),
                content: "This is absolutely breathtaking! The color palette is perfect 😍".to_string(),
                created_at: OffsetDateTime::now_utc(),
                likes: 24,
                replies: vec![
                    Comment {
                        id: "comment-1-reply-1".to_string(),
                        author_handle: "luminara".to_string(),
                        author_name: "Luminara".to_string(),
                        author_avatar: Some("https://images.unsplash.com/photo-1494790108377-be9c29b29330?w=100".to_string()),
                        author_tier: None,
                        content: "Thank you so much! I spent a lot of time getting those purples just right 💜".to_string(),
                        created_at: OffsetDateTime::now_utc(),
                        likes: 45,
                        replies: vec![],
                        is_creator: true,
                        is_pinned: false,
                    }
                ],
                is_creator: false,
                is_pinned: false,
            },
            Comment {
                id: "comment-2".to_string(),
                author_handle: "designpro".to_string(),
                author_name: "Design Pro".to_string(),
                author_avatar: None,
                author_tier: Some("Nebula".to_string()),
                content: "Would love to see a tutorial on this technique!".to_string(),
                created_at: OffsetDateTime::now_utc(),
                likes: 18,
                replies: vec![],
                is_creator: false,
                is_pinned: false,
            },
        ]
    }
}

/// Filter options for content
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContentFilter {
    pub content_type: Option<ContentType>,
    pub access_level: Option<AccessLevel>,
    pub tag: Option<String>,
    pub search_query: Option<String>,
}

/// Sort options for content
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ContentSort {
    #[default]
    Newest,
    Oldest,
    MostLiked,
    MostCommented,
}

impl ContentSort {
    pub fn label(&self) -> &'static str {
        match self {
            ContentSort::Newest => "Newest",
            ContentSort::Oldest => "Oldest",
            ContentSort::MostLiked => "Most Liked",
            ContentSort::MostCommented => "Most Discussed",
        }
    }
}

