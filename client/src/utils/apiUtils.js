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

// Refreshes rendered comments, commonly after a comment is created or deleted
export const refreshComments = async (resource_type, resource_id, getComments, setComments) => {
    try {
        const userCommentsData = await getComments(resource_type, resource_id);
        setComments(userCommentsData);
    } catch (error) {
        console.log("Error refreshing posts:", error);
    }
};

// Refreshes rendered messages, commonly after a message is created or deleted
export const refreshMessages = async (getMessages, conversation_id, setMessages) => {
    try {
        const messagesData = await getMessages(conversation_id);
        setMessages(messagesData);
    } catch (error) {
        console.log("Error refreshing messages:", error);
    }
};