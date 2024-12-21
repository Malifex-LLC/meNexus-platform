import { useState } from "react";
import useAxios from "./useAxios.js";
import { ENDPOINTS } from "../config.js";
import { replaceParams } from "../../utils/apiUtils";

const useEditComment = (refreshComments) => {
    const [editingCommentId, setEditingCommentId] = useState(null);
    const [editedCommentContent, setEditedCommentContent] = useState("");
    const { sendRequest, loading, error } = useAxios();

    const handleCommentEdit = (comment_id, comments) => {
        const commentToEdit = comments.find((comment) => comment.comment_id === comment_id);
        console.log("commentToEdit", commentToEdit);
        if (commentToEdit) {
            setEditingCommentId(commentToEdit.comment_id);
            setEditedCommentContent(commentToEdit.comment_content);
        }
    };

    const handleCommentSave = async () => {
        try {
            const url = replaceParams(ENDPOINTS.UPDATE_COMMENT, {comment_id: editingCommentId});
            console.log("handleCommentSave url: ", url);
            await sendRequest( {
                method: "PUT",
                url: url,
                data: {content: editedCommentContent},
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

export default useEditComment;