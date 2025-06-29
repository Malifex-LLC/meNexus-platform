// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetUserByHandle = () => {
    console.log("useGetUser called");
    const{ sendRequest, data, loading, error } = useAxios();

    const getUserByHandle = async (handle) => {
        console.log('getUser called for handle: ', handle);
        try {
            const response = await sendRequest({
                method: "GET",
                url: ENDPOINTS.GET_USER_BY_HANDLE,
                params: {handle} ,
                withCredentials: true
            });

            console.log('getUser response: ', response);
            return response.data;
        } catch (error) {
            console.error('getUser error: ', error);
        }
    };

    return {
        getUserByHandle,
        user: data,
        loading,
        error
    };
};

export default useGetUserByHandle;