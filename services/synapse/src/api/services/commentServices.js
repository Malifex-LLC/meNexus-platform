import Comment from '../models/comment.js';
import * as commentService from "#api/models/comment.js";

export const createComment = async (resourceType, resourceId, content, publicKey) => {
    return await Comment.createComment(resourceType, resourceId, content, publicKey);
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
    deleteComment
}