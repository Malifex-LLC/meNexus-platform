// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessage } from '#core/messenger.js'
import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import * as peerStateManager from '#core/peerStateManager.js';

export const handlePeer = async (libp2p, message) => {
    switch (message.actionType) {
        case ACTION_TYPES.PEER.REQUEST_PUBLIC_KEY:
            console.log('Received publicKey request')
            const publicKeyResponse = createMessage(
                MESSAGE_TYPES.PEER.RESPONSE,
                ACTION_TYPES.PEER.RESPONSE_PUBLIC_KEY,
                RESOURCE_TYPES.PEER_PUBLIC_KEY,
                {publicKey: process.env.PUBLIC_KEY},
                {sender: libp2p.peerId.toString()}
            );
            await sendMessage(message.meta.sender, publicKeyResponse);
            break;

        case ACTION_TYPES.PEER.RESPONSE_PUBLIC_KEY:
            console.log('Received PEER_RESPONSE_PUBLIC_KEY: ', message.payload.publicKey);
            const peerId = message.meta.sender;
            const publicKey = message.payload.publicKey;
            peerStateManager.updatePeerPublicKey(peerId, publicKey);
            break;

        default:
            console.warn('Unknown peer actionType:', message.actionType);
            break;
    }
}