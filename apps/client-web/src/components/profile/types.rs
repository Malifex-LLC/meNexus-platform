// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// Reputation level thresholds and labels
#[derive(Clone, Debug, PartialEq)]
pub enum ReputationLevel {
    Newcomer,      // 0-299
    Contributor,   // 300-599
    Established,   // 600-999
    Trusted,       // 1000-1499
    Respected,     // 1500-1999
    Distinguished, // 2000-2499
    Elite,         // 2500-2999
    Legendary,     // 3000+
}

impl ReputationLevel {
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=299 => ReputationLevel::Newcomer,
            300..=599 => ReputationLevel::Contributor,
            600..=999 => ReputationLevel::Established,
            1000..=1499 => ReputationLevel::Trusted,
            1500..=1999 => ReputationLevel::Respected,
            2000..=2499 => ReputationLevel::Distinguished,
            2500..=2999 => ReputationLevel::Elite,
            _ => ReputationLevel::Legendary,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ReputationLevel::Newcomer => "Newcomer",
            ReputationLevel::Contributor => "Contributor",
            ReputationLevel::Established => "Established",
            ReputationLevel::Trusted => "Trusted",
            ReputationLevel::Respected => "Respected",
            ReputationLevel::Distinguished => "Distinguished",
            ReputationLevel::Elite => "Elite",
            ReputationLevel::Legendary => "Legendary",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            ReputationLevel::Newcomer => "rep-newcomer bg-rep-newcomer/10 border-rep-newcomer/20",
            ReputationLevel::Contributor => "rep-contributor bg-rep-contributor/15 border-rep-contributor/30",
            ReputationLevel::Established => "rep-established bg-rep-established/15 border-rep-established/30",
            ReputationLevel::Trusted => "rep-trusted bg-rep-trusted/15 border-rep-trusted/30",
            ReputationLevel::Respected => "rep-respected bg-rep-respected/15 border-rep-respected/30",
            ReputationLevel::Distinguished => "rep-distinguished bg-rep-distinguished/15 border-rep-distinguished/30",
            ReputationLevel::Elite => "rep-elite bg-rep-elite/15 border-rep-elite/30",
            ReputationLevel::Legendary => "rep-legendary bg-rep-legendary/15 border-rep-legendary/30",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            ReputationLevel::Newcomer => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/></svg>"#,
            ReputationLevel::Contributor => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/></svg>"#,
            ReputationLevel::Established => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#,
            ReputationLevel::Trusted => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"/></svg>"#,
            ReputationLevel::Respected => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2L15.09 8.26L22 9.27L17 14.14L18.18 21.02L12 17.77L5.82 21.02L7 14.14L2 9.27L8.91 8.26L12 2Z"/></svg>"#,
            ReputationLevel::Distinguished => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M5 16L3 5l5.5 5L12 4l3.5 6L21 5l-2 11H5zm14 3c0 .6-.4 1-1 1H6c-.6 0-1-.4-1-1v-1h14v1z"/></svg>"#,
            ReputationLevel::Elite => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 1L9 9l-8 3 8 3 3 8 3-8 8-3-8-3-3-8z"/></svg>"#,
            ReputationLevel::Legendary => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-5.5-2.5l7.51-3.49L17.5 6.5 9.99 9.99 6.5 17.5zm5.5-6.6c.61 0 1.1.49 1.1 1.1s-.49 1.1-1.1 1.1-1.1-.49-1.1-1.1.49-1.1 1.1-1.1z"/></svg>"#,
        }
    }
}

/// User reputation data
#[derive(Clone, Debug)]
pub struct Reputation {
    pub score: u32,
    pub level: ReputationLevel,
    pub creator_percentage: u8,  // 0-100
    pub curator_percentage: u8,  // 0-100
    pub peer_endorsements: u32,
    pub credentials_count: u32,
}

impl Reputation {
    pub fn mock() -> Self {
        Self {
            score: 1247,
            level: ReputationLevel::Trusted,
            creator_percentage: 68,
            curator_percentage: 32,
            peer_endorsements: 89,
            credentials_count: 12,
        }
    }
}

/// A credential/badge earned by the user
#[derive(Clone, Debug)]
pub struct Credential {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub issuer_name: String,
    pub issuer_handle: String,
    pub earned_at: String,
    pub is_verified: bool,
    pub rarity: CredentialRarity,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CredentialRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl CredentialRarity {
    pub fn color_class(&self) -> &'static str {
        match self {
            CredentialRarity::Common => "from-foreground/40 to-foreground/60",
            CredentialRarity::Uncommon => "from-success to-success/70",
            CredentialRarity::Rare => "from-info to-info/70",
            CredentialRarity::Epic => "from-secondary to-secondary/70",
            CredentialRarity::Legendary => "from-warning to-warning/70",
        }
    }

