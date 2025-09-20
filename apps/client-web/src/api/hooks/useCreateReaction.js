// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateReaction = () => {
    console.log("useCreateReaction called");
    const { sendRequest, loading, error } = useAxios();

    const createReaction = async (publicKey, resourceId, resourceType, reactionType) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_REACTION,
                data: { publicKey, resourceId, resourceType, reactionType },
            });
            console.log('Reaction created successfully: ', response);
            return response;
        } catch (err) {
            console.error("Error creating reaction:", err);
        }
    };

    return {
        createReaction,
        loading,
        error
    };
};

export default useCreateReaction;