import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetUserPosts = () => {
    console.log("useGetUserPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getUserPosts = async (publicKey) => {
        console.log("getUserPosts called for public key: ", publicKey);
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_USER_POSTS,
            params: {publicKey},
            withCredentials: true
        });

        return response.data;
    };

    return {
        getUserPosts,
        posts: data,
        loading,
        error};
};

export default useGetUserPosts;