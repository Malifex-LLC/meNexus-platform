export const API_BASE_URL = "http://localhost:3001";

export const ENDPOINTS = {
    LOGIN: `${API_BASE_URL}/login`,
    LOGOUT: `${API_BASE_URL}/logout`,
    GET_CURRENT_USER: `${API_BASE_URL}/getCurrentUser`,
    CREATE_POST: `${API_BASE_URL}/createPost`,
    UPDATE_POST: `${API_BASE_URL}/updatePost/:postId`,
    DELETE_POST: `${API_BASE_URL}/deletePost/:postId`,
    GET_PROFILE: `${API_BASE_URL}/getProfile/:handle`,
    GET_USER_POSTS: `${API_BASE_URL}/getUserPosts/:handle`,
    GET_POSTS: `${API_BASE_URL}/getPosts`,
    UPLOAD_PROFILE_PICTURE: `${API_BASE_URL}/settings/uploadProfilePicture`,
    FOLLOW_USER: `${API_BASE_URL}/followUser`,
    UNFOLLOW_USER: `${API_BASE_URL}/unfollowUser`,
    FOLLOW_CHECK: `${API_BASE_URL}/followCheck`,
    SEARCH: `${API_BASE_URL}/search`,
};