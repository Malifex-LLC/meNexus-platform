import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemotePosts = () => {
    console.log("useFetchRemotePosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemotePosts = async (synapsePublicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_POSTS,
            params: {synapsePublicKey},
            withCredentials: true,
        });

        return response.data;
    };

    return {
        fetchRemotePosts,
        posts: data,
        loading,
        error};
};

export default useFetchRemotePosts;