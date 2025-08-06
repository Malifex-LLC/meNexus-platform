// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useJoinRemoteSynapse = () => {
    console.log("useJoinRemoteSynapse called");
    const { data, loading, error, sendRequest } = useAxios();

    const joinRemoteSynapse = async (publicKey, synapsePublicKey) => {
        console.log("joinRemoteSynapse called for Synapse: ", synapsePublicKey);
        const response = await sendRequest({
            method: 'POST',
            url: ENDPOINTS.JOIN_REMOTE_SYNAPSE,
            params: {publicKey, synapsePublicKey},
            withCredentials: true
        });

        return response.data;
    };

    return {
        joinRemoteSynapse,
        data,
        loading,
        error};
};

export default useJoinRemoteSynapse;