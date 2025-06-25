// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";
import {replaceParams} from "../../utils/apiUtils.js";


const useGetPosts = () => {
    console.log("useGetPost called");
    const { data, loading, error, sendRequest } = useAxios();

    const getPost = async (postId) => {
        const url = replaceParams(ENDPOINTS.GET_POST, {postId});

        try {
            const response = await sendRequest({
                method: "GET",
                url: url,
                params: { postId },
                withCredentials: true
            });

            console.log('useGetPost() response', response.data);
            return response.data;
        } catch (error) {
            console.error(error);
        }
    };

    return {
        getPost,
        post: data,
        loading,
        error};
};

export default useGetPosts;