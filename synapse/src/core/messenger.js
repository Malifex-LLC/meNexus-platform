import { SNP_VERSION } from '#protocols/snp/version.js'
import { ACTION_TYPES } from "#protocols/snp/actionTypes.js";
import { MESSAGE_TYPES } from '#protocols/snp/messageTypes.js'
import { RESOURCE_TYPES } from '#protocols/snp/resourceTypes.js'
import { createMessage, encodeMessage, decodeMessage, validateMessage} from '#protocols/snp/messageUtils.js';
import { createLibp2pInstance } from '#config/libp2p.js'; // Import the configured libp2p constructor
import { multiaddr } from 'multiaddr';
import * as peerStateManager from '#core/peerStateManager.js';
import { handleSnpMessage } from '#handlers/handleSnpMessage.js';

// console.log('peerStateManager instance:', import.meta.url);
// console.log('peerStateManager instance in messenger:', peerStateManager);

/* Using a defined PROTOCOL_ID using the SNP_VERSION allows the libp2p to support multiple versions of SNP.
*  For example if PROTOCOL_ID = `/snp/2.0.0` libp2p could call the appropriate handler ->
*  libp2p.handle(PROTOCOL_ID, handleV1);
* */
const PROTOCOL_ID = `/snp/${SNP_VERSION}`;
let libp2p = null;
const pendingRequests = new Map();

// Initialize Messenger
export const initializeMessenger = async () => {
    console.log('Initializing messenger...');
    libp2p = await createLibp2pInstance();

    libp2p.addEventListener('peer:discovery', async (event) => {
        const peerId = event.detail.id.toString();
        console.log(`Discovered peerId: ${peerId}`);
        console.log(`Local peerId: ${libp2p.peerId.toString()}`);

        // TODO Self connecting filtering does not appear to be working
        if (peerId === libp2p.peerId.toString()) {
            console.log('Skipping self-connection.');
            return;
        }
        const peerMultiaddrs = event.detail.multiaddrs
            .map((addr) => addr.toString())
            .filter(address => address.includes('/tcp/4001'));
        if (peerMultiaddrs.length === 0) {
            console.log(`Peer ${peerId} has no Messenger addresses; ignoring.`);
            return;
        }
        peerStateManager.addDiscoveredPeer(peerId, peerMultiaddrs);
        console.log('Discovered Peers: ', peerStateManager.getAllDiscoveredPeers())

        const publicKeyRequest = createMessage(
            MESSAGE_TYPES.PEER.REQUEST,
            ACTION_TYPES.PEER.REQUEST_PUBLIC_KEY,
            RESOURCE_TYPES.PEER_PUBLIC_KEY,
            {},
            {sender: libp2p.peerId.toString()}
        );
        await sendMessage(peerId, publicKeyRequest);

    });

    libp2p.addEventListener('peer:connect', (event) => {
        const peerId = event.detail.toString();
        // const peerMultiaddrs = event.detail.multiaddrs.map((addr) => addr.toString());
        // console.log(`Connected to peer: ${peerId}`);
        // peerStateManager.addConnectedPeer(peerId, peerMultiaddrs);
    });

    libp2p.addEventListener('peer:disconnect', async (event) => {
        const peerId = event.detail.toString();
        console.log(`Disconnected from peer: ${peerId}`);
        peerStateManager.removeConnectedPeer(peerId);
        peerStateManager.removeDiscoveredPeer(peerId);
    });

    // Add a handler for incoming messages
    await libp2p.handle(PROTOCOL_ID, async ({ stream, connection }) => {
        console.log(`Message received from peer: ${connection.remotePeer.toString()}`);
        //console.log('Stream', stream);
        const peerId = connection.remotePeer.toString();

        // A. single address from the connection object
        peerStateManager.mergeMultiaddrs(peerId, [connection.remoteAddr])

        const decoder = new TextDecoder();

        try {
            for await (const chunk of stream.source) {
                console.log('Raw chunk received:', chunk);

                // Convert Uint8ArrayList to a single Uint8Array
                const rawMessage = decoder.decode(chunk.subarray());
                console.log('Decoded raw message:', rawMessage);
                try {
                    const message = decodeMessage(rawMessage);
                    //console.log('Decoded message:', message);
                    validateMessage(message);
                    await handleSnpMessage(libp2p, message);
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
    // if (!peerStateManager.getConnectedPeers().has(peerId)) {
    //     console.error(`Peer ${peerId} is not currently connected.`);
    //     return;
    // }
    const peer = peerStateManager.getPeer(peerId);
    if (!peer || !peer.multiaddrs) {
        console.error(`No known peer: ${peerId}`);
        return;
    }

    const multiaddrs = peer.multiaddrs

    console.log('peer: ', peer);
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
            //console.log('Encoded message being sent:', encodedMessage);

            // Writing to the stream
            const writer = stream.sink; // Correctly use stream sink
            await writer(async function* () {
                yield new TextEncoder().encode(encodedMessage); // Encode the message as Uint8Array
            }());
            console.log('Message successfully sent to:', peerId);
            console.log('Discovered Peers after message sent: ', peerStateManager.getAllDiscoveredPeers());
            return; // Exit after successful message
        } catch (error) {
            console.error(`Failed to dial ${addr}: ${error.message}`);
        }
    }
};

export const sendMessageWithResponse = async (peerId, message, timeout = 10000) => {
    const requestId = message.meta.requestId;

    // Return a promise that tracks the response
    const responsePromise = new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
            if (pendingRequests.has(requestId)) {
                pendingRequests.delete(requestId);
                reject(new Error(`Request with ID ${requestId} timed out.`));
            }
        }, timeout);
        pendingRequests.set(requestId, { resolve, reject, timer });
    });
    try {
        await sendMessage(peerId, message);
    } catch (error) {
        pendingRequests.delete(requestId); // Clean up if the send fails
        throw error;
    }
    return responsePromise;
};

export const resolvePendingRequest = (requestId, response) => {
    if (pendingRequests.has(requestId)) {
        const { resolve, timer } = pendingRequests.get(requestId);
        clearTimeout(timer); // Clear the timeout
        resolve(response); // Resolve the promise
        pendingRequests.delete(requestId); // Clean up
    } else {
        console.warn(`No pending request found for requestId: ${requestId}`);
    }
};