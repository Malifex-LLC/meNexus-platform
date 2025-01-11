import useAxios from './useAxios.js';
import { ENDPOINTS } from '../../config/endpoints.js';
import { replaceParams } from "../../utils/apiUtils.js";

const useFollowActions = () => {
    const { sendRequest, loading, error } = useAxios();

    const followUser = async (followed_id) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.FOLLOW_USER,
                data: {followed_id: followed_id},
                withCredentials: true,
            });

            return response.data;
        } catch (err) {
            console.error('Error following user: ', err);
            throw err;
        }
    };

    const unfollowUser = async (followed_id) => {
        try {
            const url = replaceParams(ENDPOINTS.UNFOLLOW_USER, {followed_id});

            const response = await sendRequest({
                method: 'DELETE',
                url: url,
                data: {followed_id: followed_id},
                withCredentials: true,
            });
            return response.data;
        } catch (err) {
            console.error('Error unfollowing user:', err);
            throw err;
        }
    };

    const followCheck = async (followed_id) => {
        console.log("followCheck called for user:", followed_id);
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FOLLOW_CHECK,
            params: {followed_id: followed_id},
            withCredentials: true,
        });
        return response.data.isFollowing;
    };

    return { followUser, unfollowUser, followCheck, loading, error };
};

export default useFollowActions;