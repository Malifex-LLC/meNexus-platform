import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateConversation = () => {
    console.log('useCreateConversation called');
    const { sendRequest, loading, error } = useAxios();

    const createConversation = async () => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_CONVERSATION,
                withCredentials: true
            });
            console.log('Conversation created', response);
            return response.data;
        } catch (error) {
            console.error(error);
        }
    };

    return {
        createConversation,
        loading,
        error
    };
};

export default useCreateConversation;