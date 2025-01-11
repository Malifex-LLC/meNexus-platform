import express from 'express';
const router = express.Router();
import synapseController from '../controllers/synapseController.js';

// Define synapseRoutes and link them to corresponding controller functions

router.get('/getSynapseUserPosts', synapseController.getAll);

// Export the router so it can be used in server.js
export default router;