// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js'

const useCreateChatMessage = (refreshChat) => {
    console.log("useCreatePost called");
    const { sendRequest, loading, error } = useAxios();

    const createChatMessage = async (chatMessage) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_CHAT_MESSAGE,
                data: chatMessage,
            });
            console.log("Post created:", response);
            if (refreshChat) {
                refreshChat();
            }
            return response;
        } catch (err) {
            console.error("Error creating post:", err);
        }
    };

    return {
        createChatMessage,
        loading,
        error
    };
};

export default useCreateChatMessage;