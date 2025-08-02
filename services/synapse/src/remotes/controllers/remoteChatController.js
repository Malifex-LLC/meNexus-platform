// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessageWithResponse } from '#core/messenger.js'
import * as peerStateManager from '#src/core/peerStateManager.js'

export const fetchRemoteChannelChats = async (req, res) => {
    const { synapsePublicKey, channel } = req.query;
    if (!synapsePublicKey || !channel) {
        return res.status(401).json({error: 'No Synapse publicKey or channel provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const chatsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.CHANNEL_CHATS,
        {channel},
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, chatsRequest);

        res.status(200).json(response.payload.chats);
    } catch (err) {
        console.error('Error fetching chats: ', err);
        res.status(500).json({error: 'Failed to fetch chats from the synapse.'});
    }
}

export default {
    fetchRemoteChannelChats,
}