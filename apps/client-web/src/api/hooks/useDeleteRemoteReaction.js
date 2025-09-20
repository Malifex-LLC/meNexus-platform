// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from "./useAxios.js";
import {ENDPOINTS} from '../config.js'

const useDeleteRemoteReaction = () => {
    const {sendRequest, loading, error} = useAxios();

    const deleteRemoteReaction = async (publicKey, resourceId, resourceType, reactionType, synapsePublicKey) => {
        console.log("deleteRemoteReaction called for resourceId: ", resourceId);
        try {
            await sendRequest( {
                method: "POST",
                url: ENDPOINTS.DELETE_REMOTE_REACTION,
                data: {publicKey, resourceId, resourceType, reactionType, synapsePublicKey},
                withCredentials: true
            });

        } catch (error) {
            console.error("Error deleting Reaction:", error);
        }
    };

    return {
        deleteRemoteReaction,
        loading,
        error,
    };
};

export default useDeleteRemoteReaction;