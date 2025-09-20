// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateRemoteReaction = () => {
    console.log("useCreateRemoteReaction called");
    const { sendRequest, loading, error } = useAxios();

    const createRemoteReaction = async (publicKey, resourceId, resourceType, reactionType, synapsePublicKey) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_REMOTE_REACTION,
                data: { publicKey, resourceId, resourceType, reactionType, synapsePublicKey},
            });
            console.log('Reaction created successfully: ', response);
            return response;
        } catch (err) {
            console.error("Error creating reaction:", err);
        }
    };

    return {
        createRemoteReaction,
        loading,
        error
    };
};

export default useCreateRemoteReaction;