import { SNP_VERSION} from "../protocols/snp/index.js";
import { MESSAGE_TYPES } from '../protocols/snp/index.js'
import { createMessage, encodeMessage, decodeMessage, validateMessage} from '../protocols/snp/index.js';
import { createLibp2pInstance } from './config/libp2p.js'; // Import the configured libp2p constructor
import { multiaddr } from 'multiaddr';


/* Using a defined PROTOCOL_ID using the SNP_VERSION allows the libp2p to support multiple versions of SNP.
*  For example if PROTOCOL_ID = `/snp/2.0.0` libp2p could call the appropriate handler ->
*  libp2p.handle(PROTOCOL_ID, handleV1);
* */
const PROTOCOL_ID = `/snp/${SNP_VERSION}`;
let libp2p = null;
const discoveredPeers = new Map();
const connectedPeers = new Set();



// Initialize Messenger
export const initializeMessenger = async () => {
    libp2p = await createLibp2pInstance();

    libp2p.addEventListener('peer:discovery', (event) => {
        const peerId = event.detail.id.toString();
        const peerMultiaddrs = event.detail.multiaddrs.map((addr) => addr.toString());
        if (!discoveredPeers.has(peerId)) {
            console.log(`Discovered peer: ${peerId} at ${peerMultiaddrs}`);
            discoveredPeers.set(peerId, peerMultiaddrs); // Store peerId and its multiaddrs
        }
    });

    libp2p.addEventListener('peer:connect', (event) => {
        const peerId = event.detail.toString();
        console.log(`Connected to peer: ${peerId}`);
        connectedPeers.add(peerId);
    });

    libp2p.addEventListener('peer:connect', async (event) => {
        const peerId = event.detail.toString();
        console.log(`Auto-pinging connected peer: ${peerId}`);

        const pingMessage = createMessage(MESSAGE_TYPES.HEALTH.PING, {}, { sender: libp2p.peerId.toString() });

        await sendMessage(peerId, pingMessage);
    });

    libp2p.addEventListener('peer:disconnect', (event) => {
        const peerId = event.detail.toString();
        console.log(`Disconnected from peer: ${peerId}`);
        connectedPeers.delete(peerId);
    });


    // Add a handler for incoming messages
    await libp2p.handle(PROTOCOL_ID, async ({ stream, connection }) => {
        console.log(`Message received from peer: ${connection.remotePeer.toString()}`);
        //console.log('Stream', stream);

        const decoder = new TextDecoder();

        try {
            for await (const chunk of stream.source) {
                console.log('Raw chunk received:', chunk);
                const rawMessage = decoder.decode(chunk);
                console.log('Decoded raw message:', rawMessage);

                try {
                    const message = decodeMessage(rawMessage);
                    console.log('Decoded message:', message);
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
    console.log(`Sending message to peer: ${peerId}`);
    const multiaddrs = discoveredPeers.get(peerId);

    if (!multiaddrs || multiaddrs.length === 0) {
        console.error(`No known multiaddrs for peer: ${peerId}`);
        return;
    }

    for (const addr of multiaddrs) {
        try {
            const peerAddress = multiaddr(addr);
            console.log(`Dialing peerId: ${peerId} at: ${peerAddress.toString()}`);
            const stream = await libp2p.dialProtocol(peerAddress, PROTOCOL_ID);

            if (!stream || stream.status !== 'open') {
                console.error(`Failed to establish a stream for peer ${peerId} at ${peerAddress}`);
                continue; // Try the next address
            }

            //console.log('Stream established:', stream);

            const encodedMessage = encodeMessage(message);
            console.log('Encoded message being sent:', encodedMessage);

            // Writing to the stream
            const writer = stream.sink; // Correctly use stream sink
            await writer(async function* () {
                yield new TextEncoder().encode(encodedMessage); // Encode the message as Uint8Array
            }());

            console.log('Message successfully sent to:', peerId);
            return; // Exit after successful message
        } catch (error) {
            console.error(`Failed to dial ${addr}: ${error.message}`);
        }
    }
};

// Process an incoming message
const processMessage = async (message) => {
    console.log(`Processing message: ${message}`);
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