// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useFetchRemoteSynapseMembers = () => {
    console.log("useFetchRemoteSynapseMembers called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteSynapseMembers = async (synapsePublicKey) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_SYNAPSE_MEMBERS,
            params: { synapsePublicKey },
            withCredentials: true,
        });
        console.log('fetchRemoteSynapseMembers response: ', response);
        return response.data.members;
    };

    return {
        fetchRemoteSynapseMembers,
        data,
        loading,
        error};
};

export default useFetchRemoteSynapseMembers;