// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetFollowerCount = () => {
    const { sendRequest, loading, error } = useAxios();

    const getFollowerCount = async (publicKey) => {
        try {
            const response = await sendRequest({
                method: 'GET',
                url: ENDPOINTS.GET_FOLLOWER_COUNT,
                params: {publicKey},
                withCredentials: true,
            });

            console.log('getFollowerCount response: ', response);
            return response.data;
        } catch (err) {
            console.error('Error getting follower count: ', err);
            throw err;
        }
    }

    return { getFollowerCount, loading, error };
};

export default useGetFollowerCount;