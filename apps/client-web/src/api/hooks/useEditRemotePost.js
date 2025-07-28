// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from "react";
import useAxios from "./useAxios.js";
import { ENDPOINTS } from "../config.js";

const useEditRemotePost = (refreshPosts, synapsePublicKey) => {
    const [editingPostId, setEditingPostId] = useState(null);
    const [editedPostContent, setEditedPostContent] = useState("");
    const { sendRequest, loading, error } = useAxios();

    const handleRemoteEdit = (postId, posts) => {
        const postToEdit = posts.find((post) => post.post_id === postId);
        if (postToEdit) {
            setEditingPostId(postId); // Save the post ID to state
            setEditedPostContent(postToEdit.content); // Set the content for editing
        }
    };

    const handleRemoteSave = async () => {
        try {
            await sendRequest({
                method: "PUT",
                url: ENDPOINTS.UPDATE_REMOTE_POST,
                data: { postId:editingPostId, content: editedPostContent, synapsePublicKey }, // Use the updated content
            });

            setEditingPostId(null);
            setEditedPostContent("");
            refreshPosts(); // Refresh posts after saving
        } catch (error) {
            console.error("Error updating post:", error);
        }
    };

    return {
        handleRemoteEdit,
        handleRemoteSave,
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        loading,
        error,
    };
};

export default useEditRemotePost;