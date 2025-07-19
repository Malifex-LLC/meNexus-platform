// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { createLibp2p } from 'libp2p';
import { tcp } from '@libp2p/tcp';
import { webSockets } from '@libp2p/websockets';
import { noise } from '@chainsafe/libp2p-noise';
import { mplex } from '@libp2p/mplex';
import { gossipsub } from '@chainsafe/libp2p-gossipsub';
import { bootstrap } from '@libp2p/bootstrap';
import { mdns } from '@libp2p/mdns';
import { identify } from '@libp2p/identify';
import { kadDHT } from '@libp2p/kad-dht';
import { ping } from '@libp2p/ping';
import { autoNAT } from '@libp2p/autonat';
import { createFromJSON, createFromPrivKey } from '@libp2p/peer-id-factory';

import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, './synapse-config.json');

// Configure libp2p2Instance for use by Synapse
export const createLibp2pInstance = async () => {
    const synapseConfig = await loadConfig(CONFIG_FILE)
    let peerId;
    try {
        peerId = await createFromJSON(synapseConfig.identity.peerId)
        console.log('Created peerId from JSON: %s', peerId)
    } catch (err) {
        console.warn('[WARN] Failed to load peerId from config')
    }

    return await createLibp2p({
        peerId,
        addresses: {
            listen: synapseConfig.libp2p.multiaddrs,
            announce: synapseConfig.libp2p.announce,
        },

        transports: [
            tcp(),
            //webSockets()
        ],
        connectionEncrypters: [noise()],
        streamMuxers: [mplex()],
        services: {
            pubsub: gossipsub({
                emitSelf: false,
                gossipInterval: 2000,
                heartbeatInterval: 1000,
            }),
            identify: identify(),
            ping: ping(),
            dht: kadDHT({
                protocolPrefix: '/menexus',
                clientMode: false,
            }),
            autonat: autoNAT(),
        },
        peerDiscovery: [
            bootstrap({
                list: synapseConfig.libp2p.bootstrapPeers
            }),
            mdns({
                interval: 2000,
            }),
        ],
        connectionManager: {
            minConnections: 5,
            maxConnections: 20,
        },
        dialer: {
            maxParallelDials: 10,
            maxDialsPerPeer: 2,
            dialTimeout: 30000,
        },
        relay: {
            enabled: true,
            hop: {
                enabled: true,
            },
        },
        nat: {
            enabled: true,
            externalPort: 4001,
            description: 'meNexus Synapse'
        },
    });
};
