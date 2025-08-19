// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetAllActivities = () => {
    console.log("useGetAllActivities called");
    const { data, loading, error, sendRequest } = useAxios();

    const getAllActivities = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_ALL_ACTIVITIES,
            withCredentials: true,
        });
        console.log('getAllActivities response: ', response.data);
        return response.data;
    };

    return {
        getAllActivities,
        data,
        loading,
        error};
};

export default useGetAllActivities;