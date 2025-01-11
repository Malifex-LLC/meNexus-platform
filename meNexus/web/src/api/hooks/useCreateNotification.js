import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateNotification = () => {
    console.log("useCreateNotification called");
    const { sendRequest, loading, error } = useAxios();

    const createNotification = async ({user_id, actor_id, resource_type, resource_id, action}) => {
        try {
            const response = await sendRequest({
                method: "POST",
                url: ENDPOINTS.CREATE_NOTIFICATION,
                data: {user_id, actor_id, resource_type, resource_id, action},
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