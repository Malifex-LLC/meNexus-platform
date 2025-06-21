import express from 'express';
const router = express.Router();
import remoteSynapseController from '#remotes/controllers/remoteSynapseController.js'
import remoteUserController from "#remotes/controllers/remoteUserController.js";
import remotePostController from "#remotes/controllers/remotePostController.js"

// Define synapseRoutes and link them to corresponding controller functions

router.get('/fetchRemoteSynapseMetadata', remoteSynapseController.fetchRemoteSynapseMetadata);
router.post('/joinSynapse', remoteSynapseController.joinSynapse);
router.post('/leaveSynapse', remoteSynapseController.leaveSynapse);

router.get('/fetchRemoteUsers', remoteUserController.fetchRemoteUsers);

router.get('/fetchRemotePosts', remotePostController.fetchRemotePosts);
router.get('/fetchRemoteUserPosts', remotePostController.fetchRemoteUserPosts);
router.post('/createRemotePost', remotePostController.createRemotePost);



// Export the router so it can be used in server.js

export default router;