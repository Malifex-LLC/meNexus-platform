// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import useAxios from "./useAxios.js";
import { ENDPOINTS } from "../config.js";

const useEditRemoteComment = (refreshComments, synapsePublicKey) => {
    const [editingCommentId, setEditingCommentId] = useState(null);
    const [editedCommentContent, setEditedCommentContent] = useState("");
    const { sendRequest, loading, error } = useAxios();

    const handleCommentEdit = (commentId, comments) => {
        const commentToEdit = comments.find((comment) => comment.comment_id === commentId);
        console.log("commentToEdit", commentToEdit);
        if (commentToEdit) {
            setEditingCommentId(commentToEdit.comment_id);
            setEditedCommentContent(commentToEdit.comment_content);
        }
    };

    const handleCommentSave = async () => {
        try {
            await sendRequest( {
                method: "PUT",
                url: ENDPOINTS.UPDATE_REMOTE_POST_COMMENT,
                data: {commentId: editingCommentId, content: editedCommentContent, synapsePublicKey},
            });

            setEditingCommentId(null);
            setEditedCommentContent("");
            refreshComments();
        } catch (error) {
            console.error("Error updating comment", error);
        }
    };

    return {
        handleCommentEdit,
        handleCommentSave,
        editingCommentId,
        editedCommentContent,
        setEditedCommentContent,
        loading,
        error,
    }
}

export default useEditRemoteComment;