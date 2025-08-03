// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetSynapseChatChannels = () => {
    console.log("useGetSynapseChatChannels called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapseChatChannels = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_CHAT_CHANNELS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getSynapseChatChannels,
        data,
        loading,
        error};
};

export default useGetSynapseChatChannels;