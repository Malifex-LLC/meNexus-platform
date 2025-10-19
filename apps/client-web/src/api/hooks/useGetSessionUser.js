// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useGetSessionUser = () => {
    console.log('useGetSessionUser called');
    const { data, loading, error, sendRequest } = useAxios();

    const getSessionUser = async () => {
        try {
            const response = await sendRequest( {
                method: 'GET',
                url: ENDPOINTS.GET_SESSION_USER,
                withCredentials: true,
            });
            console.log('getSessionUser response: ', response);
            return response;
        } catch (err) {
            console.error('Error in getSessionUser:', err);
            throw err;
        }
    }

    return {
        getSessionUser,
        sessionData:
        data,
        loading,
        error
    };
}

export default useGetSessionUser;