// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessageWithResponse } from '#core/messenger.js'

import * as peerStateManager from '#src/core/peerStateManager.js'

export const createRemotePostComment = async (req, res) => {
    const { resourceType, resourceId, content, publicKey, synapsePublicKey } = req.body;
    if (!resourceType || !resourceId || !content || !publicKey || !synapsePublicKey) {
        return res.status(400).json({error: 'resourceType, resourceId, content, publicKey, or synapsePublicKey not found.'});
    }
    console.log('createRemotePostComment called for synapsePublicKey: ', synapsePublicKey);
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }

    const createPostCommentRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.CREATE,
        RESOURCE_TYPES.COMMENTS,
        {
            resourceType,
            resourceId,
            content,
            publicKey
        },
        {
            sender: process.env.PUBLIC_KEY
        }
    )
    try {
        const response = await sendMessageWithResponse(peerId, createPostCommentRequest);
        res.status(200).json(response.payload.comment);
    } catch (error) {
        console.error('Error creating remote post comment', error);
        res.status(500).json({error: 'Failed to create remote post comment'});
    }
}

export const fetchRemotePostComments = async (req, res) => {
    const {resourceType, resourceId, synapsePublicKey} = req.query;
    if (!resourceType || !resourceId || !synapsePublicKey) {
        return res.status(400).json({error: 'No resourceType, resourceId, or synapsePublicKey provided.'});
    }
    console.log('fetchRemotePostsComments called: ', resourceType, resourceId, synapsePublicKey);
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }

    const commentsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.FETCH,
        RESOURCE_TYPES.COMMENTS,
        {resourceType, resourceId},
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, commentsRequest);
        res.status(200).json(response.payload.comments);
    } catch (error) {
        console.error('Error fetching comments:', error);
        res.status(500).json({error: 'Failed to fetch comments from the Synapse.'});
    }
}

export default {
    createRemotePostComment,
    fetchRemotePostComments
}