import useAxios from './useAxios.js';
import {ENDPOINTS} from '../../config/endpoints.js'

const useCreatePost = (refreshPosts) => {
    console.log("useCreatePost called");
    const { sendRequest, loading, error } = useAxios();

    const createPost = async (post) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_POST,
                data: post,
            });
            console.log("Post created:", response);
            if (refreshPosts) {
                refreshPosts(); // Refresh the posts if a callback is provided
            }
        } catch (err) {
            console.error("Error creating post:", err);
        }
    };

    return {
        createPost,
        loading,
        error
    };
};

export default useCreatePost;