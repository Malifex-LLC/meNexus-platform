// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import postController from '../controllers/postController.js';
import {requireJwt} from "#api/middlewares/requireJwt.js";

// Define postRoutes and link them to corresponding controller functions

router.post('/createPost', requireJwt(['posts:write']), postController.createPost);
router.put('/updatePost/:postId', requireJwt(['posts:write']),  postController.updatePost);
router.delete('/deletePost/:postId', requireJwt(['posts:write']), postController.deletePost);
router.get('/getPost/:postId', postController.getPost)
router.get('/getAllPosts', postController.getAllPosts);
router.get('/getBoardPosts', postController.getBoardPosts);
router.get('/getPosts', postController.getPosts);
router.get('/getUserPosts', postController.getUserPosts);
router.post('/uploadPostMedia', requireJwt(['posts:write']), postController.uploadPostMedia);
router.post('/unfurlUrl', postController.unfurlUrl);

// Export the router so it can be used in server.js
export default router;