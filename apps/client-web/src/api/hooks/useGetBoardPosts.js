// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from "../config.js";


const useGetBoardPosts = () => {
    console.log("useGetBoardPosts called");
    const { data, loading, error, sendRequest } = useAxios();

    const getBoardPosts = async (board) => {
        console.log("getBoardPosts called for board:", board);

        const response = await sendRequest({
            method: 'GET',
            url: ENDPOINTS.GET_BOARD_POSTS,
            params: {board},
            withCredentials: true,
        });

        return response.data;
    };

    return {
        getBoardPosts,
        posts: data,
        loading,
        error};
};

export default useGetBoardPosts;