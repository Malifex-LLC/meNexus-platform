// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../../config/endpoints.js";

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