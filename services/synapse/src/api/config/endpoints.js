// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

export const API_BASE_URL = "http://localhost:3001";

export const ENDPOINTS = {

    WEBSOCKET: "ws://localhost:3001?user_id=",

    REGISTER_PUBLIC_KEY: `${API_BASE_URL}/api/auth/registerPublicKey`,
    STORE_PUBLIC_KEY: `${API_BASE_URL}/api/auth/storePublicKey`,
    GET_PUBLIC_KEY: `${API_BASE_URL}/api/auth/getPublicKey`,
    GET_CRYPTO_CHALLENGE: `${API_BASE_URL}/api/auth/getCryptoChallenge`,
    VERIFY_CRYPTO_SIGNATURE: `${API_BASE_URL}/api/auth/verifyCryptoSignature`,
    LOGIN: `${API_BASE_URL}/api/auth/login`,
    LOGOUT: `${API_BASE_URL}/api/auth/logout`,

    CREATE_USER: `${API_BASE_URL}/api/auth/createUser`,
    GET_ALL_USERS: `${API_BASE_URL}/api/user/getAllUsers`,
    GET_SESSION_USER: `${API_BASE_URL}/api/user/getSessionUser`,
    GET_USER: `${API_BASE_URL}/api/user/getUserByPublicKey`,
    GET_USER_BY_HANDLE: `${API_BASE_URL}/api/user/getUserByHandle`,
    GET_PROFILE: `${API_BASE_URL}/api/user/getProfile/:handle`,

    UPDATE_ACCOUNT_SETTINGS: `${API_BASE_URL}/api/auth/updateAccountSettings`,
    UPDATE_PROFILE_SETTINGS: `${API_BASE_URL}/api/user/updateProfileSettings/:handle`,

    FOLLOW_USER: `${API_BASE_URL}/api/follow/followUser`,
    UNFOLLOW_USER: `${API_BASE_URL}/api/follow/unfollowUser`,
    FOLLOW_CHECK: `${API_BASE_URL}/api/follow/followCheck`,

    CREATE_POST: `${API_BASE_URL}/api/post/createPost`,
    UPDATE_POST: `${API_BASE_URL}/api/post/updatePost/:postId`,
    DELETE_POST: `${API_BASE_URL}/api/post/deletePost/:postId`,
    GET_ALL_POSTS: `${API_BASE_URL}/api/post/getAllPosts`,
    GET_BOARD_POSTS: `${API_BASE_URL}/api/post/getBoardPosts`,
    GET_POSTS: `${API_BASE_URL}/api/post/getPosts`,
    GET_USER_POSTS: `${API_BASE_URL}/api/post/getUserPosts`,
    UPLOAD_POST_MEDIA: `${API_BASE_URL}/api/post/uploadPostMedia`,
    UNFURL_URL: `${API_BASE_URL}/api/post/unfurlUrl`,

    CREATE_COMMENT: `${API_BASE_URL}/api/comment/createComment`,
    UPDATE_COMMENT: `${API_BASE_URL}/api/comment/updateComment/:commentId`,
    DELETE_COMMENT: `${API_BASE_URL}/api/comment/deleteComment/:commentId`,
    GET_COMMENTS: `${API_BASE_URL}/api/comment/getComments`,

    CREATE_CHAT_MESSAGE: `${API_BASE_URL}/api/chat/createChatMessage`,
    GET_CHANNEL_CHAT_MESSAGES: `${API_BASE_URL}/api/chat/getChannelChatMessages`,

    CREATE_CONVERSATION: `${API_BASE_URL}/api/conversation/createConversation`,
    UPDATE_CONVERSATION_PARTICIPANTS: `${API_BASE_URL}/api/conversation/updateConversationParticipants`,
    GET_CONVERSATIONS: `${API_BASE_URL}/api/conversation/getConversations`,

    CREATE_MESSAGE: `${API_BASE_URL}/api/message/createMessage`,
    SET_MESSAGES_AS_READ: `${API_BASE_URL}/api/message/setMessagesAsRead`,
    GET_MESSAGES: `${API_BASE_URL}/api/message/getMessages`,

    CREATE_NOTIFICATION: `${API_BASE_URL}/api/notification/createNotification`,
    SET_NOTIFICATION_AS_READ: `${API_BASE_URL}/api/notification/setNotificationAsRead`,
    GET_NOTIFICATIONS: `${API_BASE_URL}/api/notification/getNotifications`,

    UPLOAD_PROFILE_PICTURE: `${API_BASE_URL}/api/settings/uploadProfilePicture`,

    SEARCH: `${API_BASE_URL}/api/search/search`,

    GET_SYNAPSE_METADATA: `${API_BASE_URL}/api/synapse/getSynapseMetadata`,
    GET_SYNAPSE_MEMBERS: `${API_BASE_URL}/api/synapse/getSynapseMembers`,
    JOIN_SYNAPSE: `${API_BASE_URL}/api/synapse/joinSynapse`,
    LEAVE_SYNAPSE: `${API_BASE_URL}/api/synapse/leaveSynapse`,
    GET_SYNAPSE_POST_BOARDS: `${API_BASE_URL}/api/synapse/getSynapsePostBoards`,
    GET_SYNAPSE_CHAT_CHANNELS: `${API_BASE_URL}/api/synapse/getSynapseChatChannels`,
    GET_ALL_DISCOVERED_PEERS: `${API_BASE_URL}/api/synapse/getAllDiscoveredPeers`,


    /* ---- REMOTE ROUTES ----------------------------------------------------- */
    FETCH_REMOTE_SYNAPSE_METADATA: `${API_BASE_URL}/remote/fetchRemoteSynapseMetadata`,
    FETCH_REMOTE_USERS: `${API_BASE_URL}/remote/getRemoteUsers`,
    FETCH_REMOTE_SYNAPSE_POST_BOARDS: `${API_BASE_URL}/remote/fetchRemoteSynapsePostBoards`,
    FETCH_REMOTE_SYNAPSE_CHAT_CHANNELS: `${API_BASE_URL}/remote/fetchRemoteSynapseChatChannels`,
    FETCH_REMOTE_POSTS: `${API_BASE_URL}/remote/getRemotePosts`,
    FETCH_REMOTE_BOARD_POSTS: `${API_BASE_URL}/remote/getRemoteBoardPosts`,
    FETCH_REMOTE_USER_POSTS: `${API_BASE_URL}/remote/getRemoteUserPosts`,
    CREATE_REMOTE_POST: `${API_BASE_URL}/remote/createRemotePost`,
    UPLOAD_REMOTE_POST_MEDIA: `${API_BASE_URL}/remote/uploadRemotePostMedia`,
    UPDATE_REMOTE_POST: `${API_BASE_URL}/remote/updateRemotePost`,
    DELETE_REMOTE_POST: `${API_BASE_URL}/remote/deleteRemotePost`,
    FETCH_REMOTE_POST_COMMENTS: `${API_BASE_URL}/remote/fetchRemotePostComments`,
    CREATE_REMOTE_POST_COMMENT: `${API_BASE_URL}/remote/createRemotePostComment`,
    UPDATE_REMOTE_POST_COMMENT: `${API_BASE_URL}/remote/updateRemotePostComment`,
    DELETE_REMOTE_POST_COMMENT: `${API_BASE_URL}/remote/deleteRemotePostComment`,
    FETCH_REMOTE_CHANNEL_CHATS: `${API_BASE_URL}/remote/fetchRemoteChannelChats`,
    JOIN_REMOTE_SYNAPSE: `${API_BASE_URL}/remote/joinRemoteSynapse`,
    LEAVE_REMOTE_SYNAPSE: `${API_BASE_URL}/remote/leaveRemoteSynapse`,
};