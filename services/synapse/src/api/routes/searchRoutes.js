// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import searchController from '../controllers/searchController.js';

// Define the searchRoutes and link it to the controller function

router.get('/search', searchController.search);

// Export the router so it can be used in server.js
export default router;