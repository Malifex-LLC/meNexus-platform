// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessageWithResponse } from '#core/messenger.js'
import * as peerStateManager from '#src/core/peerStateManager.js'

export const fetchRemoteReactions = async (req, res) => {
    const { resourceType, resourceId, synapsePublicKey } = req.query;
    if (!resourceType || !resourceId || !synapsePublicKey) {
        return res.status(401).json({error: 'No resourceType, resourceId, or synapsePublicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const fetchReactionsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.AGGREGATE,
        RESOURCE_TYPES.REACTIONS,
        {
            resourceType,
            resourceId
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, fetchReactionsRequest);
        res.status(200).json({ message: 'Reactions retrieved successfully.', response });
    } catch (error) {
        console.error('Error in fetchRemoteReactions:', error);
        res.status(500).json({error: 'Failed to fetch reactions.'});
    }
}

export const createRemoteReaction = async (req, res) => {
    const { publicKey, resourceType, resourceId, reactionType, synapsePublicKey } = req.query;
    if (!publicKey || !resourceType || !resourceId || !reactionType || !synapsePublicKey) {
        return res.status(401).json({error: 'No publicKey, resourceType, resourceId, or synapsePublicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const createReactionRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.CREATE,
        RESOURCE_TYPES.REACTIONS,
        {
            publicKey,
            resourceType,
            resourceId,
            reactionType
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, createReactionRequest);
        res.status(200).json({ message: 'Reaction created successfully.', response });
    } catch (error) {
        console.error('Error in createRemoteReaction:', error);
        res.status(500).json({error: 'Failed to create reaction.'});
    }
}

export const deleteRemoteReaction = async (req, res) => {
    const { publicKey, resourceType, resourceId, reactionType, synapsePublicKey } = req.query;
    if (!publicKey || !resourceType || !resourceId || !reactionType || !synapsePublicKey) {
        return res.status(401).json({error: 'No publicKey, resourceType, resourceId, or synapsePublicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const deleteReactionRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.DELETE,
        RESOURCE_TYPES.REACTIONS,
        {
            publicKey,
            resourceType,
            resourceId,
            reactionType
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, deleteReactionRequest);
        res.status(200).json({ message: 'Reaction deleted successfully.', response });
    } catch (error) {
        console.error('Error in deleteRemoteReaction:', error);
        res.status(500).json({error: 'Failed to create reaction.'});
    }
}

export default {
    fetchRemoteReactions,
    createRemoteReaction,
    deleteRemoteReaction,
}