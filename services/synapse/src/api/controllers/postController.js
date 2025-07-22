// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import the Post model
import Post from '../models/post.js';

// Post creation logic
export const createPost = async (req, res) => {
    const { publicKey, activeBoard, content } = req.body;
    console.log('createPost activeBoard: ', activeBoard);
    if (!publicKey || !activeBoard || !content) {
        return res.status(400).json({error: 'publicKey, activeBoard or content not found.'});
    }

    try {
        const postId = await Post.createPost(publicKey, activeBoard, content);
        res.status(200).json({ message: 'Post created successfully.', postId });
    } catch (error) {
        console.error('Error in createPost:', error);
        res.status(500).json({error: 'Failed to create post.'});
    }
};

// Post update logic
export const updatePost = async (req, res) => {
    const postId = req.params.postId;
    const updatedContent = req.body.content;
    if (!postId || !updatedContent) {
        return res.status(400).json({error: 'postID or updatedContent not found.'});
    }

    try {
        const response = await Post.updatePost(postId, updatedContent);
        res.status(200).json({message: 'Post updated successfully.', response});
    } catch (error) {
        console.error('Error in updatePost:', error);
        res.status(500).json({error: 'Failed to update post.'});
    }
}

// Post delete logic
export const deletePost = async (req, res) => {
    const postId = req.params.postId;
    if (!postId) {
        return res.status(400).json({error: 'postID not found.'});
    }

    try {
        const response = await Post.deletePost(postId);
        res.status(200).json({message: 'Post deleted successfully.', response});
    } catch (error) {
        console.error('Error in deletePost:', error);
        res.status(500).json({error: 'Failed to delete post.'});
    }
}

export const getPost = async (req, res) => {
    const postId = req.params.postId;
    if (!postId) {
        return res.status(400).json({error: 'postID not found.'});
    }

    try {
        const response = await Post.getPost(postId);
        res.status(200).json(response);
    } catch (error) {
        console.error('Error in getPost:', error);
        res.status(500).json({error: 'Failed to fetch post.'});
    }
}


// Get ALL posts (Used for getting all posts from a Synapse)
export const getAllPosts = async (req, res) => {
    try {
        const posts = await Post.getAllPosts();
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getAllPosts:', error);
        res.status(500).json({error: 'Failed to get all posts.'})
    }
}

export const getBoardPosts = async (req, res) => {
    const { board } = req.query;
    if (!board) {
        return res.status(400).json({error: 'Board not found.'});
    }
    try {
        const posts = await Post.getBoardPosts(board);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getBoardPosts:', error);
        res.status(500).json({error: 'Failed to get board posts.'})
    }
}

// Post fetching logic
export const getPosts = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey  = req.session.user.publicKey; // Get the current user's publicKey
    if (!publicKey) {
        return res.status(401).json({ error: 'publicKey not found!' });
    }

    try {
        console.log('Getting posts for publicKey: ', publicKey);
        const posts = await Post.getPosts(publicKey);
        console.log('Posting found: ',posts);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getPosts:', error);
        res.status(500).json({error: 'Failed to get posts.'});
    }
}

export const getUserPosts = async (req, res) => {
    const { publicKey } = req.query
    console.log('getUserPosts publicKey: ', publicKey);
    if (!publicKey) {
        return res.status(401).json({error: 'User not authenticated'});
    }

    try {
        const posts = await Post.getUserPosts(publicKey);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getUserPosts:', error);
        res.status(500).json({error: 'Failed to get user posts.'});
    }
}

export default {
    createPost,
    updatePost,
    deletePost,
    getPost,
    getAllPosts,
    getBoardPosts,
    getPosts,
    getUserPosts,
}