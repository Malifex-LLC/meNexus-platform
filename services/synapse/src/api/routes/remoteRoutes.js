// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import remoteSynapseController from '#remotes/controllers/remoteSynapseController.js'
import remoteUserController from "#remotes/controllers/remoteUserController.js";
import remotePostController from "#remotes/controllers/remotePostController.js"
import remoteCommentController from "#remotes/controllers/remoteCommentController.js";

// Define synapseRoutes and link them to corresponding controller functions

router.get('/fetchRemoteSynapseMetadata', remoteSynapseController.fetchRemoteSynapseMetadata);
router.post('/joinSynapse', remoteSynapseController.joinSynapse);
router.post('/leaveSynapse', remoteSynapseController.leaveSynapse);

router.get('/fetchRemoteUsers', remoteUserController.fetchRemoteUsers);

router.get('getchRemoteSynapsePostBoards', remoteSynapseController.fetchRemoteSynapsePostBoards)
router.get('/fetchRemotePosts', remotePostController.fetchRemotePosts);
router.get('/fetchRemoteBoardPosts', remotePostController.fetchRemoteBoardPosts);
router.get('/fetchRemoteUserPosts', remotePostController.fetchRemoteUserPosts);
router.get('/fetchRemotePostComments', remoteCommentController.fetchRemotePostComments);
router.post('/createRemotePost', remotePostController.createRemotePost);
router.post('/uploadRemotePostMedia', remotePostController.uploadRemotePostMedia);
router.post('/createRemotePostComment', remoteCommentController.createRemotePostComment);



// Export the router so it can be used in server.js

export default router;