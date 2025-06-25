// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../../config/endpoints.js";

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