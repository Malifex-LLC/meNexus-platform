import useAxios from './useAxios';
import { ENDPOINTS } from '../config';

const useGetFollowingCount = () => {
    const { sendRequest, loading, error } = useAxios();

    const getFollowingCount = async (user_id) => {
        try {
            const response = await sendRequest({
                method: 'GET',
                url: ENDPOINTS.GET_FOLLOWING_COUNT,
                params: {user_id},
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