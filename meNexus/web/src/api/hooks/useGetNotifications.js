// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js'

const useGetNotifications = () => {
    console.log("useGetNotifications called");
    const { data, loading, error, sendRequest } = useAxios();


    const getNotifications = async () => {
        const response = await sendRequest( {
            method: "GET",
            url: ENDPOINTS.GET_NOTIFICATIONS,
            withCredentials: true,
        });
        console.log("getNotifications response.data: ", response.data);
        return response.data.notifications;
    }

    return {
        getNotifications,
        data,
        loading,
        error
    };
};

export default useGetNotifications;