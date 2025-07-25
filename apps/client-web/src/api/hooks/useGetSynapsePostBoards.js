// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetSynapsePostBoards = () => {
    console.log("useGetSynapsePostBoards called");
    const { data, loading, error, sendRequest } = useAxios();

    const getSynapsePostBoards = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_SYNAPSE_POST_BOARDS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getSynapsePostBoards,
        posts: data,
        loading,
        error};
};

export default useGetSynapsePostBoards;