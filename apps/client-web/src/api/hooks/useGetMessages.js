// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetMessages = () => {
    console.log("useGetMessages called");
    const { sendRequest, data, loading, error } = useAxios();

    const getMessages = async (conversation_id) => {

        try {
            const response = await sendRequest({
                method: "GET",
                url: ENDPOINTS.GET_MESSAGES,
                params: { conversation_id },
                withCredentials: true
            });

            console.log('useGetMessages response', response);
            return response.data;
        } catch (error) {
            console.error(error);
        }
    };

    return {
        getMessages,
        messages: data,
        loading,
        error
    };
};

export default useGetMessages;