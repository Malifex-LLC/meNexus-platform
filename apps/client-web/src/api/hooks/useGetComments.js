// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useGetComments = () => {
    console.log("useGetComments called");
    const {data, loading, error, sendRequest} = useAxios();

    const getComments = async (resourceType, resourceId) => {

        const response = await sendRequest( {
            method: 'GET',
            url: ENDPOINTS.GET_COMMENTS,
            params: {resourceType, resourceId},
            withCredentials: true,
        });
        console.log("useGetComments response: ", response);
        return response.data;

    }

    return {
        getComments,
        comments: data,
        loading,
        error
    }
}

export default useGetComments;