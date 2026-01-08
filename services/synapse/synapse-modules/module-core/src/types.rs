// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Client-friendly Manifest Types
//!
//! These types mirror `synapse-config` manifest types but use simple
//! serializable types for Leptos server functions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use synapse_application::events::CreateRemoteEventUseCase;

// =============================================================================
// DEPENDENCIES
// =============================================================================

/// Dependencies for module-core server functions
#[cfg(feature = "ssr")]
#[derive(Clone)]
pub struct CoreDeps {
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
}

// =============================================================================
// CLIENT MANIFEST
// =============================================================================

/// Client-friendly SynapseManifest
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientManifest {
    pub version: String,
    pub identity: ClientIdentity,
    pub theme: ClientTheme,
    pub installed_modules: Vec<String>,
    pub layout: ClientLayout,
    pub capabilities: ClientCapabilities,
}

// =============================================================================
// IDENTITY
// =============================================================================

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientIdentity {
    pub public_key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

// =============================================================================
// THEME
// =============================================================================

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css_url: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub dark_mode: bool,
}

// =============================================================================
// LAYOUT
// =============================================================================

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientLayout {
    pub template: LayoutTemplate,
    /// Module assignments keyed by slot name
    pub slots: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub show_header: bool,
}

impl ClientLayout {
    /// Get modules for a specific slot
    pub fn get_slot(&self, slot: &str) -> Vec<String> {
        self.slots.get(slot).cloned().unwrap_or_default()
    }

    /// Get all slots configured for this layout template
    pub fn get_slot_configs(&self) -> Vec<SlotConfig> {
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SlotConfig {
    /// Slot name (left, center, right, main)
    pub name: String,
    /// Grid span out of 12
    pub span: u8,
    /// Module IDs assigned to this slot
    pub modules: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LayoutTemplate {
    SingleColumn,
    #[default]
    TwoColumn,
    ThreeColumn,
    Fullscreen,
}

// =============================================================================
// CAPABILITIES
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    #[serde(default = "default_true")]
    pub federation: bool,
    #[serde(default)]
    pub guest_access: bool,
    #[serde(default = "default_true")]
    pub open_membership: bool,
    #[serde(default = "default_true")]
    pub realtime: bool,
}

impl Default for ClientCapabilities {
    fn default() -> Self {
        Self {
            federation: true,
            guest_access: false,
            open_membership: true,
            realtime: true,
        }
    }
}

fn default_true() -> bool {
    true
}

// =============================================================================
// CONVERSION FROM synapse-config
// =============================================================================

#[cfg(feature = "ssr")]
impl From<synapse_config::SynapseManifest> for ClientManifest {
    fn from(m: synapse_config::SynapseManifest) -> Self {
        Self {
            version: m.version,
            identity: ClientIdentity {
                public_key: m.identity.public_key,
                name: m.identity.name,
                description: m.identity.description,
                avatar_url: m.identity.avatar_url.map(|u| u.to_string()),
                banner_url: m.identity.banner_url.map(|u| u.to_string()),
                public_url: m.identity.public_url.map(|u| u.to_string()),
                tags: m.identity.tags,
            },
            theme: ClientTheme {
                preset: m.theme.preset,
                custom_css_url: m.theme.custom_css_url.map(|u| u.to_string()),
                variables: m.theme.variables,
                accent_color: m.theme.accent_color,
                dark_mode: m.theme.dark_mode,
            },
            installed_modules: m.installed_modules,
            layout: ClientLayout {
                template: match m.layout.template {
                    synapse_config::LayoutTemplate::SingleColumn => LayoutTemplate::SingleColumn,
                    synapse_config::LayoutTemplate::TwoColumn => LayoutTemplate::TwoColumn,
                    synapse_config::LayoutTemplate::ThreeColumn => LayoutTemplate::ThreeColumn,
                    synapse_config::LayoutTemplate::Fullscreen => LayoutTemplate::Fullscreen,
                },
                slots: m.layout.slots,
                show_header: m.layout.show_header,
            },
            capabilities: ClientCapabilities {
                federation: m.capabilities.federation,
                guest_access: m.capabilities.guest_access,
                open_membership: m.capabilities.open_membership,
                realtime: m.capabilities.realtime,
            },
        }
    }
}

// =============================================================================
// WELL-KNOWN MODULE IDS
// =============================================================================

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
}
