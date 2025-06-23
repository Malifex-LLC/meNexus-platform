import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetAllPosts = () => {
    console.log("useGetAllPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getAllPosts = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_ALL_POSTS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getAllPosts,
        posts: data,
        loading,
        error};
};

export default useGetAllPosts;