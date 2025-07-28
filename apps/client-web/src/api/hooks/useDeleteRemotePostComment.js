// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from "./useAxios.js";
import { ENDPOINTS } from '../config.js'

const useDeleteRemotePostComment = (refreshComments, synapsePublicKey) => {
    const {sendRequest, loading, error} = useAxios();

    const handleDeleteComment = async (commentId) => {
        console.log("handleDeleteComment called for commentId: ", commentId);
        try {
            await sendRequest( {
                method: "DELETE",
                url: ENDPOINTS.DELETE_REMOTE_POST_COMMENT,
                data: { commentId, synapsePublicKey },
                withCredentials: true
            });

            refreshComments();
        } catch (error) {
            console.error("Error deleting comment:", error);
        }
    };

    return {
        handleDeleteComment,
        loading,
        error,
    };
};

export default useDeleteRemotePostComment;