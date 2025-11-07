// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod events;
pub mod federation;

use std::env;
use std::path::PathBuf;
use synapse_config::SynapseConfig;
use synapse_config::load_or_init_config;
use synapse_core::errors::CoreError;
use synapse_core::ports::config::errors::ConfigError;

pub fn get_synapse_config() -> Result<SynapseConfig, ConfigError> {
    let config_path: PathBuf = env::var("CONFIG_PATH")?.parse()?;
    let synapse_config = load_or_init_config(config_path)?;
    Ok(synapse_config)
}
