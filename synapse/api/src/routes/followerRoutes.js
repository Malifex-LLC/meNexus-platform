import express from 'express';
const router = express.Router();
import followerController from '../controllers/followerController.js';

// Define follwerRoutes and link them to corresponding controller functions

router.post('/followUser', followerController.followUser);
router.delete('/unfollowUser', followerController.unfollowUser);
router.get('/followCheck', followerController.followCheck)

// Export the router so it can be used in server.js
export default router;