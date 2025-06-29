// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import postController from '../controllers/postController.js';

// Define postRoutes and link them to corresponding controller functions

router.post('/createPost', postController.createPost);
router.put('/updatePost/:postId', postController.updatePost);
router.delete('/deletePost/:postId', postController.deletePost);
router.get('/getPost/:postId', postController.getPost)
router.get('/getAllPosts', postController.getAllPosts);
router.get('/getPosts', postController.getPosts);
router.get('/getUserPosts', postController.getUserPosts);

// Export the router so it can be used in server.js
export default router;