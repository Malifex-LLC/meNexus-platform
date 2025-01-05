const express = require('express');
const router = express.Router();
const postController = require('../controllers/postController');

// Define postRoutes and link them to corresponding controller functions

router.post('/createPost', postController.createPost);
router.put('/updatePost/:postId', postController.updatePost)
router.delete('/deletePost/:postId', postController.deletePost)
router.get('/getPosts', postController.getPosts);
router.get('/getUserPosts/:handle', postController.getUserPosts);

// Export the router so it can be used in app.js
module.exports = router;