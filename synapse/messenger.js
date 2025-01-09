import { SNP_VERSION} from "../protocols/snp/index.js";
import { MESSAGE_TYPES } from '../protocols/snp/index.js'
import { createMessage, encodeMessage, decodeMessage, validateMessage} from '../protocols/snp/index.js';
import { createLibp2pInstance } from './config/libp2p.js'; // Import the configured libp2p constructor


/* Using a defined PROTOCOL_ID using the SNP_VERSION allows the libp2p to support multiple versions of SNP.
*  For example if PROTOCOL_ID = `/snp/2.0.0` libp2p could call the appropriate handler ->
*  libp2p.handle(PROTOCOL_ID, handleV1);
* */
const PROTOCOL_ID = `/snp/${SNP_VERSION}`;
let libp2p = null;

// Initialize Messenger
export const initializeMessenger = async () => {
    libp2p = await createLibp2pInstance();

    libp2p.addEventListener('peer:discovery', (event) => {
        console.log(`Discovered peer: ${event.detail.id.toString()}`);
    });

    libp2p.addEventListener('peer:connect', (event) => {
        console.log(`Connected to peer: ${event.detail.toString()}`);
    });


    // Add a handler for incoming messages
    await libp2p.handle(PROTOCOL_ID, async ({ stream, connection }) => {
        console.log(`Message received from peer: ${connection.remotePeer.toString()}`);

        const decoder = new TextDecoder();

        try {
            for await (const chunk of stream.source) {
                const rawMessage = decoder.decode(chunk);

                try {
                    const message = decodeMessage(rawMessage);
                    validateMessage(message);
                    await processMessage(message);
                } catch (error) {
                    console.error('Failed to process incoming message:', error.message);
                }
            }
        } catch (error) {
            console.error('Error reading stream:', error.message);
        }
    });

    console.log(`Handler registered for protocol: ${PROTOCOL_ID}`);

    console.log('libp2p Listening addresses:', libp2p.getMultiaddrs().map((addr) => addr.toString()));


    return libp2p;
};

// Send a message
export const sendMessage = async (peerId, message) => {
    const encodedMessage = encodeMessage(message);
    console.log(`Dialing peerId: ${peerId}`);
    const { stream } = await libp2p.dialProtocol(peerId, PROTOCOL_ID);
    const writer = stream.getWriter();
    await writer.write(encodedMessage);
    await writer.close();
};

// Process an incoming message
const processMessage = async (message) => {
    switch (message.type) {
        case MESSAGE_TYPES.HEALTH.PING:
            console.log('Received PING. Sending PONG...');
            const pongMessage = createMessage(MESSAGE_TYPES.HEALTH.PONG);
            await sendMessage(message.meta.sender, pongMessage);
            break;

        case MESSAGE_TYPES.HEALTH.PONG:
            console.log('Received PONG.');
            break;

        default:
            console.warn('Unknown message type:', message.type);
            break;
    }
};