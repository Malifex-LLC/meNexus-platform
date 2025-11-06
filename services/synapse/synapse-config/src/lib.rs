// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use base64::{Engine as _, engine::general_purpose};
use libp2p::Multiaddr;
use libp2p::identity::{Keypair, PeerId};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::write;
use std::fs;
use std::path::{Path, PathBuf};
use synapse_core::errors::CoreError;
use url::Url;

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
    pub key_path: PathBuf,
    pub public_url: Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct P2pConfig {
    listen_addrs: Multiaddr,
    announce: Vec<String>,
    bootstrap: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    port: u16,
}

pub fn load_or_init_config(config_path: PathBuf) -> Result<SynapseConfig, CoreError> {
    if config_path.exists() {
        let contents = fs::read_to_string(&config_path).unwrap();
        let config: SynapseConfig = serde_json::from_str(&contents).unwrap();
        Ok(config)
    } else {
        let port: u16 = env::var("AXUM_PORT").unwrap().parse().unwrap();
        let key_path: PathBuf = env::var("KEY_PATH").unwrap().parse().unwrap();
        let announce: Vec<String> = env::var("ANNOUNCE")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let bootstrap: Vec<String> = env::var("BOOTSTRAP_LIST")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let keypair = generate_secp256k1_keypair(&key_path).unwrap();
        let peer_id: PeerId = keypair.public().to_peer_id();
        let addr_str = format!("/ip4/127.0.0.1/tcp/63443/p2p/{}", peer_id);
        let _addr: Multiaddr = addr_str.parse().expect("Invalid Multiaddr");
        let listen_port_str = env::var("LIBP2P_PORT").unwrap_or_else(|_| "0".to_string());
        let listen_addr: Multiaddr = format!("/ip4/0.0.0.0/tcp/{}", listen_port_str)
            .parse()
            .expect("Invalid listen Multiaddr");

        let init_config = SynapseConfig {
            version: "0.3.0".to_string(),
            identity: IdentityConfig {
                name: "New Synapse".to_string(),
                description: "A new init Synapse config".to_string(),
                key_path: key_path,
                public_url: Url::parse("http://localhost").unwrap(),
            },
            p2p: P2pConfig {
                listen_addrs: listen_addr,
                announce: announce,
                bootstrap: bootstrap,
            },
            api: ApiConfig { port },
        };

        let json = serde_json::to_string(&init_config).unwrap();
        fs::write(&config_path, json);

        Ok(init_config)
    }
}

// TODO remove libp2p dependency for generating secp256k1 keypairs
pub fn generate_secp256k1_keypair(key_path: &PathBuf) -> Result<Keypair, CoreError> {
    let keypair = Keypair::generate_secp256k1();
    let bytes = keypair.to_protobuf_encoding().unwrap();
    let encoded = general_purpose::STANDARD.encode(bytes);
    if let Some(parent) = key_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(&key_path, encoded).unwrap();
    Ok(keypair)
}
