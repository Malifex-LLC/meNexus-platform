import useAxios from './useAxios.js';
import {ENDPOINTS} from '../../config/endpoints.js'

const useUpdateConversationParticipants = () => {
    console.log('useUpdateConversationParticipants called');
    const { sendRequest, loading, error } = useAxios();

    const updateConversationParticipants = async (conversation_id, participants) => {
        try {
            const response = await sendRequest({
                method: 'PUT',
                url: ENDPOINTS.UPDATE_CONVERSATION_PARTICIPANTS,
                data: {conversation_id, participants},
                withCredentials: true
            });
            console.log('Participants updated', response);
            return response;
        } catch (error) {
            console.error(error);
        };
    };

    return {
        updateConversationParticipants,
        loading,
        error
    };
};

export default useUpdateConversationParticipants;