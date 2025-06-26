// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import commentController from '../controllers/commentController.js';

// Define commentRoutes and link them to corresponding controller functions

router.post('/createComment', commentController.createComment);
router.put('/updateComment/:comment_id', commentController.updateComment);
router.delete('/deleteComment/:comment_id', commentController.deleteComment);
router.get('/getComments', commentController.getComments);

// Export the router so it can be used in server.js
export default router;