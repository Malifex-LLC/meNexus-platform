// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js';

const useLogin = () => {
    console.log('useLogin called');
    const { data, loading, error, sendRequest } = useAxios();

    const login = async (email, password) => {
        try {
            const response = await sendRequest( {
                method: 'POST',
                url: ENDPOINTS.LOGIN,
                data: { email, password },
                withCredentials: true,
            });

            return response;
        } catch (err) {
            console.error('Error in useLogin:', err);
            throw err;
        }
    }

    return {
        login,
        loginData: data,
        loading,
        error
    };
}

export default useLogin;