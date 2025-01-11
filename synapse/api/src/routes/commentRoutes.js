const express = require('express');
const router = express.Router();
const commentController = require('../controllers/commentController');

// Define commentRoutes and link them to corresponding controller functions

router.post('/createComment', commentController.createComment);
router.put('/updateComment/:comment_id', commentController.updateComment);
router.delete('/deleteComment/:comment_id', commentController.deleteComment);
router.get('/getComments', commentController.getComments);

// Export the router so it can be used in server.js
module.exports = router;