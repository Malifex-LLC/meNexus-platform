// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Manifest
//!
//! The `SynapseManifest` defines how a Synapse should be rendered.
//!
//! ## Configuration via Environment Variables
//!
//! ```yaml
//! # Identity
//! - SYNAPSE_NAME=My Community
//! - SYNAPSE_DESCRIPTION=A great place
//!
//! # Theme
//! - SYNAPSE_THEME_PRESET=default
//! - SYNAPSE_ACCENT_COLOR=#6366f1
//!
//! # Installed modules (available for use)
//! - SYNAPSE_MODULES=posts,chat,members,activity,livestream
//!
//! # Layout selection
//! - SYNAPSE_LAYOUT=TwoColumn
//!
//! # Module placement (which modules go where)
//! # For TwoColumn: LEFT and RIGHT
//! - SYNAPSE_LAYOUT_LEFT=posts,chat
//! - SYNAPSE_LAYOUT_RIGHT=members,activity
//!
//! # For ThreeColumn: LEFT, CENTER, RIGHT
//! - SYNAPSE_LAYOUT_LEFT=files
//! - SYNAPSE_LAYOUT_CENTER=posts,chat
//! - SYNAPSE_LAYOUT_RIGHT=members,activity
//!
//! # For SingleColumn: MAIN
//! - SYNAPSE_LAYOUT_MAIN=posts,chat,members,activity
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// Current manifest schema version
pub const MANIFEST_VERSION: &str = "1.0.0";

// =============================================================================
// SYNAPSE MANIFEST (Root)
// =============================================================================

/// The complete manifest that describes how to render a Synapse.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseManifest {
    /// Schema version for backwards compatibility.
    pub version: String,

    /// The Synapse's identity information
    pub identity: SynapseIdentity,

    /// Theme and appearance configuration
    #[serde(default)]
    pub theme: ThemeConfig,

    /// List of installed/available module IDs
    #[serde(default)]
    pub installed_modules: Vec<String>,

    /// Layout configuration
    pub layout: LayoutConfig,

    /// Capabilities and feature flags
    #[serde(default)]
    pub capabilities: SynapseCapabilities,
}

impl SynapseManifest {
    pub fn new(identity: SynapseIdentity) -> Self {
        Self {
            version: MANIFEST_VERSION.to_string(),
            identity,
            theme: ThemeConfig::default(),
            installed_modules: vec![
                "posts".to_string(),
                "chat".to_string(),
                "members".to_string(),
                "activity".to_string(),
            ],
            layout: LayoutConfig::default(),
            capabilities: SynapseCapabilities::default(),
        }
    }
}

// =============================================================================
// IDENTITY
// =============================================================================

/// The Synapse's identity - who it is and how it presents itself.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseIdentity {
    /// The Synapse's public key
    pub public_key: String,

    /// Display name
    pub name: String,

    /// Short description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Avatar image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<Url>,

    /// Banner image URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<Url>,

    /// Public web URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<Url>,

    /// Tags for discovery
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl SynapseIdentity {
    pub fn new(public_key: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            public_key: public_key.into(),
            name: name.into(),
            description: None,
            avatar_url: None,
            banner_url: None,
            public_url: None,
            tags: Vec::new(),
        }
    }
}

// =============================================================================
// THEME
// =============================================================================

/// Theme configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeConfig {
    /// Built-in theme preset name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,

    /// Custom CSS URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css_url: Option<Url>,

    /// CSS variable overrides
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub variables: HashMap<String, String>,

    /// Primary accent color
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,

    /// Dark mode preference
    #[serde(default)]
    pub dark_mode: bool,
}

// =============================================================================
// LAYOUT
// =============================================================================

/// Layout configuration - defines structure and module placement
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutConfig {
    /// The layout template
    pub template: LayoutTemplate,

    /// Module assignments for each slot
    /// Keys are slot names: "left", "center", "right", "main"
    /// Values are lists of module IDs
    pub slots: HashMap<String, Vec<String>>,

    /// Show header with banner/title
    #[serde(default)]
    pub show_header: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        let mut slots = HashMap::new();
        slots.insert("left".to_string(), vec!["posts".to_string(), "chat".to_string()]);
        slots.insert("right".to_string(), vec!["members".to_string(), "activity".to_string()]);

        Self {
            template: LayoutTemplate::TwoColumn,
            slots,
            show_header: false,
        }
    }
}

