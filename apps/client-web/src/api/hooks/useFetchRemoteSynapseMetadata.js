// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteSynapseMetadata = () => {
    console.log("useFetchRemoteSynapseMetadata called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapseMetadata = async (publicKey) => {
        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_METADATA,
            params: {publicKey},
            withCredentials: true,
        });
        return response.data;
    };

    return {
        fetchRemoteSynapseMetadata,
        data,
        loading,
        error,
    };
};

export default useFetchRemoteSynapseMetadata;