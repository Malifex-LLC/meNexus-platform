// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import reactionServices from '../services/reactionServices.js'
import reaction from "../models/reaction.js";

export const createReaction = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    const { resourceId, resourceType, reactionType } = req.body;
    if (!publicKey || !resourceId || !resourceType || !reactionType) {
        return res.status(400).json({error: 'publicKey, resourceId, resourceType, or reactionType not found.'});
    }

    try {
        const reaction = await reactionServices.createReaction(publicKey, resourceId, resourceType, reactionType);
        res.status(200).json({message: 'Reaction created successfully.', reaction});
    } catch (error) {
        console.error('Error in createReaction:', error);
        res.status(500).json({error: 'Failed to create reaction.'});
    }
}

export const deleteReaction = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    const { resourceId, resourceType, reactionType } = req.body;
    if (!publicKey || !resourceId || !resourceType || !reactionType) {
        return res.status(400).json({error: 'publicKey, resourceId, resourceType, or reactionType not found.'});
    }

    try {
        const response = await reactionServices.deleteReaction(publicKey, resourceId, resourceType, reactionType);
        res.status(200).json({message: 'Reaction deleted successfully.', response});
    } catch (error) {
        console.error('Error in deleteReaction:', error);
        res.status(500).json({error: 'Failed to delete reaction.'});
    }
}

export const getReactions = async (req, res) => {
    const { resourceId } = req.query;
    if (!resourceId) {
        return res.status(400).json({error: 'resourceId not found'});
    }

    try {
        const response = await reactionServices.getReactions(resourceId);
        res.status(200).json(response);
    } catch (error) {
        console.error('Error in getReactions:', error);
        res.status(500).json({error: 'Failed to get reactions.'});
    }
}

export default {
    createReaction,
    deleteReaction,
    getReactions,
}