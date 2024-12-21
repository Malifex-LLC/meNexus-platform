import useAxios from "./useAxios.js";
import {ENDPOINTS} from '../config.js'
import { replaceParams } from "../../utils/apiUtils";

const useDeleteComment = (refreshComments) => {
    const {sendRequest, loading, error} = useAxios();

    const handleDeleteComment = async (comment_id) => {
        console.log("handleDeleteComment called for comment_id: ", comment_id);
        try {
            const url = replaceParams(ENDPOINTS.DELETE_COMMENT, {comment_id});
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