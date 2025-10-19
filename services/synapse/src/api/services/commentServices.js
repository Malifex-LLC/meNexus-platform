import Comment from '../models/comment.js';
import activityController from '../controllers/activityController.js';
import { ACTIVITY_TYPES, OBJECT_TYPES, CONTEXT_TYPES } from '#api/config/activityConstants.js'

import path from 'path';
import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";
import broadcastController from "#api/controllers/broadcastController.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const createComment = async (resourceType, resourceId, content, publicKey) => {
    const result = Comment.createComment(resourceType, resourceId, content, publicKey);
    const synapseConfig = await loadConfig(CONFIG_FILE)
    const activity = await activityController.createCommentActivity(publicKey, OBJECT_TYPES.POST, resourceId, CONTEXT_TYPES.SYNAPSE, synapseConfig.identity.publicKey)
    broadcastController.broadcastActivity(activity);
    return result;
}

export const updateComment = async (commentId, updatedContent) => {
    return await Comment.updateComment(commentId, updatedContent);
}

export const deleteComment = async (commentId) => {
    return await Comment.deleteComment(commentId);
}

export const getComments = async (resourceType, resourceId) => {
    return await Comment.getComments(resourceType, resourceId);
}

export default {
    createComment,
    updateComment,
    deleteComment,
    getComments
}