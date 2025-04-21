// Import the Post model
const Post = require('../models/post');

// Post creation logic
exports.createPost = async (req, res) => {
    const { content, handle } = req.body;
    if (!content || !handle) {
        return res.status(400).json({error: 'Content or handle not found.'});
    }

    try {
        const postId = await Post.createPost(content, handle);
        res.status(200).json({ message: 'Post created successfully.', postId });
    } catch (error) {
        console.error('Error in createPost:', error);
        res.status(500).json({error: 'Failed to create post.'});
    }
};

// Post update logic
exports.updatePost = async (req, res) => {
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
exports.deletePost = async (req, res) => {
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

exports.getPost = async (req, res) => {
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

// Post fetching logic
exports.getPosts = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user; // Get the current user's ID
    if (!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    try {
        const posts = await Post.getPosts(user_id);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getPosts:', error);
        res.status(500).json({error: 'Failed to get posts.'});
    }
}

exports.getUserPosts = async (req, res) => {
    const handle = req.params.handle
    if (!handle) {
        return res.status(401).json({error: 'User not authenticated'});
    }

    try {
        const posts = await Post.getUserPosts(handle);
        res.status(200).json(posts);
    } catch (error) {
        console.error('Error in getUserPosts:', error);
        res.status(500).json({error: 'Failed to get user posts.'});
    }
}