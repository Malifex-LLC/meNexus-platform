// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import the Post model
import Post from '../models/post.js';
import activityController from './activityController.js';
import broadcastController from './broadcastController.js';
import { ACTIVITY_TYPES, OBJECT_TYPES, CONTEXT_TYPES } from '#api/config/activityConstants.js'


import Busboy from 'busboy';
import fs from 'fs';
import path from 'path';

import { fetchLinkPreview } from "#utils/apiUtils.js";

import { fileURLToPath } from 'url';
import {loadConfig} from "#utils/configUtils.js";

// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');


// Post creation logic
export const createPost = async (req, res) => {
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
    const { activeBoard, content } = req.body;
    console.log('createPost publicKey: ', publicKey);
    if (!publicKey || !activeBoard || !content) {
        return res.status(400).json({error: 'publicKey, activeBoard or content not found.'});
    }

    try {
        const postId = await Post.createPost(publicKey, activeBoard, content);
        const synapseConfig = await loadConfig(CONFIG_FILE)
        const activity = await activityController.createPostActivity(publicKey, postId, CONTEXT_TYPES.SYNAPSE, synapseConfig.identity.publicKey)
        console.log('activityController.createPostActivity() response: ', activity);
        broadcastController.broadcastActivity(activity);
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
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
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

export const uploadPostMedia = async (req, res) => {
    try {
        const busboy = Busboy({ headers: req.headers });

        let publicKey = '';
        let postId = '';
        let fileInfo = null;
        let chunks = [];

        busboy.on('field', (fieldname, val) => {
            if (fieldname === 'postId') postId = val.toString();
            if (fieldname === 'publicKey') publicKey = val.toString();
        });

        busboy.on('file', (fieldname, file, info) => {
            if (fieldname !== 'post_media') {
                file.resume(); // skip irrelevant fields
                return;
            }

            fileInfo = info;

            file.on('data', (data) => {
                chunks.push(data);
            });

            file.on('limit', () => {
                console.warn('File size exceeded limit');
            });

            file.on('error', (err) => {
                console.error('Stream error:', err);
            });
        });

        busboy.on('finish', async () => {
            try {
                if (!publicKey || !postId || !fileInfo || chunks.length === 0) {
                    return res.status(400).json({ error: 'Missing fields or file data' });
                }

                const { filename, mimeType } = fileInfo;
                const savedFileName = filename;
                const savedMimeType = mimeType;

                const uploadDir = path.join(process.env.UPLOADS_DIR, publicKey, 'posts', postId);
                fs.mkdirSync(uploadDir, { recursive: true });

                const savedFilePath = path.join(uploadDir, savedFileName);
                await fs.promises.writeFile(savedFilePath, Buffer.concat(chunks));

                const mediaUrl = `/uploads/${publicKey}/posts/${postId}/${savedFileName}`;

                await Post.uploadPostMedia({
                    postId,
                    publicKey,
                    mediaUrl,
                    filename: savedFileName,
                    mimetype: savedMimeType
                });

                return res.status(200).json({ message: 'Media uploaded successfully.' });
            } catch (err) {
                console.error('Final upload handler error:', err);
                return res.status(500).json({ error: 'Failed to save media.' });
            }
        });

        req.pipe(busboy);
    } catch (err) {
        console.error('Upload controller crash:', err);
        return res.status(500).json({ error: 'Unexpected server error.' });
    }
};

export const unfurlUrl = async (req, res) => {
    const { url } = req.body;
    console.log("unfurlUrl called for url: ", url);

    if (!url) {
        return res.status(400).json({ error: 'Missing URL' });
    }

    try {
        const preview = await fetchLinkPreview(url);
        res.json(preview);
    } catch (err) {
        console.error('Failed to fetch preview:', err.message);
        res.status(500).json({ error: 'Unable to fetch link preview' });
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
    uploadPostMedia,
    unfurlUrl
}