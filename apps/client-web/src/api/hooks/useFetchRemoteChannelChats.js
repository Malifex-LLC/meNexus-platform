// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteChannelChats = () => {
    console.log("useFetchRemoteChannelChats called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteChannelChats = async (synapsePublicKey, channel) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_CHANNEL_CHATS,
            params: {synapsePublicKey, channel},
            withCredentials: true,
        });
        console.log('fetchRemoteChannelChats response ', response);
        return response.data;
    };

    return {
        fetchRemoteChannelChats,
        data,
        loading,
        error};
};

export default useFetchRemoteChannelChats;