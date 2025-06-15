import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetSynapseMetadata = () => {
    console.log("useGetSynapseMetadata called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapseMetadata = async (publicKey) => {
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_METADATA,
            params: {publicKey},
            withCredentials: true,
        });
        return response.data;
    };

    return {
        getSynapseMetadata,
        data,
        loading,
        error,
    };
};

export default useGetSynapseMetadata;