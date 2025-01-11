import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';
import { replaceParams } from "../../utils/apiUtils.js";

const useGetUser = () => {
    console.log("useGetUser called");
    const{ sendRequest, data, loading, error } = useAxios();

    const getUser = async (user_id) => {
        const url = replaceParams(ENDPOINTS.GET_USER, { user_id });

        try {
            const response = await sendRequest({
                method: "GET",
                url: url,
                params: { user_id },
                withCredentials: true
            });

            console.log(response);
            return response.data;
        } catch (error) {
            console.error(error);
        }
    };

    return {
        getUser,
        user: data,
        loading,
        error
    };
};

export default useGetUser;