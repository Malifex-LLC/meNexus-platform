import Post from '../models/post.js';

export const createPost = async (publicKey, activeBoard, content) => {
    return await Post.createPost(publicKey, activeBoard, content);
}
