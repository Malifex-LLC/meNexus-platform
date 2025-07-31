// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetChannelChatMessages = () => {
    console.log("useGetChannelChats called");
    const { data, loading, error, sendRequest } = useAxios();

    const getChannelChatMessages = async (channel) => {
        console.log("getChannelChats called for board:", channel);

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_CHANNEL_CHAT_MESSAGES,
            params: {channel},
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getChannelChatMessages,
        data,
        loading,
        error
    };
};

export default useGetChannelChatMessages;