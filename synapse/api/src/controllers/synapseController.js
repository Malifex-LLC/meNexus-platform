const { createMessage } = require('../../../../protocols/snp/messageUtils.js')
const { fetchPeerId, sendMessage, sendMessageWithResponse } = require("../../../src/messenger.js");
const { MESSAGE_TYPES} = require("../../../../protocols/snp/messageTypes.js");
const { ACTION_TYPES } = require('../../../../protocols/snp/actionTypes.js')
const { RESOURCE_TYPES} = require("../../../../protocols/snp/resourceTypes.js");


exports.getSynapseUserPosts = async (req, res) => {
    const handle = req.params.handle;
    const synapsePublicKey = req.params.publicKey;
    const peerId = await fetchPeerId(synapsePublicKey);
    if (!handle || !synapsePublicKey) {
        return res.status(401).json({error: 'No user handle or Synapse publicKey provided.'});
    }

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
        res.status(500).json({ error: 'Failed to fetch posts from the Synapse.' });
    }

}

