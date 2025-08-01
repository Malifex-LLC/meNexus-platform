// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import synapseController from "#api/controllers/synapseController.js";

// Define userRoutes and link them to corresponding controller functions

router.get('/getSynapseMetadata', synapseController.getSynapseMetadata);
router.get('/getAllDiscoveredPeers', synapseController.getAllDiscoveredPeers);
router.get('/getSynapsePostBoards', synapseController.getSynapsePostBoards);
router.get('/getSynapseChatChannels', synapseController.getSynapseChatChannels);

export default router;