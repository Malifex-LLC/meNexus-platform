import Post from '../models/post.js';

export const createPost = async (publicKey, activeBoard, content) => {
    return await Post.createPost(publicKey, activeBoard, content);
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