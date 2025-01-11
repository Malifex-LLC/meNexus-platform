import useAxios from './useAxios.js';
import { ENDPOINTS } from '../../config/endpoints.js';

const useGetSessionUser = () => {
    console.log('useGetSessionUser called');
    const { data, loading, error, sendRequest } = useAxios();

    const getSessionUser = async () => {
        try {
            const response = await sendRequest( {
                method: 'GET',
                url: ENDPOINTS.GET_SESSION_USER,
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