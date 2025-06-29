// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { initializeMessenger, sendMessage } from '../services/synapse/src/core/messenger.js';

(async () => {
    console.log('Initializing Messenger...');
    const libp2p = await initializeMessenger();

    console.log(`Synapse started. Peer ID: ${libp2p.peerId.toString()}`);
    console.log('Waiting for messages...');

    // Handle process termination (e.g., Ctrl+C)
    process.on('SIGINT', async () => {
        console.log('\nShutting down Synapse...');
        await libp2p.stop(); // Clean up libp2p instance
        process.exit(0);
    });
})();