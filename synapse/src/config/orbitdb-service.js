// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { createOrbitDB } from '@orbitdb/core';
import { createHelia } from 'helia';
import { createLibp2p } from 'libp2p';
import { LevelBlockstore } from 'blockstore-level'
import { bitswap } from '@helia/block-brokers'
import { gossipsub } from '@chainsafe/libp2p-gossipsub';
import { Identities, KeyStore } from '@orbitdb/core'
import { identify } from '@libp2p/identify';
import { tcp } from '@libp2p/tcp';
import { webSockets } from '@libp2p/websockets';
import { noise } from '@chainsafe/libp2p-noise'; // For encryption
import { mplex } from '@libp2p/mplex';
import { mdns } from '@libp2p/mdns';
import { bootstrap } from '@libp2p/bootstrap';
import path from 'path';
import { fileURLToPath } from 'url';

let orbitdbInstance = null; // To hold the OrbitDB instance
let ipfsInstance = null;
let databases = {}; // To store references to opened databases

// Initialize OrbitDB and Libp2p/Helia
export async function initializeOrbitDB() {
    if (orbitdbInstance) {
        console.log('OrbitDB already initialized');
        return orbitdbInstance;
    }
    console.log('Initializing OrbitDB...');

    const libp2p = await createLibp2p({
        addresses: {
            listen: [
                '/ip4/0.0.0.0/tcp/4002', // Listen on a TCP port
                '/ip4/0.0.0.0/udp/0/quic-v1', // Listen on a random UDP port for QUIC
            ]
        },
        transports: [
            tcp(), // Add TCP transport
            webSockets() // Optionally add WebSockets for browser support
        ],
        connectionEncrypters: [
            noise() // Ensure Noise is included for encryption
        ],
        streamMuxers: [
            mplex()
        ], // Use mplex for stream multiplexing, // Use mplex for stream multiplexing
        services: {
            pubsub: gossipsub({
                allowPublishToZeroTopicPeers: true, // Allow single peer for testing
            }),
            identify: identify(),
        },
        peerDiscovery: [
            bootstrap({
                list: [
                    '/ip4/192.168.1.60/tcp/4002/p2p/12D3KooWQgHV3CqJV5oJctAe97eNqVPU4cxzJkopYyQPw7BbAWRc',
                    '/ip4/192.168.1.188/tcp/4002/p2p/12D3KooWPWtGnKmxzX7KqJp7pLr5JHMaUhokkry8TfUwvh2EmEVK',
                ],
            }),
            mdns({
                interval: 2000,
            }),
        ],
    });
    console.log('Listening addresses:', libp2p.getMultiaddrs().map(addr => addr.toString()));



    const __filename = fileURLToPath(import.meta.url);
    const __dirname = path.dirname(__filename);

// Absolute paths to orbitdb folder
    const directory = path.resolve(__dirname, '../orbitdb');
    const keystorePath = path.resolve(directory, 'keystore');

    const blockstore = new LevelBlockstore(`${directory}/ipfs/blocks`)
    const keystore = await KeyStore({ path: keystorePath });

    ipfsInstance = await createHelia({ libp2p, blockstore, blockBrokers: [bitswap()] })

    const id = 'meNexus-orbitdb'
    const identities = await Identities({ keystore, ipfs: ipfsInstance });
    const identity = identities.createIdentity({ id })

    orbitdbInstance = await createOrbitDB({
        ipfs: ipfsInstance,
        identities: identities,
        id: id,
        directory: directory,
    });
    console.log('OrbitDB initialized');

    // Log when a peer connects
    libp2p.addEventListener('peer:connect', ({ detail }) => {
        console.log('Connected to peer:', detail)
    })

    // Log when a peer disconnects
    libp2p.addEventListener('peer:disconnect', ({ detail }) => {
        console.log('Disconnected from peer:', detail)
    })

    console.log('Connected peers:', libp2p.getPeers());

    //
    // console.log('Database Address:', db.address);
    console.log('Identity:', orbitdbInstance.identity);

    return orbitdbInstance;
}

export function getIPFSInstance() {
    if (!ipfsInstance) {
        throw new Error('IPFS instance is not initialized. Call initializeOrbitDB first.');
    }
    return ipfsInstance;
}

// Open or create a orbitdb
export async function getDatabase(nameOrAddr, type = 'documents', options = {}) {
    if (!orbitdbInstance) {
        throw new Error('OrbitDB is not initialized. Call initializeOrbitDB first.');
    }

    if (databases[nameOrAddr]) {
        console.log(`Database "${nameOrAddr}" already opened.`);
        return databases[nameOrAddr];
    }

    console.log(`Opening database "${nameOrAddr}" of type "${type}"...`);
    const db = await orbitdbInstance.open(nameOrAddr, { type, ...options });
    databases[nameOrAddr] = db; // Store reference to the orbitdb
    console.log('Databases object:', databases);

    db.events.on('replicate', (address) => {
        console.log(`[Replicate] Starting replication for database: ${address}`);
    });

    db.events.on('replicate.progress', (address, hash, entry, progress, have) => {
        console.log(`[Replicate Progress] Database: ${address}`);
        console.log(`  - Hash: ${hash}`);
        console.log(`  - Entry:`, entry);
        console.log(`  - Progress: ${progress}`);
        console.log(`  - Have: ${have}`);
    });

    db.events.on('replicated', (address) => {
        console.log(`[Replicated] Replication complete for database: ${address}`);
    });

    db.events.on('update', (entry) => {
        console.log(`[Update] New entry received:`, entry);
    });

    return db;
}

// Close a specific orbitdb
export async function closeDatabase(name) {
    if (databases[name]) {
        console.log(`Closing database "${name}"...`);
        await databases[name].close();
        delete databases[name];
    }
}

// Close all databases and OrbitDB
export async function closeOrbitDB() {
    console.log('Closing all databases and OrbitDB...');
    for (const name in databases) {
        await databases[name].close();
    }
    databases = {};

    if (orbitdbInstance) {
        await orbitdbInstance.stop();
        orbitdbInstance = null;
        console.log('OrbitDB stopped');
    }
}