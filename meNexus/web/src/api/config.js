export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;
export const WS_BASE_URL = import.meta.env.VITE_WS_BASE_URL;

export const ENDPOINTS = {

    WEBSOCKET: WS_BASE_URL,

    LOGIN: `${API_BASE_URL}/api/auth/login`,
    LOGOUT: `${API_BASE_URL}/api/auth/logout`,

    CREATE_USER: `${API_BASE_URL}/api/auth/createUser`,
    GET_SESSION_USER: `${API_BASE_URL}/api/user/getSessionUser`,
    GET_USER: `${API_BASE_URL}/api/user/getUserById/:user_id`,
    GET_PROFILE: `${API_BASE_URL}/api/user/getProfile/:handle`,

    UPDATE_ACCOUNT_SETTINGS: `${API_BASE_URL}/api/auth/updateAccountSettings`,
    UPDATE_PROFILE_SETTINGS: `${API_BASE_URL}/api/user/updateProfileSettings/:handle`,

    FOLLOW_USER: `${API_BASE_URL}/api/follow/followUser`,
    UNFOLLOW_USER: `${API_BASE_URL}/api/follow/unfollowUser`,
    FOLLOW_CHECK: `${API_BASE_URL}/api/follow/followCheck`,
    GET_FOLLOWER_COUNT: `${API_BASE_URL}/api/follow/getFollowerCount`,
    GET_FOLLOWING_COUNT: `${API_BASE_URL}/api/follow/getFollowingCount`,

    CREATE_POST: `${API_BASE_URL}/api/post/createPost`,
    UPDATE_POST: `${API_BASE_URL}/api/post/updatePost/:postId`,
    DELETE_POST: `${API_BASE_URL}/api/post/deletePost/:postId`,
    GET_POST: `${API_BASE_URL}/api/post/getPost/:postId`,
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

    /* ---- SYNAPSE ROUTES ----------------------------------------------------- */
    GET_SYNAPSE_USERS: `${API_BASE_URL}/synapse/getSynapseUsers`,
    GET_SYNAPSE_POSTS: `${API_BASE_URL}/synapse/getSynapsePosts`,
    GET_SYNAPSE_USER_POSTS: `${API_BASE_URL}/synapse/getSynapseUserPosts`,
};