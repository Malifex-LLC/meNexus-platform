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
    GET_SESSION_USER: `${API_BASE_URL}/api/user/getSessionUser`,
    GET_USER: `${API_BASE_URL}/api/user/getUserByPublicKey`,
    GET_PROFILE: `${API_BASE_URL}/api/user/getProfile/:handle`,

    UPDATE_ACCOUNT_SETTINGS: `${API_BASE_URL}/api/auth/updateAccountSettings`,
    UPDATE_PROFILE_SETTINGS: `${API_BASE_URL}/api/user/updateProfileSettings/:handle`,

    FOLLOW_USER: `${API_BASE_URL}/api/follow/followUser`,
    UNFOLLOW_USER: `${API_BASE_URL}/api/follow/unfollowUser`,
    FOLLOW_CHECK: `${API_BASE_URL}/api/follow/followCheck`,

    CREATE_POST: `${API_BASE_URL}/api/post/createPost`,
    UPDATE_POST: `${API_BASE_URL}/api/post/updatePost/:postId`,
    DELETE_POST: `${API_BASE_URL}/api/post/deletePost/:postId`,
    GET_POSTS: `${API_BASE_URL}/api/post/getPosts`,
    GET_USER_POSTS: `${API_BASE_URL}/api/post/getUserPosts/:handle`,

    CREATE_COMMENT: `${API_BASE_URL}/api/comment/createComment`,
    UPDATE_COMMENT: `${API_BASE_URL}/api/comment/updateComment/:comment_id`,
    DELETE_COMMENT: `${API_BASE_URL}/api/comment/deleteComment/:comment_id`,
    GET_COMMENTS: `${API_BASE_URL}/api/comment/getComments`,

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
};