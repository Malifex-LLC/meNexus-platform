// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteReactions = () => {
    console.log("useFetchRemoteReactions called");
    const {data, loading, error, sendRequest} = useAxios();

    const fetchRemoteReactions = async (resourceType, resourceId, synapsePublicKey) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_REACTIONS,
            params: {resourceType, resourceId, synapsePublicKey},
            withCredentials: true,
        });
        console.log("useGetReactions response: ", response);
        return response.data;

    }

    return {
        fetchRemoteReactions,
        data,
        loading,
        error
    }
}

export default useFetchRemoteReactions;