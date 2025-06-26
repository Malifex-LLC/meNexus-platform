// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import * as peerStateManager from "#core/peerStateManager.js";
import {ACTION_TYPES, createMessage, MESSAGE_TYPES, RESOURCE_TYPES} from "#protocols/snp/index.js";
import {sendMessageWithResponse} from "#core/messenger.js";


export const fetchRemoteUsers = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }

    const usersRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.ALL_USERS,
        {},
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, usersRequest);
        res.status(200).json(response.payload.users);
    } catch (err) {
        console.error('Error fetching users: ', err);
        res.status(500).json({error: 'Failed to fetch users from the synapse.'});
    }
}

export default {
    fetchRemoteUsers,
}