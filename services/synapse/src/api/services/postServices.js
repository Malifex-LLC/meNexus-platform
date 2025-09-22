import Post from '../models/post.js';
import activityController from "#api/controllers/activityController.js";
import { ACTIVITY_TYPES, OBJECT_TYPES, CONTEXT_TYPES } from '#api/config/activityConstants.js'
import broadcastController from "#api/controllers/broadcastController.js";

import path from 'path';
import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const createPost = async (publicKey, activeBoard, content) => {
    const postId = await Post.createPost(publicKey, activeBoard, content);
    const synapseConfig = await loadConfig(CONFIG_FILE)
    const activity = await activityController.createPostActivity(publicKey, postId, CONTEXT_TYPES.SYNAPSE, synapseConfig.identity.publicKey)
    console.log('activityController.createPostActivity() response: ', activity);
    broadcastController.broadcastActivity(activity);
    return postId;
}

export const updatePost = async (postId, updatedContent) => {
    return await Post.updatePost(postId, updatedContent);
}

export const deletePost = async (postId) => {
    return await Post.deletePost(postId);
}

export default {
    createPost,
    updatePost,
    deletePost
}