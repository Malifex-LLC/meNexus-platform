// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessage } from '#core/messenger.js'
import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { sendRequest, replaceParams } from "#utils/apiUtils.js";
import { ENDPOINTS } from "#api/config/endpoints.js";
import { resolvePendingRequest } from "#core/messenger.js";
import * as peerStateManager from '#core/peerStateManager.js';
import postServices from '#api/services/postServices.js';
import commentServices from '#api/services/commentServices.js';
import synapseServices from '#api/services/synapseServices.js';
import reactionServices from "#api/services/reactionServices.js";
import FormData from 'form-data';

export const handleData = async (libp2p, message) => {
    switch (message.messageType) {
        case MESSAGE_TYPES.DATA.REQUEST:
            switch (message.actionType) {
                case ACTION_TYPES.DATA.QUERY:
                    console.log(`Received DATA_QUERY from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_METADATA) {
                        console.log(`Received SYNAPSE_METADATA request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_SYNAPSE_METADATA,
                            withCredentials: true,
                        });
                        const localMetadata = response.data;

                        const metadataResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_METADATA,
                            { localMetadata },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, metadataResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_MEMBERS) {
                        console.log(`Received SYNAPSE_MEMBERS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_SYNAPSE_MEMBERS,
                            withCredentials: true,
                        });
                        //console.log('GET_SYNAPSE_MEMBERS response: ', response);
                        const members = response.data;

                        const membersResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_MEMBERS,
                            { members },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, membersResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_ACTIVITIES) {
                        console.log(`Received SYNAPSE_ACTIVITIES request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_ALL_ACTIVITIES,
                            withCredentials: true,
                        });
                        //console.log('GET_ALL_ACTIVITIES response: ', response);
                        const activities = response.data;

                        const activitiesResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_ACTIVITIES,
                            { activities },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, activitiesResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_USER_ACTIVITIES) {
                        console.log(`Received SYNAPSE_USER_ACTIVITIES request from ${message.meta.sender}.`);
                        const { publicKey } = message.payload;
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_USER_ACTIVITIES,
                            params: { publicKey },
                            withCredentials: true,
                        });
                        //console.log('GET_USER_ACTIVITIES response: ', response);
                        const activities = response.data;

                        const activitiesResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_USER_ACTIVITIES,
                            { activities },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, activitiesResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_POST_BOARDS) {
                        console.log(`Received SYNAPSE_POST_BOARDS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_SYNAPSE_POST_BOARDS,
                            withCredentials: true,
                        });
                        //console.log("GET_SYNAPSE_POST_BOARDS response ", response);
                        const postBoards = response.data;

                        const postBoardsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_POST_BOARDS,
                            { postBoards },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, postBoardsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_CHAT_CHANNELS) {
                        console.log(`Received SYNAPSE_CHAT_CHANNELS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_SYNAPSE_CHAT_CHANNELS,
                            withCredentials: true,
                        });
                        //console.log("GET_SYNAPSE_CHAT_CHANNELS response ", response);
                        const chatChannels = response.data;

                        const chatChannelsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.SYNAPSE_CHAT_CHANNELS,
                            { chatChannels },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, chatChannelsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.ALL_USERS) {
                        console.log(`Received ALL_USERS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_ALL_USERS,
                            withCredentials: true,
                        });

                        //console.log("GET_ALL_USERS response ", response);
                        const users = response.data;

                        const usersResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.ALL_USERS,
                            {users},
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, usersResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.ALL_POSTS) {
                        console.log(`Received ALL_POSTS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_ALL_POSTS,
                            withCredentials: true,
                        });

                        //console.log("GET_ALL_POSTS response ", response);
                        const posts = response.data;

                        const postsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.ALL_POSTS,
                            { posts },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, postsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.BOARD_POSTS) {
                        console.log(`Received BOARD_POSTS request from ${message.meta.sender}.`);
                        const { board } = message.payload;
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_BOARD_POSTS,
                            params: { board },
                            withCredentials: true,
                        });

                        //console.log("GET_BOARD_POSTS response ", response);
                        const posts = response.data;

                        const postsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.BOARD_POSTS,
                            { posts },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, postsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.CHANNEL_CHATS) {
                        console.log(`Received CHANNEL_CHATS request from ${message.meta.sender}.`);
                        const { channel } = message.payload;
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_CHANNEL_CHAT_MESSAGES,
                            params: { channel },
                            withCredentials: true,
                        });

                        //console.log("GET_CHANNEL_CHATS response ", response);
                        const chats = response.data;

                        const chatsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.CHANNEL_CHATS,
                            { chats },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, chatsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.REACTIONS) {
                        console.log(`Received REACTIONS request from ${message.meta.sender}.`);
                        const { resourceType, resourceId } = message.payload;
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_REACTIONS,
                            params: { resourceType, resourceId },
                            withCredentials: true,
                        });

                        //console.log("GET_REACTIONS response ", response);
                        const reactions = response.data;

                        const reactionsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.REACTIONS,
                            { reactions },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, reactionsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peerId - response not sent.');
                        }
                    }
                    break;

                case ACTION_TYPES.RESOURCE.FETCH:
                    console.log(`Received RESOURCE_FETCH from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.ALL_POSTS) {
                        console.log(`Received ALL_POSTS request from ${message.meta.sender}.`);
                        const { handle } = message.payload;
                        const url = ENDPOINTS.GET_USER_POSTS.replace(':handle', encodeURIComponent(handle));
                        const response = await sendRequest({
                            method: 'GET',
                            url: url,
                            params: { handle },
                            withCredentials: true
                        });

                        //console.log("ALL_POSTS response: ", response);
                        const posts = response.data;

                        const dataResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.ALL_POSTS,
                            { posts },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, dataResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.COMMENTS) {
                        console.log(`Received COMMENTS request from ${message.meta.sender}.`);
                        const { resourceType, resourceId } = message.payload;
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_COMMENTS,
                            params: { resourceType, resourceId },
                            withCredentials: true
                        });

                        //console.log("COMMENTS response ", response);
                        const comments = response.data;

                        const commentsResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.DATA.AGGREGATE,
                            RESOURCE_TYPES.COMMENTS,
                            { comments },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );
                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, commentsResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    break;

                case ACTION_TYPES.RESOURCE.CREATE:
                    console.log(`Received RESOURCE_CREATE from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.POST) {
                        console.log(`Received Create POST request from ${message.meta.sender}.`);
                        console.log("Received Create POST payload:", message.payload);
                        const { publicKey, activeBoard, content } = message.payload;
                        const response = await postServices.createPost(publicKey, activeBoard, content);
                        //console.log("CREATE_POST response ", response);
                        const post = response.data;

                        const createPostResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.CREATE,
                            RESOURCE_TYPES.POST,
                            { post },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, createPostResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }

                    }
                    if (message.resourceType === RESOURCE_TYPES.MEDIA) {
                        console.log(`Received Create MEDIA request from ${message.meta.sender}.`);
                        const { postId, publicKey, filename, mimetype, file } = message.payload;

                        const fileBuffer = Buffer.from(file, 'base64'); // decode from base64
                        const formData = new FormData();

                        // Append fields
                        formData.append('postId', postId);
                        formData.append('publicKey', publicKey);
                        formData.append('post_media', fileBuffer, {
                            filename: filename,
                            contentType: mimetype,
                        });

                        const response = await sendRequest({
                            method: 'POST',
                            url: ENDPOINTS.UPLOAD_POST_MEDIA,
                            data: formData,
                            headers: formData.getHeaders?.(),
                        })
                        //console.log("UPLOAD_POST_MEDIA response: ", response);
                        const upload = response.data;

                        const uploadPostMediaResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.CREATE,
                            RESOURCE_TYPES.POST,
                            { upload },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, uploadPostMediaResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.COMMENTS) {
                        console.log(`Received Create COMMENTS request from ${message.meta.sender}.`);
                        const { resourceType, resourceId, content, publicKey } = message.payload;
                        const response = await commentServices.createComment(resourceType, resourceId, content, publicKey);
                        //console.log("CREATE_POST response ", response);
                        const comment = response.data;

                        const createCommentResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.CREATE,
                            RESOURCE_TYPES.COMMENTS,
                            { comment },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, createCommentResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.REACTIONS) {
                        console.log(`Received Create REACTION request from ${message.meta.sender}.`);
                        const { publicKey, resourceType, resourceId, reactionType } = message.payload;
                        const response = await reactionServices.createReaction(publicKey, resourceId, resourceType, reactionType)
                        //console.log("CREATE_REACTION response ", response);
                        const reaction = response.data;

                        const createReactionResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.CREATE,
                            RESOURCE_TYPES.COMMENTS,
                            { reaction },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, createReactionResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    break;

                case ACTION_TYPES.RESOURCE.UPDATE:
                    console.log(`Received RESOURCE_UPDATE request from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.POST) {
                        console.log(`Received UPDATE POST request from ${message.meta.sender}.`);
                        const { postId, content } = message.payload;
                        const response = await postServices.updatePost(postId, content);
                        //console.log('updatedPost response: ', response);
                        const updatedPost = response.data;

                        const updatedPostResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.UPDATE,
                            RESOURCE_TYPES.POST,
                            { updatedPost },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, updatedPostResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }

                    if (message.resourceType === RESOURCE_TYPES.POST_COMMENT) {
                        console.log(`Received UPDATE COMMENT request from ${message.meta.sender}.`);
                        const { commentId, content } = message.payload;

                        const response = await commentServices.updateComment(commentId, content);
                        //console.log('updateComment response ', response);
                        const updatedComment = response.data;

                        const updatedCommentResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.UPDATE,
                            RESOURCE_TYPES.POST_COMMENT,
                            { updatedComment },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, updatedCommentResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }

                    }

                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_MEMBERS) {
                        console.log(`Received UPDATE SYNAPSE_MEMBERS request from ${message.meta.sender}.`);
                        const { publicKey } = message.payload;

                        const response = await synapseServices.joinSynapse(publicKey);
                        //console.log('joinSynapse response ', response);
                        const joined = response.data;

                        const updatedSynapseMembers = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.UPDATE,
                            RESOURCE_TYPES.SYNAPSE_MEMBERS,
                            { joined },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, updatedSynapseMembers);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }

                    }
                    break;

                case ACTION_TYPES.RESOURCE.DELETE:
                    console.log(`Received DELETE request from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.POST) {
                        console.log(`Received DELETE POST request from ${message.meta.sender}.`);
                        const { postId } = message.payload;
                        const url = replaceParams(ENDPOINTS.DELETE_POST, {postId});
                        console.log('deletePost url ', url)
                        const response = await postServices.deletePost(postId);
                        //console.log('deletePost response: ', response);
                        const deletedPost = response.data;

                        const deletedPostResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.DELETE,
                            RESOURCE_TYPES.POST,
                            { deletedPost },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, deletedPostResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.POST_COMMENT) {
                        console.log(`Received DELETE POST COMMENT request from ${message.meta.sender}.`);
                        const { commentId } = message.payload;
                        const response = await commentServices.deleteComment(commentId);
                        //console.log('deleteComment response ', response);
                        const deletedComment = response.data;

                        const deletedCommentResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.DELETE,
                            RESOURCE_TYPES.POST_COMMENT,
                            { deletedComment },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, deletedCommentResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_MEMBERS) {
                        console.log(`Received DELETE SYNAPSE_MEMBER request from ${message.meta.sender}.`);
                        const { publicKey } = message.payload;
                        const response = await synapseServices.leaveSynapse(publicKey)
                        //console.log('leaveSynapse response ', response);
                        const leaveSynapse = response.data;

                        const leaveSynapseResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.DELETE,
                            RESOURCE_TYPES.POST_COMMENT,
                            { leaveSynapse },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, leaveSynapseResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    if (message.resourceType === RESOURCE_TYPES.REACTIONS) {
                        console.log(`Received DELETE REACTION request from ${message.meta.sender}.`);
                        const { publicKey, resourceType, resourceId, reactionType } = message.payload;
                        const response = await reactionServices.deleteReaction(publicKey, resourceId, resourceType, reactionType)
                        //console.log('DELETE_REACTION response ', response);
                        const deleteReaction = response.data;

                        const deleteReactionResponse = createMessage(
                            MESSAGE_TYPES.DATA.RESPONSE,
                            ACTION_TYPES.RESOURCE.DELETE,
                            RESOURCE_TYPES.REACTIONS,
                            { deleteReaction },
                            {
                                sender: libp2p.peerId.toString(),
                                requestId: message.meta.requestId,
                            }
                        );

                        const { peerId } = peerStateManager.getPeerByPublicKey(message.meta.sender);
                        if (peerId) {
                            await sendMessage(peerId, deleteReactionResponse);
                        } else {
                            console.warn('Cannot map publicKey to peer-id - response not sent.');
                        }
                    }
                    break;

                case ACTION_TYPES.DATA.AGGREGATE:
                    console.log(`Received DATA_RESPONSE from ${message.meta.sender}.`);
                    resolvePendingRequest(message.meta.requestId, message);
                    break;

                default:
                    console.warn('Unknown data actionType:', message.actionType);
                    break;

            }
            break;

        case MESSAGE_TYPES.DATA.RESPONSE:
            console.log(`Received DATA_RESPONSE from ${message.meta.sender}.`);
            resolvePendingRequest(message.meta.requestId, message);
            break;

        default:
            console.warn(`Unhandled messageType in handleData: ${message.messageType}`);
            break;
    }
}