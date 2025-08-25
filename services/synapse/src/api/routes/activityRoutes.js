// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import activityController from '../controllers/activityController.js';

router.get('/getAllActivities', activityController.getAllActivities);
router.get('/getUserActivities', activityController.getUserActivities);

export default router;