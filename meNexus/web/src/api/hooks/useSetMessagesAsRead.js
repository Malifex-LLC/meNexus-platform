// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js'

const useSetMessagesAsRead = () => {
    console.log('useSetMessagesAsRead called');
    const { sendRequest, loading, error } = useAxios();

    const setMessagesAsRead = async (conversation_id) => {
        const response = await sendRequest({
            method: 'PUT',
            url: ENDPOINTS.SET_MESSAGES_AS_READ,
            data: {conversation_id},
            withCredentials: true,
        });
        console.log('Successfully set messages as read', response);
    }

    return {
        setMessagesAsRead,
        loading,
        error
    }
};

export default useSetMessagesAsRead;