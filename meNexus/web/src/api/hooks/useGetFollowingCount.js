import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetFollowingCount = () => {
    const { sendRequest, loading, error } = useAxios();

    const getFollowingCount = async (publicKey) => {
        try {
            const response = await sendRequest({
                method: 'GET',
                url: ENDPOINTS.GET_FOLLOWING_COUNT,
                params: {publicKey},
                withCredentials: true,
            });

            return response.data;
        } catch (err) {
            console.error('Error getting following count: ', err);
            throw err;
        }
    }

    return { getFollowingCount, loading, error };
};

export default useGetFollowingCount;