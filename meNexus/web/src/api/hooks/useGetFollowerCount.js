import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetFollowerCount = () => {
    const { sendRequest, loading, error } = useAxios();

    const getFollowerCount = async (user_id) => {
        try {
            const response = await sendRequest({
                method: 'GET',
                url: ENDPOINTS.GET_FOLLOWER_COUNT,
                params: {user_id},
                withCredentials: true,
            });

            return response.data;
        } catch (err) {
            console.error('Error getting follower count: ', err);
            throw err;
        }
    }

    return { getFollowerCount, loading, error };
};

export default useGetFollowerCount;