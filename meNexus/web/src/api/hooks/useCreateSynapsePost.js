import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateSynapsePost = (refreshPosts) => {
    console.log("useCreateSynapsePost called");
    const { sendRequest, loading, error } = useAxios();

    const createSynapsePost = async (post) => {
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
        createSynapsePost,
        loading,
        error
    };
};

export default useCreateSynapsePost;