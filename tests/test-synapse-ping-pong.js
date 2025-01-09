import { initializeMessenger, sendMessage } from '../synapse/messenger.js';
import { createMessage,  } from '../protocols/snp/index.js';
import { MESSAGE_TYPES } from '../protocols/snp/index.js';

(async () => {
    console.log('Initializing Messenger...');
    const libp2p = await initializeMessenger();

    console.log(`Synapse started. Peer ID: ${libp2p.peerId.toString()}`);
    console.log('Waiting for messages...');

    // const targetPeerId = '12D3KooWJpf1BwUkrde2Z2zde7ncWu3h5pSCBAT3AiJWtuCSVNkt'; // Replace with actual Peer ID
    // if (targetPeerId) {
    //     console.log(`Sending PING to ${targetPeerId}`);
    //     const pingMessage = createMessage(MESSAGE_TYPES.HEALTH.PING, {}, { sender: libp2p.peerId.toString() });
    //     await sendMessage(targetPeerId, pingMessage);
    // }

    // Handle process termination (e.g., Ctrl+C)
    process.on('SIGINT', async () => {
        console.log('\nShutting down Synapse...');
        await libp2p.stop(); // Clean up libp2p instance
        process.exit(0);
    });
})();