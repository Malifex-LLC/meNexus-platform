// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js';

const useLogout = () => {
    console.log('useLogout called');
    const { data, loading, error, sendRequest } = useAxios();

    const logout = async () => {
        try {
            const url = ENDPOINTS.LOGOUT;
            const response = await sendRequest( {
                method: 'POST',
                url: url,
                withCredentials: true
            });

            return response;
        } catch (err) {
            console.log('Error in useLogout', err);
            throw err;
        }
    }
    return {
        logout,
        logoutData: data,
        loading,
        error
    };
}

export default useLogout;