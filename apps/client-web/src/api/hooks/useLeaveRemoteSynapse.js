// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useLeaveRemoteSynapse = () => {
    console.log("useLeaveRemoteSynapse called");
    const { data, loading, error, sendRequest } = useAxios();

    const leaveRemoteSynapse = async (publicKey, synapsePublicKey) => {
        console.log("leaveRemoteSynapse called for Synapse: ", synapsePublicKey);
        const response = await sendRequest({
            method: 'POST',
            url: ENDPOINTS.LEAVE_REMOTE_SYNAPSE,
            params: { publicKey, synapsePublicKey },
            withCredentials: true
        });

        return response.data;
    };

    return {
        leaveRemoteSynapse,
        data,
        loading,
        error};
};

export default useLeaveRemoteSynapse;