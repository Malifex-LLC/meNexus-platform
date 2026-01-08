// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

/// Settings tab/section
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsTab {
    Profile,
    Account,
    Privacy,
    Notifications,
    Appearance,
    Security,
    Network,
    Data,
}

impl SettingsTab {
    pub fn label(&self) -> &'static str {
        match self {
            SettingsTab::Profile => "Profile",
            SettingsTab::Account => "Account",
            SettingsTab::Privacy => "Privacy",
            SettingsTab::Notifications => "Notifications",
            SettingsTab::Appearance => "Appearance",
            SettingsTab::Security => "Security",
            SettingsTab::Network => "Network",
            SettingsTab::Data => "Data & Storage",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SettingsTab::Profile => "Manage your public profile information",
            SettingsTab::Account => "Email, password, and account management",
            SettingsTab::Privacy => "Control who can see and interact with you",
            SettingsTab::Notifications => "Configure how you receive updates",
            SettingsTab::Appearance => "Customize your visual experience",
            SettingsTab::Security => "Sessions, devices, and security keys",
            SettingsTab::Network => "P2P and sync preferences",
            SettingsTab::Data => "Storage, cache, and data export",
        }
    }

    pub fn icon_svg(&self) -> &'static str {
        match self {
            SettingsTab::Profile => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>"#,
            SettingsTab::Account => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M5.121 17.804A13.937 13.937 0 0112 16c2.5 0 4.847.655 6.879 1.804M15 10a3 3 0 11-6 0 3 3 0 016 0zm6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>"#,
            SettingsTab::Privacy => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>"#,
            SettingsTab::Notifications => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>"#,
            SettingsTab::Appearance => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/>"#,
            SettingsTab::Security => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>"#,
            SettingsTab::Network => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>"#,
            SettingsTab::Data => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"/>"#,
        }
    }

    pub fn all() -> Vec<SettingsTab> {
        vec![
            SettingsTab::Profile,
            SettingsTab::Account,
            SettingsTab::Privacy,
            SettingsTab::Notifications,
            SettingsTab::Appearance,
            SettingsTab::Security,
            SettingsTab::Network,
            SettingsTab::Data,
        ]
    }
}

/// User theme options - controls main app appearance
/// This affects: Control Panel, Dashboard, Messages, Notifications, Search, Profile, Settings
/// Does NOT affect: Synapse layouts and modules (those use Synapse themes)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserTheme {
    Midnight,
    Obsidian,
    Slate,
    Aurora,
    Gruvbox,
    Light,
}

impl UserTheme {
    pub fn label(&self) -> &'static str {
        match self {
            UserTheme::Midnight => "Midnight",
            UserTheme::Obsidian => "Obsidian",
            UserTheme::Slate => "Slate",
            UserTheme::Aurora => "Aurora",
            UserTheme::Gruvbox => "Gruvbox",
            UserTheme::Light => "Light",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UserTheme::Midnight => "Deep blue with coral accents",
            UserTheme::Obsidian => "Pure black with emerald accents",
            UserTheme::Slate => "Cool gray with blue accents",
            UserTheme::Aurora => "Purple tones with violet accents",
            UserTheme::Gruvbox => "Retro warm with orange accents",
            UserTheme::Light => "Clean light mode",
        }
    }

    pub fn colors(&self) -> (&'static str, &'static str, &'static str) {
        // Returns (background, card, accent)
        match self {
            UserTheme::Midnight => ("#1a1a2e", "#1f1f3a", "#ff6b6b"),
            UserTheme::Obsidian => ("#0d0d0d", "#141414", "#10b981"),
            UserTheme::Slate => ("#1e293b", "#1e293b", "#3b82f6"),
            UserTheme::Aurora => ("#1a1625", "#1f1b2e", "#a855f7"),
            UserTheme::Gruvbox => ("#282828", "#3c3836", "#fe8019"),
            UserTheme::Light => ("#f8fafc", "#ffffff", "#6366f1"),
        }
    }

    pub fn all() -> Vec<UserTheme> {
        vec![
            UserTheme::Midnight,
            UserTheme::Obsidian,
            UserTheme::Slate,
            UserTheme::Aurora,
            UserTheme::Gruvbox,
            UserTheme::Light,
        ]
    }
}

// Keep ThemeOption for backwards compatibility
pub type ThemeOption = UserTheme;

/// Font size options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl FontSize {
    pub fn label(&self) -> &'static str {
        match self {
            FontSize::Small => "Small",
            FontSize::Medium => "Medium",
            FontSize::Large => "Large",
            FontSize::ExtraLarge => "Extra Large",
        }
    }

    pub fn all() -> Vec<FontSize> {
        vec![FontSize::Small, FontSize::Medium, FontSize::Large, FontSize::ExtraLarge]
    }
}

