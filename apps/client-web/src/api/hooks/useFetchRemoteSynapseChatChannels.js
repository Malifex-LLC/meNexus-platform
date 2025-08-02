// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useFetchRemoteSynapseChatChannels = () => {
    console.log("useFetchRemoteSynapseChatChannels called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapseChatChannels = async (synapsePublicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_CHAT_CHANNELS,
            params: { synapsePublicKey },
            withCredentials: true,
        });

        return response.data;
    };

    return {
        fetchRemoteSynapseChatChannels,
        data,
        loading,
        error};
};

export default useFetchRemoteSynapseChatChannels;