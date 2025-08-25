// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetUserActivities = () => {
    console.log("useGetUserActivities called");
    const { data, loading, error, sendRequest } = useAxios();

    const getUserActivities = async (publicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_USER_ACTIVITIES,
            params: { publicKey },
            withCredentials: true,
        });
        console.log('getUserActivities response: ', response.data);
        return response.data;
    };

    return {
        getUserActivities,
        data,
        loading,
        error};
};

export default useGetUserActivities;