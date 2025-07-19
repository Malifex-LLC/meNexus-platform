// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import fs from 'fs/promises';
import { exec, spawn } from 'child_process';
import dotenv from 'dotenv';
dotenv.config(); // Load base environment variables
import { generateCryptoKeysUtil } from '#utils/cryptoUtils.js'; // Utility for keypair generation
import { initializeMessenger} from "./messenger.js";
import { initializeSnpPubSub} from "./snpPubSub.js";
import { createSecp256k1PeerId, exportToProtobuf } from '@libp2p/peer-id-factory';
import { base64pad } from 'multiformats/bases/base64'; // needed to encode the keys


import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../config/synapse-config.json');

// Await ensures .env is loaded before server.js runs...was causing mySQL to fail to connect before
const { startApi } = await import('#api/server.js');



// Initialize the Synapse instance
const initializeSynapse = async () => {
    console.log('Initializing Synapse...');
    let config;

    // Load or generate configuration
    try {
        config = await loadConfig(CONFIG_FILE);
        console.log('Loaded existing configuration:', config);
    } catch (error) {
        console.warn('No configuration found. Generating default configuration...');
        const { publicKey, privateKey } = await generateCryptoKeysUtil();
        const peerId = await createSecp256k1PeerId();
        const peerIdJson = {
            id: peerId.toString(),
            publicKey: base64pad.encode(peerId.publicKey),   // Uint8Array → base64 string
            privateKey: base64pad.encode(peerId.privateKey) // needs non-null assertion
        };
        config = {
            identity: {
                publicKey,
                privateKey,
                peerId: peerIdJson
            },
            api: { port: process.env.EXPRESS_PORT || 3001 },
            metadata: { name: 'DefaultSynapse', description: 'A new Synapse instance' }
        };
        console.log('peerId JSON:', peerId.toJSON());
        console.log('config.identity.peerId BEFORE save:', config.identity.peerId);
        await saveConfig(CONFIG_FILE, config);
        console.log('Generated and saved new configuration:', config);
    }

    // Set environment variables for the Synapse
    process.env.PUBLIC_KEY = config.identity.publicKey;
    process.env.PRIVATE_KEY = config.identity.privateKey;
    process.env.EXPRESS_PORT = config.api.port;

    await startServer();
    await startSnpPubSub();
};

const startServer = async () => {
    console.log('Starting API server...');
    await startApi();
};

const startSnpPubSub = async () => {
    console.log('Starting snpPubSub...');
    const libp2pInstance = await initializeMessenger();
    await initializeSnpPubSub(libp2pInstance);
}

// Run the initialization
initializeSynapse().catch((err) => {
    console.error('Failed to initialize Synapse:', err);
});
