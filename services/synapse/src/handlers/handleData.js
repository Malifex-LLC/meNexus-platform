// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { createMessage } from '#protocols/snp/messageUtils.js';
import { sendMessage } from '#core/messenger.js'
import { MESSAGE_TYPES, ACTION_TYPES, RESOURCE_TYPES } from "#protocols/snp/index.js";
import { sendRequest } from "#utils/apiUtils.js";
import { ENDPOINTS } from "#api/config/endpoints.js";
import { resolvePendingRequest } from "#core/messenger.js";
import * as peerStateManager from '#core/peerStateManager.js';
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
                        console.log("GET_SYNAPSE_METADATA response ", response);
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
                    if (message.resourceType === RESOURCE_TYPES.SYNAPSE_POST_BOARDS) {
                        console.log(`Received SYNAPSE_POST_BOARDS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_SYNAPSE_POST_BOARDS,
                            withCredentials: true,
                        });
                        console.log("GET_SYNAPSE_POST_BOARDS response ", response);
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
                    if (message.resourceType === RESOURCE_TYPES.ALL_USERS) {
                        console.log(`Received ALL_USERS request from ${message.meta.sender}.`);
                        const response = await sendRequest({
                            method: 'GET',
                            url: ENDPOINTS.GET_ALL_USERS,
                            withCredentials: true,
                        });

                        console.log("GET_ALL_USERS response ", response);
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

                        console.log("GET_ALL_POSTS response ", response);
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

                        console.log("GET_BOARD_POSTS response ", response);
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

                        console.log("ALL_POSTS response: ", response);
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

                        console.log("COMMENTS response ", response);
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
                        const response = await sendRequest({
                            method: 'POST',
                            url: ENDPOINTS.CREATE_POST,
                            data: {publicKey, activeBoard, content},
                        });
                        console.log("CREATE_POST response ", response);
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
                        console.log("UPLOAD_POST_MEDIA response: ", response);
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
                        const response = await sendRequest({
                            method: 'POST',
                            url: ENDPOINTS.CREATE_COMMENT,
                            data: {
                                resourceType,
                                resourceId,
                                content,
                                publicKey,
                            },
                        });
                        console.log("CREATE_POST response ", response);
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
                    break;

                case ACTION_TYPES.RESOURCE.UPDATE:
                    console.log(`Received RESOURCE_UPDATE request from ${message.meta.sender}.`);
                    if (message.resourceType === RESOURCE_TYPES.POST) {
                        console.log(`Received UPDATE POST request from ${message.meta.sender}.`);
                        const { postId, content } = message.payload;
                        const url = `${ENDPOINTS.UPDATE_POST}/${postId}`

                        const updatedPost = await sendRequest({
                            method: 'PUT',
                            url: url,
                            data: { content },
                        });

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