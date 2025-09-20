// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import reactionController from '../controllers/reactionController.js';
import {deleteReaction} from "#api/models/reaction.js";

router.post('/createReaction', reactionController.createReaction);
router.post('/deleteReaction', reactionController.deleteReaction);
router.get('/getReactions', reactionController.getReactions);

export default router;