import Comment from '../models/comment.js';

export const createComment = async (resourceType, resourceId, content, publicKey) => {
    return await Comment.createComment(resourceType, resourceId, content, publicKey);
}

export default {
    createComment,
}