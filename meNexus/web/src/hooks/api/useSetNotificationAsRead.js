import useAxios from './useAxios.js'
import { ENDPOINTS } from '../../config/endpoints.js'

const useSetNotificationAsRead = () => {
    console.log('useSetNotificationAsRead called');
    const {data, loading, error, sendRequest} = useAxios();

    const setNotificationAsRead = async (notification_id) => {
        const response = await sendRequest({
            method: 'PUT',
            url: ENDPOINTS.SET_NOTIFICATION_AS_READ,
            data: {notification_id},
            withCredentials: true
        });
        console.log("Successfully updated notification", response);
    }

    return {
        setNotificationAsRead: setNotificationAsRead,
        data,
        loading,
        error
    }
};

export default useSetNotificationAsRead;