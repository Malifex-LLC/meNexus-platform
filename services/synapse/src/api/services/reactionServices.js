import Reaction from '../models/reaction.js';
import { v4 as uuidv4 } from 'uuid';

export const createReaction = async (publicKey, resourceId, resourceType, reactionType) => {
    const id = uuidv4();
    return await Reaction.createReaction(id, publicKey, resourceId, resourceType, reactionType)
}

export const deleteReaction = async (publicKey, resourceId, resourceType, reactionType) => {
    return await Reaction.deleteReaction(publicKey, resourceId, resourceType, reactionType)
}

export const getReactions = async (resourceId) => {
    return await Reaction.getReactions(resourceId);
}

export default {
    createReaction,
    deleteReaction,
    getReactions,
}