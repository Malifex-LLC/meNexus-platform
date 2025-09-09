// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import dotenv from 'dotenv';
dotenv.config(); // Load base environment variables
import { initializeMessenger} from "./messenger.js";
import { initializeSnpPubSub} from "./snpPubSub.js";
import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';

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
        console.warn('No synapse-config.json found. Run pnpm run init:synapse-config:local and retry.');
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
