// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Synapse Configuration
//!
//! Configuration for a Synapse node, sourced from environment variables.
//!
//! ## Environment Variables
//!
//! ### Operational Config
//! - `CONFIG_PATH` - Path to store generated config JSON
//! - `PRIVATE_KEY_PATH` - Path to store/load private key
//! - `AXUM_PORT` - HTTP API port
//! - `LIBP2P_PORT` - P2P networking port
//!
//! ### Identity
//! - `SYNAPSE_NAME` - Display name
//! - `SYNAPSE_DESCRIPTION` - Short description
//! - `SYNAPSE_PUBLIC_URL` - Public web URL
//! - `SYNAPSE_AVATAR_URL` - Avatar image URL
//! - `SYNAPSE_BANNER_URL` - Banner image URL
//! - `SYNAPSE_TAGS` - Comma-separated tags
//!
//! ### Theme
//! - `SYNAPSE_THEME_PRESET` - Theme preset (default, midnight, etc.)
//! - `SYNAPSE_ACCENT_COLOR` - Accent color hex
//! - `SYNAPSE_DARK_MODE` - Dark mode (true/false)
//!
//! ### Modules & Layout
//! - `SYNAPSE_MODULES` - Comma-separated list of installed modules
//! - `SYNAPSE_LAYOUT` - Layout template (SingleColumn, TwoColumn, ThreeColumn, Fullscreen)
//! - `SYNAPSE_LAYOUT_LEFT` - Modules for left slot
//! - `SYNAPSE_LAYOUT_CENTER` - Modules for center slot
//! - `SYNAPSE_LAYOUT_RIGHT` - Modules for right slot
//! - `SYNAPSE_LAYOUT_MAIN` - Modules for main slot (SingleColumn/Fullscreen)
//!
//! ### Capabilities
//! - `SYNAPSE_FEDERATION` - Enable federation (true/false)
//! - `SYNAPSE_GUEST_ACCESS` - Allow guests (true/false)
//! - `SYNAPSE_SHOW_HEADER` - Show header with banner/title (true/false)

pub mod error;
pub mod manifest;

// Re-export manifest types
pub use manifest::{
    SynapseManifest,
    SynapseIdentity,
    ThemeConfig,
    LayoutConfig,
    LayoutTemplate,
    SlotConfig,
    SynapseCapabilities,
    known_modules,
    MANIFEST_VERSION,
};

use base64::{Engine as _, engine::general_purpose, engine::general_purpose::URL_SAFE_NO_PAD};
use libp2p::Multiaddr;
use libp2p::identity::Keypair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use url::Url;

use crate::error::SynapseConfigError;

