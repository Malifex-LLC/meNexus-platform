const express = require('express');
const router = express.Router();
const followerController = require('../controllers/followerController');

// Define follwerRoutes and link them to corresponding controller functions

router.post('/followUser', followerController.followUser);
router.delete('/unfollowUser', followerController.unfollowUser);
router.get('/followCheck', followerController.followCheck)

// Export the router so it can be used in server.js
module.exports = router;