// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetPosts = () => {
    console.log("useGetUserPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getPosts = async () => {

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_POSTS,
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getPosts,
        posts: data,
        loading,
        error};
};

export default useGetPosts;