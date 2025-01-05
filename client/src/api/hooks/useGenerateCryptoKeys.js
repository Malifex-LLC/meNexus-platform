import useAxios from './useAxios';
import { ENDPOINTS } from '../config';
import * as secp from '@noble/secp256k1';

const useGenerateCryptoKeys = () => {
    const { sendRequest, loading, error } = useAxios();

    const generateCryptoKeys = async () => {
        try {
            // Generate private key
            const privateKey = secp.utils.randomPrivateKey();

            // Generate public key (compressed format by default)
            const publicKey = secp.getPublicKey(privateKey, true);

            // Send public key to the backend
            // await sendRequest({
            //     method: 'POST',
            //     url: ENDPOINTS.REGISTER_PUBLIC_KEY,
            //     data: {
            //         publicKey: publicKey, // Send as hex
            //     },
            // });

            // Return private key (user must securely store it)
            return {
                publicKey: publicKey,
                privateKey: privateKey,
            };
        } catch (error) {
            console.error('Error generating crypto keys:', error);
            throw error;
        }
    };

    return {
        generateCryptoKeys,
        loading,
        error
    };
}

export default useGenerateCryptoKeys;