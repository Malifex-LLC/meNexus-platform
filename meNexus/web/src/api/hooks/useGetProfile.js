import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";
import { replaceParams } from "../../utils/apiUtils.js";

const useGetProfile = () => {
    console.log('useGetProfile called');
    const { data, loading, error, sendRequest } = useAxios();

    const getProfile = async (handle) => {
        const url = replaceParams(ENDPOINTS.GET_PROFILE, {handle});
        const response = await sendRequest({
            method: 'GET',
            url: url
        });

        console.log(response);
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