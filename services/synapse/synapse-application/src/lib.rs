// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod events;
pub mod federation;

use std::env;
use std::path::PathBuf;
use synapse_config::SynapseConfig;
use synapse_config::load_or_init_config;
use synapse_core::errors::CoreError;

pub fn get_synapse_config() -> Result<SynapseConfig, CoreError> {
    let config_path: PathBuf = env::var("CONFIG_PATH").unwrap().parse().unwrap();
    let synapse_config = load_or_init_config(config_path).unwrap();
    Ok(synapse_config)
}
