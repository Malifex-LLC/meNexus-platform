// Synapse.js
import fs from 'fs/promises';
import path from 'path';
import { exec, spawn } from 'child_process';
import dotenv from 'dotenv';
dotenv.config({ path: '../api/config/.env' }); // Load base environment variables
import { generateCryptoKeys } from '../utils/cryptoUtils.js'; // Utility for keypair generation
import { loadConfig, saveConfig } from '../utils/configUtils.js'; // Config file handlers
import { initializeMessenger} from "./messenger.js";
import { initializeSnpPubSub} from "./snpPubSub.js";

const CONFIG_FILE = path.join(process.cwd(), '../config/synapse-config.json');

// Initialize the Synapse instance
const initializeSynapse = async () => {
    let config;

    // Load or generate configuration
    try {
        config = await loadConfig(CONFIG_FILE);
        console.log('Loaded existing configuration:', config);
    } catch (error) {
        console.warn('No configuration found. Generating default configuration...');
        const { publicKey, privateKey } = await generateCryptoKeys();
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

const startServer = () => {
    console.log('Starting Synapse...');
    const serverProcess = spawn('node', ['server.js'], { cwd: '../api/src', stdio: 'inherit' });

    serverProcess.on('error', (error) => {
        console.error(`Error starting server: ${error.message}`);
    });

    serverProcess.on('exit', (code, signal) => {
        if (code !== 0) {
            console.error(`Server process exited with code: ${code}, signal: ${signal}`);
        } else {
            console.log('Server process exited successfully.');
        }
    });
};

const startSnpPubSub = async () => {
    const libp2pInstance = await initializeMessenger();
    await initializeSnpPubSub(libp2pInstance);
}

// Run the initialization
initializeSynapse().catch((err) => {
    console.error('Failed to initialize Synapse:', err);
});
