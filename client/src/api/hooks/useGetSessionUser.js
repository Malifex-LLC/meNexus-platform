import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetSessionUser = () => {
    console.log('useGetSessionUser called');
    const { data, loading, error, sendRequest } = useAxios();

    const getSessionUser = async () => {
        try {
            const url = ENDPOINTS.GET_CURRENT_USER;
            const response = await sendRequest( {
                method: 'GET',
                url: url,
                withCredentials: true,
            });

            return response;
        } catch (err) {
            console.error('Error in getSessionUser:', err);
            throw err;
        }
    }

    return {
        getSessionUser,
        sessionData:
        data,
        loading,
        error
    };
}

export default useGetSessionUser;