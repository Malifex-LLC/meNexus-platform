// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::ModulePostsError;
use serde::{Deserialize, Serialize};
use std::fs;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostsConfig {
    pub channels: Vec<String>,
}

#[cfg(feature = "ssr")]
impl Default for PostsConfig {
    fn default() -> Self {
        Self {
            channels: vec![
                "General".to_string(),
                "Memes".to_string(),
                "Support".to_string(),
            ],
        }
    }
}

#[cfg(feature = "ssr")]
pub fn load_config() -> Result<PostsConfig, ModulePostsError> {
    let config_path = "services/synapse/synapse-modules/module-posts/src/config/config.json";
    let config_contents = fs::read_to_string(config_path).unwrap();
    let config: PostsConfig = serde_json::from_str(&config_contents).unwrap();
    Ok(config)
}
