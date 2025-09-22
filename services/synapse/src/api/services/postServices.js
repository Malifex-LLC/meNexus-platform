import Post from '../models/Post.js';

export const createPost = async (publicKey, activeBoard, content) => {
    return await Post.createPost(publicKey, activeBoard, content);
}
