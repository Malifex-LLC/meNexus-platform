import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateMessage = () => {
    console.log("useCreateMessage called");
    const { sendRequest, loading, error } = useAxios();

    const createMessage = async (conversation_id, message) => {
        try {
            const response = await sendRequest( {
                method: "POST",
                url: ENDPOINTS.CREATE_MESSAGE,
                data: {conversation_id, message},
                withCredentials: true
            });
            console.log("Message created", response);
        } catch (err) {
            console.error("Error creating message", err);
        }

    };

    return {
        createMessage,
        loading,
        error
    }
};

export default useCreateMessage;