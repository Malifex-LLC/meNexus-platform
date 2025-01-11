import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetCryptoChallenge = () => {
    console.log('useGetCryptoChallenge called');
    const { sendRequest, loading, error  } = useAxios();

    const getCryptoChallenge = async () => {
        const response = await sendRequest({
            method: "GET",
            url: ENDPOINTS.GET_CRYPTO_CHALLENGE,
            withCredentials: true
        });
        console.log(response);
        return response;
    };

    return {
        getCryptoChallenge,
        loading,
        error
    };
};

export default useGetCryptoChallenge;