// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express from 'express';
const router = express.Router();
import remoteSynapseController from '#remotes/controllers/remoteSynapseController.js'
import remoteUserController from "#remotes/controllers/remoteUserController.js";
import remotePostController from "#remotes/controllers/remotePostController.js"
import remoteCommentController from "#remotes/controllers/remoteCommentController.js";
import remoteChatController from "#remotes/controllers/remoteChatController.js";
import remoteReactionController from "#remotes/controllers/remoteReactionController.js";
import {requireJwt} from "#api/middlewares/requireJwt.js";

// Define synapseRoutes and link them to corresponding controller functions

router.get('/fetchRemoteSynapseMetadata', remoteSynapseController.fetchRemoteSynapseMetadata);
router.get('/fetchRemoteSynapseMembers', remoteSynapseController.fetchRemoteSynapseMembers);
router.post('/joinRemoteSynapse', requireJwt('users:write'), remoteSynapseController.joinRemoteSynapse);
router.post('/leaveRemoteSynapse', requireJwt('users:write'), remoteSynapseController.leaveRemoteSynapse);
router.get('/fetchRemoteSynapsePostBoards', remoteSynapseController.fetchRemoteSynapsePostBoards);
router.get('/fetchRemoteSynapseChatChannels', remoteSynapseController.fetchRemoteSynapseChatChannels);

router.get('/fetchRemoteUsers', remoteUserController.fetchRemoteUsers);

router.get('/fetchRemoteSynapseAllActivities', remoteSynapseController.fetchRemoteSynapseAllActivities)
router.get('/fetchRemoteSynapseUserActivities', remoteSynapseController.fetchRemoteSynapseUserActivities)

router.get('/fetchRemotePosts', remotePostController.fetchRemotePosts);
router.get('/fetchRemoteBoardPosts', remotePostController.fetchRemoteBoardPosts);
router.get('/fetchRemoteUserPosts', remotePostController.fetchRemoteUserPosts);
router.post('/createRemotePost', requireJwt('posts:write'), remotePostController.createRemotePost);
router.post('/uploadRemotePostMedia', requireJwt('posts:write'), remotePostController.uploadRemotePostMedia);
router.put('/updateRemotePost', requireJwt('posts:write'), remotePostController.updateRemotePost);
router.delete('/deleteRemotePost', requireJwt('posts:write'), remotePostController.deleteRemotePost);

router.get('/fetchRemotePostComments', remoteCommentController.fetchRemotePostComments);
router.post('/createRemotePostComment', requireJwt('comments:write'), remoteCommentController.createRemotePostComment);
router.put('/updateRemotePostComment', requireJwt('comments:write'), remoteCommentController.updateRemotePostComment);
router.delete('/deleteRemotePostComment', requireJwt('comments:write'), remoteCommentController.deleteRemotePostComment);

router.get('/fetchRemoteReactions', remoteReactionController.fetchRemoteReactions);
router.post('/createRemoteReaction', requireJwt('reactions:write'), remoteReactionController.createRemoteReaction);
router.post('/deleteRemoteReaction', requireJwt('reactions:write'), remoteReactionController.deleteRemoteReaction);

router.get('/fetchRemoteChannelChats', remoteChatController.fetchRemoteChannelChats);



// Export the router so it can be used in server.js

export default router;