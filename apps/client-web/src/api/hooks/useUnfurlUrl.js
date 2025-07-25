import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useUnfurlUrl = () => {
    const { sendRequest, loading, error } = useAxios();

    const unfurlUrl = async (url) => {
        try {
            const response = await sendRequest( {
                method: "POST",
                url: ENDPOINTS.UNFURL_URL,
                data: {url},
                withCredentials: true
            });
            console.log("unfurlUrl response ", response);
            return response.data;
        } catch (err) {
            console.error("Error unfurling url", err);
        }
    }

    return {
        unfurlUrl,
        loading,
        error
    }
}

export default useUnfurlUrl;