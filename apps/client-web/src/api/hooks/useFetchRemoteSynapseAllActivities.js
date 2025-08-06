// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useFetchRemoteSynapseAllActivities = () => {
    console.log("useFetchRemoteSynapseAllActivities called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapseAllActivities = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_ALL_ACTIVITIES,
            withCredentials: true,
        });
        console.log('fetchRemoteSynapseAllActivities response: ', response.data);
        return response.data;
    };

    return {
        fetchRemoteSynapseAllActivities,
        data,
        loading,
        error};
};

export default useFetchRemoteSynapseAllActivities;