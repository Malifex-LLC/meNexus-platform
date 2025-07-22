// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useLeaveSynapse = () => {
    console.log("useLeaveSynapse called");
    const { data, loading, error, sendRequest } = useAxios();

    const leaveSynapse = async (publicKey, synapsePublicKey) => {
        console.log("leaveSynapse called for Synapse: ", synapsePublicKey);
        const response = await sendRequest({
            method: 'POST',
            url: ENDPOINTS.LEAVE_SYNAPSE,
            params: {publicKey, synapsePublicKey},
            withCredentials: true
        });

        return response.data;
    };

    return {
        leaveSynapse,
        data,
        loading,
        error};
};

export default useLeaveSynapse;