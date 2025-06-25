// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js'

const useSetNotificationAsRead = () => {
    console.log('useSetNotificationAsRead called');
    const {data, loading, error, sendRequest} = useAxios();

    const setNotificationAsRead = async (notification_id) => {
        const response = await sendRequest({
            method: 'PUT',
            url: ENDPOINTS.SET_NOTIFICATION_AS_READ,
            data: {notification_id},
            withCredentials: true
        });
        console.log("Successfully updated notification", response);
    }

    return {
        setNotificationAsRead: setNotificationAsRead,
        data,
        loading,
        error
    }
};

export default useSetNotificationAsRead;