// =============================================================================
// OPERATIONAL CONFIG
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SynapseConfig {
    pub version: String,
    pub identity: IdentityConfig,
    pub p2p: P2pConfig,
    pub api: ApiConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityConfig {
    pub name: String,
    pub description: String,
    pub public_key: String,
    pub private_key_path: PathBuf,
    pub public_url: Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct P2pConfig {
    pub listen_addrs: Multiaddr,
    pub announce: Vec<String>,
    pub bootstrap: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    pub port: u16,
}

// =============================================================================
// COMBINED CONFIG
// =============================================================================

#[derive(Clone, Debug)]
pub struct SynapseFullConfig {
    pub config: SynapseConfig,
    pub manifest: SynapseManifest,
}

// =============================================================================
// ENVIRONMENT HELPERS
// =============================================================================

fn env_var_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_var_opt(key: &str) -> Option<String> {
    env::var(key).ok().filter(|s| !s.is_empty())
}

fn env_var_list(key: &str) -> Vec<String> {
    env::var(key)
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn env_var_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes"))
        .unwrap_or(default)
}

fn load_or_generate_keypair(key_path: &PathBuf) -> Result<Keypair, SynapseConfigError> {
    if key_path.exists() {
        let encoded = fs::read_to_string(key_path)?;
        let bytes = general_purpose::STANDARD.decode(encoded.trim())?;
        let keypair = Keypair::from_protobuf_encoding(&bytes)?;
        Ok(keypair)
    } else {
        let keypair = Keypair::generate_secp256k1();
        let bytes = keypair.to_protobuf_encoding()?;
        let encoded = general_purpose::STANDARD.encode(bytes);
        if let Some(parent) = key_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(key_path, encoded)?;
        Ok(keypair)
    }
}

// =============================================================================
// CONFIG BUILDING
// =============================================================================

fn build_synapse_config(keypair: &Keypair, private_key_path: PathBuf) -> Result<SynapseConfig, SynapseConfigError> {
    let port: u16 = env::var("AXUM_PORT")?.parse()?;
    let listen_port_str = env::var("LIBP2P_PORT")?;
    
    let pk_bytes = keypair.public().encode_protobuf();
    let pk_str = URL_SAFE_NO_PAD.encode(pk_bytes);
    
    let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", listen_port_str)
        .parse()
        .expect("Invalid listen Multiaddr");

    let public_url_str = env_var_or("SYNAPSE_PUBLIC_URL", "http://localhost");
    let public_url = Url::parse(&public_url_str)?;

    Ok(SynapseConfig {
        version: "0.3.0".to_string(),
        identity: IdentityConfig {
            name: env_var_or("SYNAPSE_NAME", "New Synapse"),
            description: env_var_or("SYNAPSE_DESCRIPTION", "A meNexus Synapse"),
            public_key: pk_str,
            private_key_path,
            public_url,
        },
        p2p: P2pConfig {
            listen_addrs: listen_addr,
            announce: env_var_list("ANNOUNCE"),
            bootstrap: env_var_list("BOOTSTRAP_LIST"),
        },
        api: ApiConfig { port },
    })
}

fn build_synapse_manifest(public_key: &str) -> Result<SynapseManifest, SynapseConfigError> {
    // Check for full JSON override
    if let Some(manifest_json) = env_var_opt("SYNAPSE_MANIFEST_JSON") {
        return Ok(serde_json::from_str(&manifest_json)?);
    }

    // Build identity
    let mut identity = SynapseIdentity::new(public_key, env_var_or("SYNAPSE_NAME", "New Synapse"));
    identity.description = env_var_opt("SYNAPSE_DESCRIPTION");
    identity.avatar_url = env_var_opt("SYNAPSE_AVATAR_URL").and_then(|s| Url::parse(&s).ok());
    identity.banner_url = env_var_opt("SYNAPSE_BANNER_URL").and_then(|s| Url::parse(&s).ok());
    identity.public_url = env_var_opt("SYNAPSE_PUBLIC_URL").and_then(|s| Url::parse(&s).ok());
    identity.tags = env_var_list("SYNAPSE_TAGS");

    // Build theme
    let theme = ThemeConfig {
        preset: env_var_opt("SYNAPSE_THEME_PRESET"),
        accent_color: env_var_opt("SYNAPSE_ACCENT_COLOR"),
        dark_mode: env_var_bool("SYNAPSE_DARK_MODE", true),
        ..Default::default()
    };

    // Get installed modules
    let installed_modules = {
        let modules = env_var_list("SYNAPSE_MODULES");
        if modules.is_empty() {
            vec!["posts".to_string(), "chat".to_string(), "members".to_string(), "activity".to_string()]
        } else {
            modules
        }
    };

    // Parse layout template
    let template = match env_var_or("SYNAPSE_LAYOUT", "TwoColumn").to_lowercase().as_str() {
        "singlecolumn" | "single" => LayoutTemplate::SingleColumn,
        "threecolumn" | "three" => LayoutTemplate::ThreeColumn,
        "fullscreen" | "full" => LayoutTemplate::Fullscreen,
        _ => LayoutTemplate::TwoColumn,
    };

    // Build slot assignments from env vars
    let mut slots = HashMap::new();
    
    // Check for explicit slot assignments
    let left = env_var_list("SYNAPSE_LAYOUT_LEFT");
    let center = env_var_list("SYNAPSE_LAYOUT_CENTER");
    let right = env_var_list("SYNAPSE_LAYOUT_RIGHT");
    let main = env_var_list("SYNAPSE_LAYOUT_MAIN");

    // If no explicit assignments, use smart defaults based on template
    let has_explicit = !left.is_empty() || !center.is_empty() || !right.is_empty() || !main.is_empty();

    if has_explicit {
        if !left.is_empty() { slots.insert("left".to_string(), left); }
        if !center.is_empty() { slots.insert("center".to_string(), center); }
        if !right.is_empty() { slots.insert("right".to_string(), right); }
        if !main.is_empty() { slots.insert("main".to_string(), main); }
    } else {
        // Smart defaults based on template and installed modules
        match template {
            LayoutTemplate::SingleColumn | LayoutTemplate::Fullscreen => {
                slots.insert("main".to_string(), installed_modules.clone());
            }
            LayoutTemplate::TwoColumn => {
                let (left, right) = split_modules_two_column(&installed_modules);
                slots.insert("left".to_string(), left);
                slots.insert("right".to_string(), right);
            }
            LayoutTemplate::ThreeColumn => {
                let (left, center, right) = split_modules_three_column(&installed_modules);
                slots.insert("left".to_string(), left);
                slots.insert("center".to_string(), center);
                slots.insert("right".to_string(), right);
            }
        }
    }

    let layout = LayoutConfig {
        template,
        slots,
        show_header: env_var_bool("SYNAPSE_SHOW_HEADER", false),
    };

    // Build capabilities
    let capabilities = SynapseCapabilities {
        federation: env_var_bool("SYNAPSE_FEDERATION", true),
        guest_access: env_var_bool("SYNAPSE_GUEST_ACCESS", false),
        open_membership: env_var_bool("SYNAPSE_OPEN_MEMBERSHIP", true),
        realtime: env_var_bool("SYNAPSE_REALTIME", true),
        file_uploads: env_var_bool("SYNAPSE_FILE_UPLOADS", true),
    };

    Ok(SynapseManifest {
        version: MANIFEST_VERSION.to_string(),
        identity,
        theme,
        installed_modules,
        layout,
        capabilities,
    })
}

/// Split modules for two-column layout
fn split_modules_two_column(modules: &[String]) -> (Vec<String>, Vec<String>) {
    let primary = ["posts", "chat", "livestream", "creator"];
    let secondary = ["members", "activity", "files", "calendar"];

    let mut left = Vec::new();
    let mut right = Vec::new();

    for module in modules {
        if primary.contains(&module.as_str()) {
            left.push(module.clone());
        } else if secondary.contains(&module.as_str()) {
            right.push(module.clone());
        } else {
            left.push(module.clone());
        }
    }

    // Ensure at least one module per column
    if left.is_empty() && !right.is_empty() {
        left.push(right.remove(0));
    }
    if right.is_empty() && left.len() > 1 {
        right.push(left.pop().unwrap());
    }

    (left, right)
}

/// Split modules for three-column layout
fn split_modules_three_column(modules: &[String]) -> (Vec<String>, Vec<String>, Vec<String>) {
    let left_mods = ["files", "calendar"];
    let center_mods = ["posts", "chat", "livestream", "creator"];
    let right_mods = ["members", "activity"];

    let mut left = Vec::new();
    let mut center = Vec::new();
    let mut right = Vec::new();

    for module in modules {
        if left_mods.contains(&module.as_str()) {
            left.push(module.clone());
        } else if center_mods.contains(&module.as_str()) {
            center.push(module.clone());
        } else if right_mods.contains(&module.as_str()) {
            right.push(module.clone());
        } else {
            center.push(module.clone());
        }
    }

    (left, center, right)
}

// =============================================================================
// PUBLIC API
// =============================================================================

/// Initialize and load the complete Synapse configuration.
pub fn init_synapse_config() -> Result<SynapseFullConfig, SynapseConfigError> {
    let config_path: PathBuf = env::var("CONFIG_PATH")?.parse()?;
    let manifest_path = config_path.with_file_name("synapse_manifest.json");
    let private_key_path: PathBuf = env::var("PRIVATE_KEY_PATH")?.parse()?;

    let keypair = load_or_generate_keypair(&private_key_path)?;
    let public_key = URL_SAFE_NO_PAD.encode(keypair.public().encode_protobuf());

    let config = build_synapse_config(&keypair, private_key_path)?;
    let manifest = build_synapse_manifest(&public_key)?;

    // Persist to JSON
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&config_path, serde_json::to_string_pretty(&config)?)?;
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    tracing::info!("Synapse config initialized: {}", config.identity.name);

    Ok(SynapseFullConfig { config, manifest })
}

pub fn get_synapse_config() -> Result<SynapseConfig, SynapseConfigError> {
    Ok(init_synapse_config()?.config)
}

pub fn get_synapse_manifest() -> Result<SynapseManifest, SynapseConfigError> {
    Ok(init_synapse_config()?.manifest)
}

pub fn load_manifest_from_file() -> Result<SynapseManifest, SynapseConfigError> {
    let config_path: PathBuf = env::var("CONFIG_PATH")?.parse()?;
    let manifest_path = config_path.with_file_name("synapse_manifest.json");
    let contents = fs::read_to_string(&manifest_path)?;
    Ok(serde_json::from_str(&contents)?)
}

// Legacy support
#[deprecated(since = "0.4.0", note = "Use init_synapse_config instead")]
pub fn load_or_init_config(_config_path: PathBuf) -> Result<SynapseConfig, SynapseConfigError> {
    get_synapse_config()
}

#[deprecated(since = "0.4.0", note = "Keypair generation is internal")]
pub fn generate_secp256k1_keypair(key_path: &PathBuf) -> Result<Keypair, SynapseConfigError> {
    load_or_generate_keypair(key_path)
}
