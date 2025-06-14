import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetComments = () => {
    console.log("useGetComments called");
    const {data, loading, error, sendRequest} = useAxios();

    const getComments = async (resource_type, resource_id) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.GET_COMMENTS,
            params: {resource_type, resource_id},
            withCredentials: true,
        });
        console.log("useGetComments response: ", response);
        return response.data;

    }

    return {
        getComments,
        comments: data,
        loading,
        error
    }
}

export default useGetComments;