// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import synapseController, {forceLeaveSynapse} from "#api/controllers/synapseController.js";
import {requireJwt} from "#api/middlewares/requireJwt.js";

// Define userRoutes and link them to corresponding controller functions

router.get('/getSynapseMetadata', synapseController.getSynapseMetadata);
router.get('/getAllDiscoveredPeers', synapseController.getAllDiscoveredPeers);
router.get('/getSynapseMembers', synapseController.getSynapseMembers);
router.post('/joinSynapse', requireJwt(['users:write']), synapseController.joinSynapse);
router.post('/leaveSynapse', requireJwt(['users:write']), synapseController.leaveSynapse);
router.post('/forceLeaveSynapse', synapseController.forceLeaveSynapse);
router.get('/getSynapsePostBoards', synapseController.getSynapsePostBoards);
router.get('/getSynapseChatChannels', synapseController.getSynapseChatChannels);

export default router;