/// Profile visibility options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileVisibility {
    Public,
    NetworkOnly,
    Private,
}

impl ProfileVisibility {
    pub fn label(&self) -> &'static str {
        match self {
            ProfileVisibility::Public => "Public",
            ProfileVisibility::NetworkOnly => "Network Only",
            ProfileVisibility::Private => "Private",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ProfileVisibility::Public => "Anyone can see your profile",
            ProfileVisibility::NetworkOnly => "Only users in your network",
            ProfileVisibility::Private => "Only people you follow",
        }
    }
}

/// DM permission options
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMPermission {
    Everyone,
    Following,
    Mutuals,
    Nobody,
}

impl DMPermission {
    pub fn label(&self) -> &'static str {
        match self {
            DMPermission::Everyone => "Everyone",
            DMPermission::Following => "People you follow",
            DMPermission::Mutuals => "Mutual followers only",
            DMPermission::Nobody => "Nobody",
        }
    }
}

/// User profile data for settings
#[derive(Clone, Debug)]
pub struct ProfileSettingsData {
    pub display_name: String,
    pub handle: String,
    pub bio: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub location: String,
    pub website: String,
    pub birthday: Option<String>,
    pub pronouns: String,
}

impl Default for ProfileSettingsData {
    fn default() -> Self {
        Self {
            display_name: "Neo Anderson".to_string(),
            handle: "neo".to_string(),
            bio: "The One. Building the future of decentralized networks. Exploring the boundaries between reality and code.".to_string(),
            avatar_url: None,
            banner_url: None,
            location: "Zion".to_string(),
            website: "https://matrix.network".to_string(),
            birthday: Some("1962-03-11".to_string()),
            pronouns: "he/him".to_string(),
        }
    }
}

/// Active session data
#[derive(Clone, Debug)]
pub struct ActiveSession {
    pub id: String,
    pub device_name: String,
    pub device_type: String,
    pub location: String,
    pub ip_address: String,
    pub last_active: String,
    pub is_current: bool,
}

impl ActiveSession {
    pub fn mock_sessions() -> Vec<ActiveSession> {
        vec![
            ActiveSession {
                id: "session-1".to_string(),
                device_name: "MacBook Pro".to_string(),
                device_type: "desktop".to_string(),
                location: "San Francisco, CA".to_string(),
                ip_address: "192.168.1.xxx".to_string(),
                last_active: "Now".to_string(),
                is_current: true,
            },
            ActiveSession {
                id: "session-2".to_string(),
                device_name: "iPhone 15".to_string(),
                device_type: "mobile".to_string(),
                location: "San Francisco, CA".to_string(),
                ip_address: "192.168.1.xxx".to_string(),
                last_active: "2 hours ago".to_string(),
                is_current: false,
            },
            ActiveSession {
                id: "session-3".to_string(),
                device_name: "Firefox on Linux".to_string(),
                device_type: "desktop".to_string(),
                location: "New York, NY".to_string(),
                ip_address: "10.0.0.xxx".to_string(),
                last_active: "3 days ago".to_string(),
                is_current: false,
            },
        ]
    }
}

/// Connected application
#[derive(Clone, Debug)]
pub struct ConnectedApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub connected_date: String,
    pub last_used: String,
}

impl ConnectedApp {
    pub fn mock_apps() -> Vec<ConnectedApp> {
        vec![
            ConnectedApp {
                id: "app-1".to_string(),
                name: "Matrix Mobile".to_string(),
                description: "Official mobile application".to_string(),
                permissions: vec!["Read profile".to_string(), "Post content".to_string(), "Send DMs".to_string()],
                connected_date: "Jan 15, 2024".to_string(),
                last_used: "2 hours ago".to_string(),
            },
            ConnectedApp {
                id: "app-2".to_string(),
                name: "Analytics Dashboard".to_string(),
                description: "Track your engagement metrics".to_string(),
                permissions: vec!["Read profile".to_string(), "Read posts".to_string()],
                connected_date: "Feb 20, 2024".to_string(),
                last_used: "1 week ago".to_string(),
            },
        ]
    }
}

/// Blocked user
#[derive(Clone, Debug)]
pub struct BlockedUser {
    pub handle: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub blocked_date: String,
}

impl BlockedUser {
    pub fn mock_blocked() -> Vec<BlockedUser> {
        vec![
            BlockedUser {
                handle: "agent_smith".to_string(),
                display_name: "Agent Smith".to_string(),
                avatar_url: None,
                blocked_date: "Dec 5, 2024".to_string(),
            },
        ]
    }
}
