import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";
import { replaceParams } from "../../utils/apiUtils";

const useGetUserPosts = () => {
    console.log("useGetUserPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getUserPosts = async (handle) => {
        const url = replaceParams(ENDPOINTS.GET_USER_POSTS, {handle});
        const response = await sendRequest({
            method: 'GET',
            url: url
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