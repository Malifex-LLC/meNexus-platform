// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from "./useAxios.js";
import {ENDPOINTS} from '../config.js'
import { replaceParams } from "../../utils/apiUtils.js";

const useDeletePost = (refreshPosts) => {
    const { sendRequest, loading, error } = useAxios();

    const handleDelete = async (postId) => {
        try {
            const url = replaceParams(ENDPOINTS.DELETE_POST, { postId });
            await sendRequest({
                method: "DELETE",
                url: url,
            });

            refreshPosts(); // Refresh posts after deletion
        } catch (error) {
            console.error("Error deleting post:", error);
        }
    };

    return {
        handleDelete,
        loading,
        error,
    };
};

export default useDeletePost;