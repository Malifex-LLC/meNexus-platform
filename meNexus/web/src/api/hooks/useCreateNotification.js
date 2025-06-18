import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateNotification = () => {
    //console.log("useCreateNotification called");
    const { sendRequest, loading, error } = useAxios();

    const createNotification = async ({public_key, actor_public_key, resource_type, resource_id, action}) => {
        console.log('createNotification called for publicKey: ', public_key);
        try {
            const response = await sendRequest({
                method: "POST",
                url: ENDPOINTS.CREATE_NOTIFICATION,
                data: {public_key, actor_public_key, resource_type, resource_id, action},
                withCredentials: true,
            });
            console.log("Successfully created notification", response);
            return response.data;
        } catch (error) {
            console.error("Error creating notification", error);
        }
    };

    return {
        createNotification,
        loading,
        error,
    };
};

export default useCreateNotification;