    pub fn border_class(&self) -> &'static str {
        match self {
            CredentialRarity::Common => "border-foreground/30",
            CredentialRarity::Uncommon => "border-success/30",
            CredentialRarity::Rare => "border-info/30",
            CredentialRarity::Epic => "border-secondary/30",
            CredentialRarity::Legendary => "border-warning/30",
        }
    }
}

impl Credential {
    pub fn mock_credentials() -> Vec<Credential> {
        vec![
            Credential {
                id: "cred-1".to_string(),
                name: "Early Adopter".to_string(),
                description: "Joined during the alpha phase".to_string(),
                icon_url: None,
                issuer_name: "meNexus".to_string(),
                issuer_handle: "menexus".to_string(),
                earned_at: "Dec 2024".to_string(),
                is_verified: true,
                rarity: CredentialRarity::Legendary,
            },
            Credential {
                id: "cred-2".to_string(),
                name: "Code Contributor".to_string(),
                description: "Contributed code to the platform".to_string(),
                icon_url: None,
                issuer_name: "meNexus Dev".to_string(),
                issuer_handle: "menexus-dev".to_string(),
                earned_at: "Jan 2025".to_string(),
                is_verified: true,
                rarity: CredentialRarity::Epic,
            },
            Credential {
                id: "cred-3".to_string(),
                name: "Community Helper".to_string(),
                description: "Helped 50+ community members".to_string(),
                icon_url: None,
                issuer_name: "Support Team".to_string(),
                issuer_handle: "support".to_string(),
                earned_at: "Feb 2025".to_string(),
                is_verified: true,
                rarity: CredentialRarity::Rare,
            },
            Credential {
                id: "cred-4".to_string(),
                name: "Verified Creator".to_string(),
                description: "Identity verified as content creator".to_string(),
                icon_url: None,
                issuer_name: "Creator Program".to_string(),
                issuer_handle: "creators".to_string(),
                earned_at: "Mar 2025".to_string(),
                is_verified: true,
                rarity: CredentialRarity::Uncommon,
            },
        ]
    }
}

/// A showcased post
#[derive(Clone, Debug)]
pub struct ShowcasedPost {
    pub id: String,
    pub content: String,
    pub timestamp: String,
    pub likes: u32,
    pub comments: u32,
    pub is_pinned: bool,
}

impl ShowcasedPost {
    pub fn mock_posts() -> Vec<ShowcasedPost> {
        vec![
            ShowcasedPost {
                id: "post-1".to_string(),
                content: "Just shipped a major update to the sync protocol. Performance is now 3x faster on large document sets. Check out the benchmarks! 🚀".to_string(),
                timestamp: "2 days ago".to_string(),
                likes: 247,
                comments: 34,
                is_pinned: true,
            },
            ShowcasedPost {
                id: "post-2".to_string(),
                content: "The future of social networking is decentralized. No more platform lock-in, no more data harvesting. Your identity, your data, your rules.".to_string(),
                timestamp: "1 week ago".to_string(),
                likes: 892,
                comments: 156,
                is_pinned: false,
            },
            ShowcasedPost {
                id: "post-3".to_string(),
                content: "Been working on local-first architecture for 3 years now. The tooling has finally caught up with the vision. Exciting times ahead!".to_string(),
                timestamp: "2 weeks ago".to_string(),
                likes: 534,
                comments: 78,
                is_pinned: false,
            },
        ]
    }
}

/// A favorite Synapse
#[derive(Clone, Debug)]
pub struct FavoriteSynapse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub member_count: u32,
    pub online_count: u32,
    pub icon_url: Option<String>,
}

impl FavoriteSynapse {
    pub fn mock_synapses() -> Vec<FavoriteSynapse> {
        vec![
            FavoriteSynapse {
                id: "syn-1".to_string(),
                name: "Rust Devs".to_string(),
                description: "A community for Rust enthusiasts".to_string(),
                member_count: 12_450,
                online_count: 1_247,
                icon_url: None,
            },
            FavoriteSynapse {
                id: "syn-2".to_string(),
                name: "Local-First".to_string(),
                description: "Building offline-first applications".to_string(),
                member_count: 3_890,
                online_count: 456,
                icon_url: None,
            },
            FavoriteSynapse {
                id: "syn-3".to_string(),
                name: "P2P Networks".to_string(),
                description: "Peer-to-peer technology discussions".to_string(),
                member_count: 8_234,
                online_count: 892,
                icon_url: None,
            },
        ]
    }
}

