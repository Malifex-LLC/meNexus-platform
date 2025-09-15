import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGiphySearch = () => {
    const { sendRequest, loading, error } = useAxios();
    const apiKey = 'atg4tpCdGT3jvNZtVZA3W0AgqfS8S0f5'

    const giphySearch = async (query) => {
        try {
            const response = await sendRequest( {
                method: "GET",
                url: ENDPOINTS.SEARCH_GIPHY,
                params: {
                    api_key: apiKey,
                    q: query,
                    limit: 25,
                    offset: 0,
                    rating: 'r',
                    lang: 'en',
                    bundle: 'messaging_non_clips'
                }
            });
            console.log("giphySearch response ", response.data.data);
            return response.data.data;
        } catch (err) {
            console.error("Error searching Giphy", err);
        }
    }

    return {
        giphySearch,
        loading,
        error
    }
}

export default useGiphySearch;