import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useVerifyCryptoSignature = () => {
    console.log('useVerifyCryptoSignature called');
    const { sendRequest, loading, error  } = useAxios();

    const verifyCryptoSignature = async (signature, challenge, publicKey) => {
        const response = await sendRequest({
            method: 'POST',
            url: ENDPOINTS.VERIFY_CRYPTO_SIGNATURE,
            data: {signature, challenge, publicKey},
            withCredentials: true
        });
        console.log(response);
        return response;
    };

    return {
        verifyCryptoSignature,
        loading,
        error
    };
};

export default useVerifyCryptoSignature;