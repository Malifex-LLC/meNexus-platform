// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { initializeSnpPubSub } from '#src/snpPubSub.js';
import { createLibp2pInstance } from '#config/libp2p.js';
import { MESSAGE_TYPES } from '#protocols/snp/messageTypes.js';
import { ACTION_TYPES} from "#protocols/snp/actionTypes.js";
import { initializeMessenger} from "#src/messenger.js";

(async () => {
    // Step 1: Initialize libp2p and snpPubSub
    const libp2pInstance = await initializeMessenger();
    const { publish, subscribe } = await initializeSnpPubSub(libp2pInstance);

    // Step 2: Define test topic and message
    const testTopic = 'test:echo';
    const testMessage = {
        type: MESSAGE_TYPES.HEALTH.ECHO,
        action: ACTION_TYPES.HEALTH.ECHO,
        payload: { content: 'Hello, world!' }
    };

    // Step 3: Subscribe to the test topic
    subscribe(testTopic, (message) => {
        console.log(`Message received on topic "${testTopic}":`, message);
    });

    // Step 4: Publish the test message every 10 seconds
    setInterval(async () => {
        await publish(testTopic, testMessage);
        console.log(`Published test message to topic "${testTopic}"`);
    }, 10000); // 10 seconds in milliseconds

    // Handle process termination (e.g., Ctrl+C)
    process.on('SIGINT', async () => {
        console.log('\nShutting down Synapse...');
        await libp2pInstance.stop(); // Clean up libp2p instance
        process.exit(0);
    });
})();
