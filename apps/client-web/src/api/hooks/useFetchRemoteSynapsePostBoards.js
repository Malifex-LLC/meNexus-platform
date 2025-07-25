// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useFetchRemoteSynapsePostBoards = () => {
    console.log("useFetchRemoteSynapsePostBoards called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapsePostBoards = async (synapsePublicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_POST_BOARDS,
            params: {synapsePublicKey},
            withCredentials: true,
        });
        console.log("fetchRemoteSynapsePostBoards response ", response);

        return response.data.postBoards;
    };

    return {
        fetchRemoteSynapsePostBoards,
        posts: data,
        loading,
        error};
};

export default useFetchRemoteSynapsePostBoards;