// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";

const useFetchRemoteBoardPosts = () => {
    console.log("useFetchRemoteBoardPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const fetchRemoteBoardPosts = async (synapsePublicKey, board) => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.FETCH_REMOTE_POSTS,
            params: {synapsePublicKey, board},
            withCredentials: true,
        });

        return response.data;
    };

    return {
        fetchRemoteBoardPosts,
        posts: data,
        loading,
        error};
};

export default useFetchRemoteBoardPosts;