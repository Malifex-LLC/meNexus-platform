// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useJoinSynapse = () => {
    console.log("useJoinSynapse called");
    const { data, loading, error, sendRequest } = useAxios();

    const joinSynapse = async (publicKey, synapsePublicKey) => {
        console.log("joinSynapse called for Synapse: ", synapsePublicKey);
        const response = await sendRequest({
            method: 'POST',
            url: ENDPOINTS.JOIN_SYNAPSE,
            params: {publicKey, synapsePublicKey},
            withCredentials: true
        });

        return response.data;
    };

    return {
        joinSynapse,
        data,
        loading,
        error};
};

export default useJoinSynapse;