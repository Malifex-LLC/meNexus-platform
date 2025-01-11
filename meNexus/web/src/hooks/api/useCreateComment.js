import useAxios from './useAxios.js';
import {ENDPOINTS} from '../../config/endpoints.js'

const useCreateComment = (refreshComments) => {
    console.log("useCreateComment called");
    const { sendRequest, loading, error } = useAxios();

    const createComment = async (comment) => {
        console.log("creating comment:", comment);
        try {
            const response = await sendRequest ( {
                method: "POST",
                url: ENDPOINTS.CREATE_COMMENT,
                data: comment,
                withCredentials: true
            });
            console.log("Comment created:", response);
            refreshComments();
        } catch (err) {
            console.error("Error creating comment:", err);
        }
    };

    return {
        createComment,
        loading,
        error,
    }

};

export default useCreateComment;