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
            const senderPeerId = message.meta.sender
            const publicKeyResponse = createMessage(
                MESSAGE_TYPES.PEER.RESPONSE,
                ACTION_TYPES.PEER.RESPONSE_PUBLIC_KEY,
                RESOURCE_TYPES.PEER_PUBLIC_KEY,
                {
                    publicKey: process.env.PUBLIC_KEY,
                },
                {sender: libp2p.peerId.toString()}
            );
            if (!peerStateManager.hasSentPublicKeyTo(senderPeerId)) {
                await sendMessage(senderPeerId, publicKeyResponse);
                peerStateManager.markPublicKeySentTo(senderPeerId);
            } else {
                console.log(`Already sent public key to ${senderPeerId}`);
            }
            break;

        case ACTION_TYPES.PEER.RESPONSE_PUBLIC_KEY:
            console.log('Received PEER_RESPONSE_PUBLIC_KEY: ', message.payload.publicKey);
            const peerId = message.meta.sender;
            const publicKey = message.payload.publicKey;

            const alreadyHadKey = !!peerStateManager.getPublicKeyByPeerId(peerId);
            peerStateManager.updatePeerPublicKey(peerId, publicKey);

            // 🔁 If we haven't sent our key to them yet, do it now (reciprocal send)
            if (!peerStateManager.hasSentPublicKeyTo(peerId)) {
                const reciprocal = createMessage(
                    MESSAGE_TYPES.PEER.RESPONSE,
                    ACTION_TYPES.PEER.RESPONSE_PUBLIC_KEY,
                    RESOURCE_TYPES.PEER_PUBLIC_KEY,
                    {
                        publicKey: process.env.PUBLIC_KEY
                    },
                    { sender: libp2p.peerId.toString() }
                );
                await sendMessage(peerId, reciprocal);
                peerStateManager.markPublicKeySentTo(peerId);
            }

            // ✅ Optional: if we received a response *but never sent a request*, this covers that edge
            if (!alreadyHadKey) {
                console.log(`🔁 Received public key from ${peerId}, sent ours in response.`);
            }
            break;


        default:
            console.warn('Unknown peer actionType:', message.actionType);
            break;
    }
}