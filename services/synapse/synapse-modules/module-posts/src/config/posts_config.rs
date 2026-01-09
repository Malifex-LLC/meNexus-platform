// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::ModulePostsError;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::OnceLock;

/// Posts module configuration loaded from environment variables.
///
/// ## Environment Variables
/// - `MODULE_POSTS_CHANNELS` - Comma-separated list of channel names (default: "general,memes,support")
/// - `MODULE_POSTS_DEFAULT_CHANNEL` - Default channel for new posts (default: "general")
/// - `MODULE_POSTS_MAX_POST_LENGTH` - Maximum post content length in characters (default: 5000)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PostsConfig {
    pub channels: Vec<String>,
    pub default_channel: String,
    pub max_post_length: usize,
}

impl Default for PostsConfig {
    fn default() -> Self {
        Self {
            channels: vec![
                "general".to_string(),
                "memes".to_string(),
                "support".to_string(),
            ],
            default_channel: "general".to_string(),
            max_post_length: 5000,
        }
    }
}

/// Global cached config instance
static POSTS_CONFIG: OnceLock<PostsConfig> = OnceLock::new();

/// Load posts module configuration from environment variables.
/// 
/// Configuration is cached after first load for performance.
pub fn load_config() -> Result<PostsConfig, ModulePostsError> {
    Ok(POSTS_CONFIG.get_or_init(load_config_from_env).clone())
}

/// Load configuration directly from environment variables without caching.
fn load_config_from_env() -> PostsConfig {
    let channels = env::var("MODULE_POSTS_CHANNELS")
        .map(|s| s.split(',').map(|c| c.trim().to_string()).collect())
        .unwrap_or_else(|_| PostsConfig::default().channels);

    let default_channel = env::var("MODULE_POSTS_DEFAULT_CHANNEL")
        .unwrap_or_else(|_| PostsConfig::default().default_channel);

    let max_post_length = env::var("MODULE_POSTS_MAX_POST_LENGTH")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(PostsConfig::default().max_post_length);

    PostsConfig {
        channels,
        default_channel,
        max_post_length,
    }
}
