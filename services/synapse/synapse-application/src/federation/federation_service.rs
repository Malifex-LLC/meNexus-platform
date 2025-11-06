// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::{SynapseConfig, get_synapse_config};
use adapter_libp2p::create_libp2p_transport;
use synapse_core::errors::CoreError;

pub async fn initialize_p2p() -> Result<(), CoreError> {
    // Create transport
    let config: SynapseConfig = get_synapse_config().unwrap();
    let mut transport = create_libp2p_transport(config).await?;

    // Start - this runs the swarm event loop
    transport.start().await?;

    Ok(())
}
