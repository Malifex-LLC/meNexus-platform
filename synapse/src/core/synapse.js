// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import fs from 'fs/promises';
import path from 'path';
import { exec, spawn } from 'child_process';
import dotenv from 'dotenv';
dotenv.config({ path: '#api/config/.env' }); // Load base environment variables
import { generateCryptoKeysUtil } from '#utils/cryptoUtils.js'; // Utility for keypair generation
import { loadConfig, saveConfig } from '#utils/configUtils.js'; // Config file handlers
import { initializeMessenger} from "./messenger.js";
import { initializeSnpPubSub} from "./snpPubSub.js";

// Await ensures .env is loaded before server.js runs...was causing mySQL to fail to connect before
const { startApi } = await import('#api/server.js');
const CONFIG_FILE = path.join(process.cwd(), '../config/synapse-config.json');



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
        config = {
            identity: { publicKey, privateKey },
            api: { port: process.env.EXPRESS_PORT || 4000 },
            metadata: { name: 'DefaultSynapse', description: 'A new Synapse instance' }
        };
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
