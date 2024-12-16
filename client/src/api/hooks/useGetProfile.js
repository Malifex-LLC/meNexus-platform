import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";
import { replaceParams } from "../../utils/apiUtils";

const useGetProfile = () => {
    const { data, loading, error, sendRequest } = useAxios();

    const getProfile = async (handle) => {
        const url = replaceParams(ENDPOINTS.GET_PROFILE, {handle});
        const response = await sendRequest({
            method: 'GET',
            url: url
        });

        return response.data;
    };

    return {
        getProfile,
        profile: data,
        loading,
        error
    };
};

export default useGetProfile;