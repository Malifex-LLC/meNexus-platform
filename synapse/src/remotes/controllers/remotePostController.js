import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessageWithResponse } from '#core/messenger.js'

import * as peerStateManager from '#src/core/peerStateManager.js'

export const fetchRemotePosts = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }

    const postsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.ALL_POSTS,
        {},
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, postsRequest);
        res.status(200).json(response.payload.posts);
    } catch (err) {
        console.error('Error fetching posts: ', err);
        res.status(500).json({error: 'Failed to fetch posts from the synapse.'});
    }
}

export const fetchRemoteUserPosts = async (req, res) => {
    //console.log('Discovered Peers in getSynapseUserPosts: ', peerStateManager.getAllDiscoveredPeers());
    const handle = req.query.handle;
    const synapsePublicKey = req.query.publicKey;
    if (!handle || !synapsePublicKey) {
        return res.status(401).json({error: 'No user handle or Synapse publicKey provided.'});
    }
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }
    console.log('getSynapseUserPosts handle: ', handle);
    console.log('getSynapseUserPosts synapsePublicKey: ', synapsePublicKey);
    console.log('getSynapseUserPosts peerId: ', peerId);

    const postsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.FETCH,
        RESOURCE_TYPES.ALL_POSTS,
        {
            handle
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        // Wait for the response
        const response = await sendMessageWithResponse(peerId, postsRequest);

        // Return the posts to the client
        res.status(200).json(response.payload.posts);
    } catch (err) {
        console.error('Error fetching posts:', err);
        res.status(500).json({error: 'Failed to fetch posts from the Synapse.'});
    }
}

export const createRemotePost = async (req, res) => {
    const { publicKey, content, synapsePublicKey } = req.body;
    if (!publicKey || !content || !synapsePublicKey) {
        return res.status(400).json({error: 'publicKey, synapsePublicKey, or content not found.'});
    }
    console.log('createSynapsePost called for synapsePublicKey: ', synapsePublicKey)
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(401).json({error: 'No peerId returned from peerStateManager.'});
    }

    const createPostRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.CREATE,
        RESOURCE_TYPES.POST,
        {
            publicKey,
            content
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, createPostRequest);
        res.status(200).json({ message: 'Post created successfully.', response });
    } catch (error) {
        console.error('Error in createPost:', error);
        res.status(500).json({error: 'Failed to create post.'});
    }
}



export default {
    fetchRemotePosts,
    fetchRemoteUserPosts,
    createRemotePost,
}