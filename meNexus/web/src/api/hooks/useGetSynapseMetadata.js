import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetSynapseMetadata = () => {
    console.log("useGetSynapseMetadata called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapseMetadata = async () => {
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_METADATA,
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