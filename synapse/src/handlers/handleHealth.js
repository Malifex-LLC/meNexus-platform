import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessage } from '#core/messenger.js'
import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import * as peerStateManager from '#core/peerStateManager.js';

export const handleHealth = async (libp2p, message) => {
    switch (message.actionType) {
        case ACTION_TYPES.HEALTH.PING:
            console.log(`Received PING from ${message.meta.sender}. Sending PONG...`);
            const pongMessage = createMessage(
                MESSAGE_TYPES.HEALTH.PONG,
                ACTION_TYPES.HEALTH.PONG,
                RESOURCE_TYPES.HEALTH_STATUS,
                {},
                { sender: libp2p.peerId.toString() });
            await sendMessage(message.meta.sender, pongMessage);
            break;

        case ACTION_TYPES.HEALTH.PONG:
            console.log(`Received PONG from ${message.meta.sender}.`);
            break;

        default:
            console.warn('Unknown health actionType:', message.actionType);
            break;
    }
}