// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from "./useAxios.js";
import {ENDPOINTS} from '../config.js'

const useDeleteReaction = () => {
    const {sendRequest, loading, error} = useAxios();

    const deleteReaction = async (publicKey, resourceId, resourceType, reactionType) => {
        console.log("useDeleteReaction called for resourceId: ", resourceId);
        try {
            await sendRequest( {
                method: "POST",
                url: ENDPOINTS.DELETE_REACTION,
                data: {publicKey, resourceId, resourceType, reactionType},
                withCredentials: true
            });

        } catch (error) {
            console.error("Error deleting Reaction:", error);
        }
    };

    return {
        deleteReaction,
        loading,
        error,
    };
};

export default useDeleteReaction;