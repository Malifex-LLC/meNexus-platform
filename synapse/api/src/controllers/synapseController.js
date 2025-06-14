import { createMessage } from '#protocols/snp/messageUtils.js'
import { sendMessageWithResponse } from "#src/messenger.js";
import { MESSAGE_TYPES} from "#protocols/snp/messageTypes.js";
import { ACTION_TYPES } from '#protocols/snp/actionTypes.js'
import { RESOURCE_TYPES} from "#protocols/snp/resourceTypes.js";
import * as peerStateManager from '#src/peerStateManager.js'

// console.log('peerStateManager instance:', import.meta.url);
// console.log('peerStateManager instance in messenger:', peerStateManager);

export const getSynapseUsers = async (req, res) => {
    const synapsePublicKey = req.query.publicKey;
    const {peerId} = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!synapsePublicKey) {
        return res.status(401).json({error: 'No Synapse publicKey provided.'});
    }

    const resource = RESOURCE_TYPES.ALL_USERS;

    const usersRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.DATA.QUERY,
        {
            resource,
        },
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

export const getSynapseUserPosts = async (req, res) => {
    //console.log('Discovered Peers in getSynapseUserPosts: ', peerStateManager.getAllDiscoveredPeers());
    const handle = req.query.handle;
    const synapsePublicKey = req.query.publicKey;
    const {peerId} = peerStateManager.getPeerByPublicKey(synapsePublicKey);
    if (!handle || !synapsePublicKey) {
        return res.status(401).json({error: 'No user handle or Synapse publicKey provided.'});
    }
    console.log('getSynapseUserPosts handle: ', handle);
    console.log('getSynapseUserPosts synapsePublicKey: ', synapsePublicKey);
    console.log('getSynapseUserPosts peerId: ', peerId);

    const resource = RESOURCE_TYPES.ALL_POSTS

    const postsRequest = createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        ACTION_TYPES.RESOURCE.FETCH,
        {
            resource,
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

export default {
    getSynapseUsers,
    getSynapseUserPosts,
}