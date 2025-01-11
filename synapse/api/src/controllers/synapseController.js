import Post from '../models/Post';
import { createMessage } from '../../../../protocols/snp/messageUtils.js'
import { fetchPeerId, sendMessage } from "../../../src/messenger";
import { MESSAGE_TYPES} from "../../../../protocols/snp/messageTypes.js";
import { ACTION_TYPES } from '../../../../protocols/snp/actionTypes.js'

exports.getSynapseUserPosts = async (req, res) => {
    const handle = req.params.handle;
    const synapsePublicKey = req.params.publicKey;
    if (!handle || !synapsePublicKey) {
        return res.status(401).json({error: 'No user handle or Synapse publicKey provided.'});
    }

    createMessage(
        MESSAGE_TYPES.DATA.REQUEST,
        {},

    )

    try {
        const posts = await Post.getUserPosts(handle);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getUserPosts:', error);
        res.status(500).json({error: 'Failed to get user posts.'});
    }
}

