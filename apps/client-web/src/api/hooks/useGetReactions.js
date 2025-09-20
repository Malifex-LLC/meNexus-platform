// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetReactions = () => {
    console.log("useGetReactions called");
    const {data, loading, error, sendRequest} = useAxios();

    const getReactions = async (resourceType, resourceId) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.GET_REACTIONS,
            params: {resourceType, resourceId},
            withCredentials: true,
        });
        console.log("useGetReactions response: ", response);
        return response.data;

    }

    return {
        getReactions,
        data,
        loading,
        error
    }
}

export default useGetReactions;