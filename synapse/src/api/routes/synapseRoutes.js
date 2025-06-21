import express from 'express';
const router = express.Router();
import synapseController from "#api/controllers/synapseController.js";

// Define userRoutes and link them to corresponding controller functions

router.get('/getSynapseMetadata', synapseController.getSynapseMetadata);

export default router;