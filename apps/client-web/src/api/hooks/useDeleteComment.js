// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from "./useAxios.js";
import {ENDPOINTS} from '../config.js'
import { replaceParams } from "../../utils/apiUtils.js";

const useDeleteComment = (refreshComments) => {
    const {sendRequest, loading, error} = useAxios();

    const handleDeleteComment = async (commentId) => {
        console.log("handleDeleteComment called for comment_id: ", commentId);
        try {
            const url = replaceParams(ENDPOINTS.DELETE_COMMENT, {commentId});
            await sendRequest( {
                method: "DELETE",
                url: url,
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

export default useDeleteComment;