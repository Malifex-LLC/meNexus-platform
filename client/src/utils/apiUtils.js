// replaceParams is used to construct URLs during API calls using api config ENDPOINTS
export const replaceParams = (url, params) => {
    Object.keys(params).forEach((key) => {
        url = url.replace(`:${key}`, params[key]);
    });
    return url;
};

// Refreshes rendered posts, commonly after a post is created or deleted
export const refreshPosts = async (getUserPosts, handle, setPosts) => {
    try {
        const userPostsData = await getUserPosts(handle);
        setPosts(userPostsData);
    } catch (error) {
        console.log("Error refreshing posts:", error);
    }
};