// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import the Comment model
import Comment from '../models/comment.js';
import activityController from './activityController.js';
import { ACTIVITY_TYPES, OBJECT_TYPES, CONTEXT_TYPES } from '#api/config/activityConstants.js'

import path from 'path';
import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

export const createComment = async (req, res) => {
    const { resourceType, resourceId, content, publicKey} = req.body;
    console.log('createComment called from controller:', resourceType, resourceId, content, publicKey);
    if (!resourceType || !resourceId || !content || !publicKey) {
        return res.status(400).json({error: 'resourceType, resourceId, content, or publicKey not found.'});
    }

    const result = await Comment.createComment(resourceType, resourceId, content, publicKey);

    if (result.affectedRows === 0) {
        return res.status(500).json({error: 'Failed to create a comment'});
    }

    const synapseConfig = await loadConfig(CONFIG_FILE)
    await activityController.createCommentActivity(publicKey, OBJECT_TYPES.POST, resourceId, CONTEXT_TYPES.SYNAPSE, synapseConfig.identity.publicKey)
    return res.status(200).json(result);
}

export const updateComment = async (req, res) => {
    const commentId = req.params.commentId;
    const updatedContent = req.body.content;

    const result = await Comment.updateComment(commentId, updatedContent);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'Comment not found'});
    }

    return res.status(200).json(result);
}

export const deleteComment = async (req, res) => {
    const commentId = req.params.commentId;

    const result = await Comment.deleteComment(commentId);

    if (result.affectedRows === 0) {
        return res.status(404).json({error: 'Comment not found'});
    }

    return res.status(200).json(result);
}

export const getComments = async (req, res) => {
    const {resourceType, resourceId } = req.query;

    if (!resourceType || resourceId.trim() === "") {
        return res.status(400).json({ error: "Invalid getComments query." });
    }

    const results = await Comment.getComments(resourceType, resourceId);
    return res.status(200).json(results);
}

export default {
    createComment,
    updateComment,
    deleteComment,
    getComments
}