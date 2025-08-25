// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useFetchRemoteSynapseUserActivities = () => {
    console.log("useFetchRemoteSynapseUserActivities called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapseUserActivities = async (synapsePublicKey, publicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_USER_ACTIVITIES,
            params: { synapsePublicKey, publicKey },
            withCredentials: true,
        });
        console.log('fetchRemoteSynapseUserActivities response: ', response.data);
        return response.data.activities;
    };

    return {
        fetchRemoteSynapseUserActivities,
        data,
        loading,
        error};
};

export default useFetchRemoteSynapseUserActivities;