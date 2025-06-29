// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import {ENDPOINTS} from '../config.js'

const useCreateUser = () => {
    console.log('useCreateUser called');
    const { sendRequest, loading, error } = useAxios();

    const createUser = async (userData) => {
        try {
            const response = await sendRequest({
                method: 'POST',
                url: ENDPOINTS.CREATE_USER,
                data: userData,
            });
            console.log("User created", response);
            return response;
        } catch (error) {
            console.error("Error creating user:", error);
        }
    };

    return {
        createUser,
        loading,
        error
    };
};

export default useCreateUser;