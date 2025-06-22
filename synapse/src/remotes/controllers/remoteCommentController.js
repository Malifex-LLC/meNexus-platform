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
        return res.status(400).json({error: 'peerId not found.'});
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
        }
    )
}

export default {
    createRemotePostComment,
}