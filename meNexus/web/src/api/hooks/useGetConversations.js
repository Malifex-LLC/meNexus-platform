// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js'

const useGetConversations = () => {
    console.log('useGetConversations called');
    const { sendRequest, data, loading, error } = useAxios();

    const getConversations = async () => {

        try {
            const response = await sendRequest({
                method: 'GET',
                url: ENDPOINTS.GET_CONVERSATIONS,
                withCredentials: true
            });

            console.log('useGetConversations response:', response)
            return response.data;
        } catch (error) {
            console.error(error);
        }
    };

    return {
        getConversations,
        conversations: data,
        loading,
        error
    };
};

export default useGetConversations;