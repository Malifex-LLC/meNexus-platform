import express from 'express';
const router = express.Router();
import synapseController from '../controllers/synapseController.js';

// Define synapseRoutes and link them to corresponding controller functions

router.get('/getSynapseMetadata', synapseController.getSynapseMetadata);
router.get('/getLocalSynapseMetadata', synapseController.getLocalSynapseMetadata);

router.get('/getSynapseUsers', synapseController.getSynapseUsers);
router.get('/getSynapsePosts', synapseController.getSynapsePosts);
router.get('/getSynapseUserPosts', synapseController.getSynapseUserPosts);

router.post('/createSynapsePost', synapseController.createSynapsePost);

router.post('/joinSynapse', synapseController.joinSynapse);
router.post('/leaveSynapse', synapseController.leaveSynapse);

// Export the router so it can be used in server.js

export default router;