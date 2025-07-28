// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessageWithResponse } from '#core/messenger.js'
import * as peerStateManager from '#src/core/peerStateManager.js'
import Busboy from 'busboy';

export const fetchRemotePosts = async (req, res) => {
    const synapsePublicKey = req.query.synapsePublicKey;
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

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

export const fetchRemoteBoardPosts = async (req, res) => {
    const { synapsePublicKey, board } = req.query;
    if (!synapsePublicKey || !board) {
        return res.status(401).json({error: 'No Synapse publicKey or board provided.'});
    }
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const postsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.BOARD_POSTS,
        {board},
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
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;
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
    const { publicKey, activeBoard, content, synapsePublicKey } = req.body;
    if (!publicKey || !activeBoard || !content || !synapsePublicKey) {
        return res.status(400).json({error: 'publicKey, activeBoard, content, or synapsePublicKey not found.'});
    }
    console.log('createSynapsePost called for synapsePublicKey: ', synapsePublicKey)
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const createPostRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.CREATE,
        RESOURCE_TYPES.POST,
        {
            publicKey,
            activeBoard,
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

export const uploadRemotePostMedia = async (req, res) => {
    try {
        const busboy = Busboy({ headers: req.headers });

        let synapsePublicKey = '';
        let publicKey = '';
        let postId = '';
        let filename = '';
        let mimetype = '';
        let fileBuffer = Buffer.alloc(0);

        busboy.on('field', (fieldname, val) => {
            const value = typeof val === 'string' ? val : val.toString();
            if (fieldname === 'synapsePublicKey') synapsePublicKey = value;
            if (fieldname === 'publicKey') publicKey = value;
            if (fieldname === 'postId') postId = value;
        });

        busboy.on('file', (fieldname, file, info) => {
            if (fieldname !== 'post_media') {
                file.resume(); // Ignore other files
                return;
            }

            filename = info.filename;
            mimetype = info.mimeType;

            file.on('data', (chunk) => {
                fileBuffer = Buffer.concat([fileBuffer, chunk]);
            });
        });

        busboy.on('finish', async () => {
            if (!publicKey || !postId || !filename || !fileBuffer.length) {
                return res.status(400).json({ error: 'Missing file or required metadata.' });
            }

            // Encode file as base64 (or keep as Buffer for SNP if you're handling raw buffers)
            const encodedFile = fileBuffer.toString('base64');

            // Construct SNP payload
            const remotePostMediaRequest = {
                postId,
                publicKey,
                filename,
                mimetype,
                file: encodedFile
            };

            const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
            if (!peer || !peer.peerId) {
                return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
            }
            const { peerId } = peer;

            const uploadRemotePostMediaRequest = createMessage(
                MESSAGE_TYPES.DATA.REQUEST,
                ACTION_TYPES.RESOURCE.CREATE,
                RESOURCE_TYPES.MEDIA,
                remotePostMediaRequest,
                {sender: process.env.PUBLIC_KEY}
            );

            console.log("Sending uploadRemotePostMediaRequest: ", uploadRemotePostMediaRequest);
            try {
                const response = await sendMessageWithResponse(peerId, uploadRemotePostMediaRequest);
                res.status(200).json({ message: 'Remote post media uploaded successfully.', response });
            } catch (error) {
                console.error('Error in uploadRemotePostMedia:', error);
                res.status(500).json({error: 'Failed to upload post media.'});
            }
        });

        req.pipe(busboy);
    } catch (err) {
        console.error('Error in uploadRemotePostMedia:', err);
        return res.status(500).json({ error: 'Unexpected server error.' });
    }
};

export const updateRemotePost = async (req, res) => {
    const {postId, content, synapsePublicKey} = req.body;

    if (!postId || !content || !synapsePublicKey) {
        return res.status(400).json({error: 'postId or content not found.'});
    }
    console.log('updateRemotePost called for synapsePublicKey: ', synapsePublicKey)
    const peer = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peer || !peer.peerId) {
        return res.status(401).json({ error: 'No peerId returned from peerStateManager.' });
    }
    const { peerId } = peer;

    const createPostRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.UPDATE,
        RESOURCE_TYPES.POST,
        {
            postId,
            content
        },
        {sender: process.env.PUBLIC_KEY}
    )
    try {
        const response = await sendMessageWithResponse(peerId, createPostRequest);
        res.status(200).json({ message: 'Post updated successfully.', response });
    } catch (error) {
        console.error('Error in updateRemotePost:', error);
        res.status(500).json({error: 'Failed to update post.'});
    }

}




export default {
    fetchRemotePosts,
    fetchRemoteBoardPosts,
    fetchRemoteUserPosts,
    createRemotePost,
    uploadRemotePostMedia,
    updateRemotePost
}