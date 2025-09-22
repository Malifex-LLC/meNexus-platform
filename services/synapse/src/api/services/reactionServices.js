import Reaction from '../models/reaction.js';

export const createReaction = async (id, publicKey, resourceId, resourceType, reactionType) => {
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