impl LayoutConfig {
    /// Get modules for a specific slot
    pub fn get_slot(&self, slot: &str) -> Vec<String> {
        self.slots.get(slot).cloned().unwrap_or_default()
    }

    /// Get slot configuration for the current template
    pub fn get_slot_config(&self) -> Vec<SlotConfig> {
        match self.template {
            LayoutTemplate::SingleColumn => vec![
                SlotConfig { name: "main".to_string(), span: 12, modules: self.get_slot("main") },
            ],
            LayoutTemplate::TwoColumn => vec![
                SlotConfig { name: "left".to_string(), span: 8, modules: self.get_slot("left") },
                SlotConfig { name: "right".to_string(), span: 4, modules: self.get_slot("right") },
            ],
            LayoutTemplate::ThreeColumn => vec![
                SlotConfig { name: "left".to_string(), span: 3, modules: self.get_slot("left") },
                SlotConfig { name: "center".to_string(), span: 6, modules: self.get_slot("center") },
                SlotConfig { name: "right".to_string(), span: 3, modules: self.get_slot("right") },
            ],
            LayoutTemplate::Fullscreen => vec![
                SlotConfig { name: "main".to_string(), span: 12, modules: self.get_slot("main") },
            ],
        }
    }
}

/// Configuration for a single slot/column
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SlotConfig {
    /// Slot name (left, center, right, main)
    pub name: String,
    /// Grid span (out of 12)
    pub span: u8,
    /// Module IDs in this slot
    pub modules: Vec<String>,
}

/// Available layout templates
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LayoutTemplate {
    /// Single column (mobile-first, blog-style)
    SingleColumn,
    /// Two columns (main + sidebar) - 8/4 split
    #[default]
    TwoColumn,
    /// Three columns (sidebar + main + sidebar) - 3/6/3 split
    ThreeColumn,
    /// Full-width single module
    Fullscreen,
}

// =============================================================================
// CAPABILITIES
// =============================================================================

/// Feature flags the Synapse supports
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseCapabilities {
    #[serde(default = "default_true")]
    pub federation: bool,
    #[serde(default)]
    pub guest_access: bool,
    #[serde(default = "default_true")]
    pub open_membership: bool,
    #[serde(default = "default_true")]
    pub realtime: bool,
    #[serde(default = "default_true")]
    pub file_uploads: bool,
}

impl Default for SynapseCapabilities {
    fn default() -> Self {
        Self {
            federation: true,
            guest_access: false,
            open_membership: true,
            realtime: true,
            file_uploads: true,
        }
    }
}

fn default_true() -> bool {
    true
}

// =============================================================================
// WELL-KNOWN MODULES
// =============================================================================

/// Well-known module identifiers
pub mod known_modules {
    pub const POSTS: &str = "posts";
    pub const CHAT: &str = "chat";
    pub const ACTIVITY: &str = "activity";
    pub const MEMBERS: &str = "members";
    pub const LIVESTREAM: &str = "livestream";
    pub const CREATOR: &str = "creator";
    pub const FILES: &str = "files";
    pub const MEDIA: &str = "media";
    pub const CALENDAR: &str = "calendar";
    pub const WIKI: &str = "wiki";
    pub const POLLS: &str = "polls";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_serialization() {
        let identity = SynapseIdentity::new("pk_test123", "Test Synapse");
        let manifest = SynapseManifest::new(identity);

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        println!("{}", json);

        let parsed: SynapseManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.version, MANIFEST_VERSION);
        assert_eq!(parsed.identity.name, "Test Synapse");
    }

    #[test]
    fn test_slot_config() {
        let layout = LayoutConfig::default();
        let slots = layout.get_slot_config();
        
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].name, "left");
        assert_eq!(slots[0].span, 8);
        assert_eq!(slots[1].name, "right");
        assert_eq!(slots[1].span, 4);
    }
}
