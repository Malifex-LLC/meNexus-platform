import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteComments = () => {
    console.log("useFetchRemoteComments called");
    const {data, loading, error, sendRequest} = useAxios();

    const fetchRemoteComments = async (resourceType, resourceId, synapsePublicKey) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.GET_COMMENTS,
            params: {resourceType, resourceId, synapsePublicKey},
            withCredentials: true,
        });
        console.log("useGetComments response: ", response);
        return response.data;

    }

    return {
        fetchRemoteComments,
        comments: data,
        loading,
        error
    }
}

export default useFetchRemoteComments;