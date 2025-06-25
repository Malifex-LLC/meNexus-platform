// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteComments = () => {
    console.log("useFetchRemoteComments called");
    const {data, loading, error, sendRequest} = useAxios();

    const fetchRemoteComments = async (resourceType, resourceId, synapsePublicKey) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_POST_COMMENTS,
            params: {resourceType, resourceId, synapsePublicKey},
            withCredentials: true,
        });
        console.log("useGetComments response: ", response);
        return response.data;

    }

    return {
        fetchRemoteComments,
        comments: data,
        loading,
        error
    }
}

export default useFetchRemoteComments;