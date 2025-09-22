// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import commentController from '../controllers/commentController.js';
import {requireJwt} from "#api/middlewares/requireJwt.js";


// Define commentRoutes and link them to corresponding controller functions

router.post('/createComment', requireJwt(['comments:write']), commentController.createComment);
router.put('/updateComment/:commentId', requireJwt(['comments:write']), commentController.updateComment);
router.delete('/deleteComment/:commentId', requireJwt(['comments:write']), commentController.deleteComment);
router.get('/getComments', commentController.getComments);

// Export the router so it can be used in server.js
export default router;