import { initializeSnpPubSub } from '../protocols/snp/snpPubSub.js';
import { createLibp2pInstance } from '../synapse/config/libp2p.js';
import { MESSAGE_TYPES } from '../protocols/snp/messageTypes.js';
import { ACTION_TYPES} from "../protocols/snp/actionTypes.js";

(async () => {
    // Step 1: Initialize libp2p and snpPubSub
    const libp2pInstance = await createLibp2pInstance();
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

    // Step 4: Publish a test message
    await publish(testTopic, testMessage);

    console.log(`Published test message to topic "${testTopic}"`);
})();
