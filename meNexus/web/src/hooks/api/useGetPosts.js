import useAxios from './useAxios.js';
import { ENDPOINTS } from "../../config/endpoints.js";


const useGetPosts = () => {
    console.log("useGetUserPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getPosts = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_POSTS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getPosts,
        posts: data,
        loading,
        error};
};

export default useGetPosts;