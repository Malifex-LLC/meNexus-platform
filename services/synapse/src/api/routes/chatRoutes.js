// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import chatController from '../controllers/chatController.js';
import {requireJwt} from "#api/middlewares/requireJwt.js";

// Define messageRoutes and link them to corresponding controller functions

router.post('/createChatMessage', requireJwt(['chats:write']), chatController.createChatMessage);
router.get('/getChannelChatMessages', chatController.getChannelChatMessages);

// Export the router so it can be used in server.js
export default router;