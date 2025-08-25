// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import Activity from '../models/activity.js';
import {ACTIVITY_TYPES, CONTEXT_TYPES, OBJECT_TYPES} from '#api/config/activityConstants.js'
import { v4 as uuidv4 } from 'uuid';

export const createPostActivity = async (actorPublicKey, objectId, contextType, contextId) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.POSTED,
            actorPublicKey,
            objectType: OBJECT_TYPES.POST,
            objectId,
            contextType,
            contextId,
            published});
    } catch (error) {
        console.error('Error in createPostActivity', error);
    }
}

export const createCommentActivity = async (actorPublicKey, objectType, objectId, contextType, contextId) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.COMMENTED,
            actorPublicKey,
            objectType,
            objectId,
            contextType,
            contextId,
            published
        });
    } catch (error) {
        console.error('Error in createCommentActivity', error);
    }
}

export const createFollowActivity = async (actorPublicKey, targetId) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.FOLLOWED,
            actorPublicKey,
            objectType: OBJECT_TYPES.USER,
            objectId: actorPublicKey,
            targetType: OBJECT_TYPES.USER,
            targetId,
            contextType: CONTEXT_TYPES.GLOBAL,
            published
        });
    } catch (error) {
        console.error('Error in createFollowActivity', error);
    }
}

export const createUnfollowActivity = async (actorPublicKey, targetId) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.UNFOLLOWED,
            actorPublicKey,
            objectType: OBJECT_TYPES.USER,
            objectId: actorPublicKey,
            targetType: OBJECT_TYPES.USER,
            targetId,
            contextType: CONTEXT_TYPES.GLOBAL,
            published
        });
    } catch (error) {
        console.error('Error in createUnfollowActivity', error);
    }
}

export const createJoinSynapseActivity = async (actorPublicKey, synapsePublicKey) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.JOINED,
            actorPublicKey,
            objectType: OBJECT_TYPES.SYNAPSE,
            objectId: synapsePublicKey,
            targetType: OBJECT_TYPES.SYNAPSE,
            targetId: synapsePublicKey,
            contextType: CONTEXT_TYPES.SYNAPSE,
            contextId: synapsePublicKey,
            published
        });
    } catch (error) {
        console.error('Error in createJoinSynapseActivity', error);
    }
}

export const createLeaveSynapseActivity = async (actorPublicKey, synapsePublicKey) => {
    try {
        const id = uuidv4();
        const published = new Date().toISOString().slice(0, 19).replace('T', ' ')
        return await Activity.createActivity({
            id,
            type: ACTIVITY_TYPES.LEFT,
            actorPublicKey,
            objectType: OBJECT_TYPES.SYNAPSE,
            objectId: synapsePublicKey,
            targetType: OBJECT_TYPES.SYNAPSE,
            targetId: synapsePublicKey,
            contextType: CONTEXT_TYPES.SYNAPSE,
            contextId: synapsePublicKey,
            published
        });
    } catch (error) {
        console.error('Error in createLeaveSynapseActivity', error);
    }
}

export const getAllActivities = async (req, res) => {
    try {
        const activities = await Activity.getAllActivities();
        res.status(200).json(activities);
    } catch (err) {
        console.error('Error getting all activities: ', err);
        res.status(500).json({error: 'Error getting all activities'});
    }
}

export const getUserActivities = async (req, res) => {
    const { publicKey } = req.query
    if (!publicKey) {
        res.status(400).json({error: 'No publicKey provided'});
    }
    try {
        const activities = await Activity.getUserActivities(publicKey);
        res.status(200).json(activities);
    } catch (err) {
        console.error('Error getting all activities: ', err);
        res.status(500).json({error: 'Error getting all activities'});
    }
}

export default {
    createPostActivity,
    createCommentActivity,
    createFollowActivity,
    createUnfollowActivity,
    createJoinSynapseActivity,
    createLeaveSynapseActivity,
    getAllActivities,
    getUserActivities
}