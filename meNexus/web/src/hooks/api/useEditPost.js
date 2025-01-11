import { useState } from "react";
import useAxios from "./useAxios.js";
import { ENDPOINTS } from "../../config/endpoints.js";
import { replaceParams } from "../../utils/apiUtils.js";

const useEditPost = (refreshPosts) => {
    const [editingPostId, setEditingPostId] = useState(null);
    const [editedPostContent, setEditedPostContent] = useState("");
    const { sendRequest, loading, error } = useAxios();

    const handleEdit = (postId, posts) => {
        const postToEdit = posts.find((post) => post.post_id === postId);
        if (postToEdit) {
            setEditingPostId(postId); // Save the post ID to state
            setEditedPostContent(postToEdit.content); // Set the content for editing
        }
    };

    const handleSave = async () => {
        try {
            const url = replaceParams(ENDPOINTS.UPDATE_POST, { postId: editingPostId });
            await sendRequest({
                method: "PUT",
                url: url,
                data: { content: editedPostContent }, // Use the updated content
            });

            setEditingPostId(null);
            setEditedPostContent("");
            refreshPosts(); // Refresh posts after saving
        } catch (error) {
            console.error("Error updating post:", error);
        }
    };

    return {
        handleEdit,
        handleSave,
        editingPostId,
        editedPostContent,
        setEditedPostContent,
        loading,
        error,
    };
};

export default useEditPost;