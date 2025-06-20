import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetSynapsePosts = () => {
    console.log("useGetSynapsePosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapsePosts = async (publicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_POSTS,
            params: {publicKey},
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getSynapsePosts,
        posts: data,
        loading,
        error};
};

export default useGetSynapsePosts;