// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetUser = () => {
    console.log("useGetUser called");
    const{ sendRequest, data, loading, error } = useAxios();

    const getUser = async (publicKey) => {
        console.log('getUser called for publicKey: ', publicKey);
        try {
            const response = await sendRequest({
                method: "GET",
                url: ENDPOINTS.GET_USER,
                params: {publicKey} ,
                withCredentials: true
            });

            console.log('getUser response: ', response);
            return response.data;
        } catch (error) {
            console.error('getUser error: ', error);
        }
    };

    return {
        getUser,
        user: data,
        loading,
        error
    };
};

export default useGetUser;