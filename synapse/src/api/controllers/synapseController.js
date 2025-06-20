import { createMessage } from '#protocols/snp/messageUtils.js'
import { sendMessageWithResponse } from "#core/messenger.js";
import { MESSAGE_TYPES} from "#protocols/snp/messageTypes.js";
import { ACTION_TYPES } from '#protocols/snp/actionTypes.js'
import { RESOURCE_TYPES} from "#protocols/snp/resourceTypes.js";
import { loadConfig, saveConfig } from '#utils/configUtils.js';
const CONFIG_FILE = '#config/synapse-config.json';
import * as peerStateManager from '#src/core/peerStateManager.js'
import path from "path";
import {getGlobalUsersDB} from "#src/orbitdb/globalUsers.js";
import Post from "#src/api/models/post.js";


// console.log('peerStateManager instance:', import.meta.url);
// console.log('peerStateManager instance in messenger:', peerStateManager);

// TODO Need to fix so that the privateKey is not included in the Synapse Metadata returned
export const getSynapseMetadata = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    console.log('getSynapseMetadata for synapsePublicKey: ', synapsePublicKey)
    const localMetadata = await loadConfig(CONFIG_FILE);
    if (synapsePublicKey === localMetadata.identity.publicKey) {
        return res.status(200).json(localMetadata);
    }
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }
    const metadataRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.SYNAPSE_METADATA,
        {},
        {sender: process.env.PUBLIC_KEY}
    );
    try {
        const response = await sendMessageWithResponse(peerId, metadataRequest);
        res.status(200).json(response.payload.metadata);
    } catch (err) {
        console.error('Error fetching Synapse metadata:', err);
        res.status(500).json({error: 'Failed to fetch metadata from the synapse.'});
    }
}

export const getLocalSynapseMetadata = async (req, res) => {
    try {
        const metadata = await loadConfig(CONFIG_FILE);
        res.status(200).json(metadata);
    } catch (err) {
        console.error('Error fetching Local Synapse Metadata:', err);
        res.status(500).json({error: 'Failed to fetch Metadata from the Synapse '});
    }
}

export const getSynapseUsers = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
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

export const getSynapsePosts = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }

    const resource = RESOURCE_TYPES.ALL_POSTS;

    const postsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        RESOURCE_TYPES.ALL_POSTS,
        {
            resource,
        },
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


export const getSynapseUserPosts = async (req, res) => {
    //console.log('Discovered Peers in getSynapseUserPosts: ', peerStateManager.getAllDiscoveredPeers());
    const handle = req.query.handle;
    const synapsePublicKey = req.query.publicKey;
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!handle || !synapsePublicKey) {
        return res.status(401).json({error: 'No user handle or Synapse publicKey provided.'});
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

export const createSynapsePost = async (req, res) => {
    const { publicKey, content, synapsePublicKey } = req.body;
    if (!publicKey || !content || !synapsePublicKey) {
        return res.status(400).json({error: 'publicKey or content not found.'});
    }
    console.log('createSynapsePost called for synapsePublicKey: ', synapsePublicKey)
    const { peerId } = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!peerId) {
        return res.status(400).json({error: 'peerId not found.'});
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

export const joinSynapse = async (req, res) => {
    const {publicKey, synapsePublicKey} = req.query;
    if (!publicKey || !synapsePublicKey) {
        return res.status(401).json({error: 'No user publicKey or Synapse publicKey provided.'});
    }
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if(updatedUser) {
        try {
            updatedUser.synapses.push(synapsePublicKey);
            await db.put(updatedUser);
            res.status(200).json(updatedUser);
        } catch (err) {
            console.error('Error joining Synapse: ', err);
        }
    }
}

export const leaveSynapse = async (req, res) => {

}

export default {
    getSynapseMetadata,
    getLocalSynapseMetadata,
    getSynapseUsers,
    getSynapsePosts,
    getSynapseUserPosts,
    createSynapsePost,
    joinSynapse,
    leaveSynapse
}