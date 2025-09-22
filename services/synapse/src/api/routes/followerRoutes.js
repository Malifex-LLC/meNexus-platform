// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import followerController from '../controllers/followerController.js';
import {requireJwt} from "#api/middlewares/requireJwt.js";

// Define follwerRoutes and link them to corresponding controller functions

router.post('/followUser', requireJwt(['follow:write']), followerController.followUser);
router.delete('/unfollowUser', requireJwt(['follow:write']), followerController.unfollowUser);
router.get('/followCheck', requireJwt(['follow:read']),  followerController.followCheck)
router.get('/getFollowerCount', followerController.getFollowerCount);
router.get('/getFollowingCount', followerController.getFollowingCount);

// Export the router so it can be used in server.js
export default router;