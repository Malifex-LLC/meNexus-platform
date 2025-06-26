// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import userController from '../controllers/userController.js';

// Define userRoutes and link them to corresponding controller functions

router.get('/getAllUsers', userController.getAllUsers);
router.get('/getSessionUser', userController.getSessionUser);
router.get('/getUserByPublicKey', userController.getUserByPublicKey);
router.get('/getUserByHandle', userController.getUserByHandle);
router.get('/getProfile/:handle', userController.getProfile);
router.put('/updateProfileSettings/:publicKey', userController.updateProfileSettings);

// Export the router so it can be used in server.js
export default router;