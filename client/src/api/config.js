export const API_BASE_URL = "http://localhost:3001";

export const ENDPOINTS = {
    LOGIN: `${API_BASE_URL}/login`,
    LOGOUT: `${API_BASE_URL}/logout`,
    GET_CURRENT_USER: `${API_BASE_URL}/getCurrentUser`,
    GET_PROFILE: `${API_BASE_URL}/getProfile/:handle`,

    FOLLOW_USER: `${API_BASE_URL}/followUser`,
    UNFOLLOW_USER: `${API_BASE_URL}/unfollowUser`,
    FOLLOW_CHECK: `${API_BASE_URL}/followCheck`,

    CREATE_POST: `${API_BASE_URL}/createPost`,
    UPDATE_POST: `${API_BASE_URL}/updatePost/:postId`,
    DELETE_POST: `${API_BASE_URL}/deletePost/:postId`,
    GET_POSTS: `${API_BASE_URL}/getPosts`,
    GET_USER_POSTS: `${API_BASE_URL}/getUserPosts/:handle`,

    CREATE_COMMENT: `${API_BASE_URL}/createComment`,
    UPDATE_COMMENT: `${API_BASE_URL}/updateComment/:comment_id`,
    DELETE_COMMENT: `${API_BASE_URL}/deleteComment/:comment_id`,
    GET_COMMENTS: `${API_BASE_URL}/getComments`,

    CREATE_NOTIFICATION: `${API_BASE_URL}/createNotification`,
    UPDATE_NOTIFICATION: `${API_BASE_URL}/updateNotification`,
    GET_NOTIFICATIONS: `${API_BASE_URL}/getNotifications`,

    UPLOAD_PROFILE_PICTURE: `${API_BASE_URL}/settings/uploadProfilePicture`,

    SEARCH: `${API_BASE_URL}/search`,
};