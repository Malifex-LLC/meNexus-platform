// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import authController from '../controllers/authController.js'

// Define authRoutes and link them to corresponding controller functions

router.post('/createUser', authController.createUser);
router.get('/generateCryptoKeys', authController.generateCryptoKeys);
router.post('/storePublicKey', authController.storePublicKey);
router.get('/getUserIdByPublicKey', authController.getUserIdByPublicKey);
router.get('/getAllPublicKeys', authController.getAllPublicKeys);
router.get('/getCryptoChallenge', authController.getCryptoChallenge);
router.post('/verifyCryptoSignature', authController.verifyCryptoSignature);
router.post('/refresh', authController.refresh);
router.post('/logout', authController.logout);
router.put('/updateAccountSettings', authController.updateAccountSettings);

// Export the router so it can be used in server.js
export default router;