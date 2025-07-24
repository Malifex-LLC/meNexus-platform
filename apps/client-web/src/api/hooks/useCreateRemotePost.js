// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateRemotePost = (refreshPosts) => {
    console.log("useCreateRemotePost called");
    const { sendRequest, loading, error } = useAxios();

    const createRemotePost = async (post) => {
        console.log('createSynapsePost called for post:', post);
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_REMOTE_POST,
                data: post
            });
            console.log("Post created:", response);
            if (refreshPosts) {
                refreshPosts(); // Refresh the posts if a callback is provided
            }
            return response;
        } catch (err) {
            console.error("Error creating post:", err);
        }
    };

    return {
        createRemotePost,
        loading,
        error
    };
};

export default useCreateRemotePost;