/// A favorite user
#[derive(Clone, Debug)]
pub struct FavoriteUser {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_verified: bool,
    pub mutual_follows: bool,
}

impl FavoriteUser {
    pub fn mock_users() -> Vec<FavoriteUser> {
        vec![
            FavoriteUser {
                handle: "trinity".to_string(),
                display_name: "Trinity".to_string(),
                avatar_url: None,
                is_verified: true,
                mutual_follows: true,
            },
            FavoriteUser {
                handle: "morpheus".to_string(),
                display_name: "Morpheus".to_string(),
                avatar_url: None,
                is_verified: true,
                mutual_follows: true,
            },
            FavoriteUser {
                handle: "oracle".to_string(),
                display_name: "The Oracle".to_string(),
                avatar_url: None,
                is_verified: false,
                mutual_follows: false,
            },
        ]
    }
}

/// An external link highlighted by the user
#[derive(Clone, Debug)]
pub struct ExternalLink {
    pub id: String,
    pub title: String,
    pub url: String,
    pub icon: LinkIcon,
}

#[derive(Clone, Debug)]
pub enum LinkIcon {
    GitHub,
    Twitter,
    Website,
    LinkedIn,
    YouTube,
    Discord,
    Custom(String),
}

impl LinkIcon {
    pub fn svg(&self) -> &'static str {
        match self {
            LinkIcon::GitHub => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/></svg>"#,
            LinkIcon::Twitter => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>"#,
            LinkIcon::Website => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9 9 0 100-18 9 9 0 000 18z"/><path stroke-linecap="round" stroke-linejoin="round" d="M3.6 9h16.8M3.6 15h16.8M11.5 3a17 17 0 000 18M12.5 3a17 17 0 010 18"/></svg>"#,
            LinkIcon::LinkedIn => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433c-1.144 0-2.063-.926-2.063-2.065 0-1.138.92-2.063 2.063-2.063 1.14 0 2.064.925 2.064 2.063 0 1.139-.925 2.065-2.064 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z"/></svg>"#,
            LinkIcon::YouTube => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/></svg>"#,
            LinkIcon::Discord => r#"<svg viewBox="0 0 24 24" fill="currentColor"><path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189Z"/></svg>"#,
            LinkIcon::Custom(_) => r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/></svg>"#,
        }
    }
}

impl ExternalLink {
    pub fn mock_links() -> Vec<ExternalLink> {
        vec![
            ExternalLink {
                id: "link-1".to_string(),
                title: "GitHub".to_string(),
                url: "https://github.com/neo".to_string(),
                icon: LinkIcon::GitHub,
            },
            ExternalLink {
                id: "link-2".to_string(),
                title: "Twitter".to_string(),
                url: "https://twitter.com/neo".to_string(),
                icon: LinkIcon::Twitter,
            },
            ExternalLink {
                id: "link-3".to_string(),
                title: "Personal Site".to_string(),
                url: "https://neo.dev".to_string(),
                icon: LinkIcon::Website,
            },
        ]
    }
}

/// Extended profile data for the profile page
#[derive(Clone, Debug)]
pub struct ProfileData {
    pub public_key: String,
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub joined_at: String,
    pub is_verified: bool,
    pub is_online: bool,
    pub followers_count: u32,
    pub following_count: u32,
    pub posts_count: u32,
    pub reputation: Reputation,
}

impl ProfileData {
    pub fn mock() -> Self {
        Self {
            public_key: "02a3b5c7d9e1f2a4b6c8d0e2f4a6b8c0d2e4f6a8b0c2d4e6f8a0b2c4d6e8f0a2b4".to_string(),
            handle: "neo".to_string(),
            display_name: "Neo Anderson".to_string(),
            bio: Some("Building the future of decentralized social networking. Rust enthusiast, local-first advocate, and believer in user sovereignty. The Matrix has you.".to_string()),
            location: Some("The Matrix".to_string()),
            avatar_url: None,
            banner_url: None,
            joined_at: "December 2024".to_string(),
            is_verified: true,
            is_online: true,
            followers_count: 12_847,
            following_count: 342,
            posts_count: 1_247,
            reputation: Reputation::mock(),
        }
    }

    pub fn get_initials(&self) -> String {
        self.display_name
            .split_whitespace()
            .take(2)
            .filter_map(|s| s.chars().next())
            .collect::<String>()
            .to_uppercase()
    }
}
