// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import useAxios from './useAxios.js';
import { ENDPOINTS } from '../config.js';

const useUpdateAccountSettings = () => {
    const {sendRequest, loading, error } = useAxios();

    const updateAccountSettings = async (updatedFields) => {
        console.log('updateAccountSettings called for fields: ', updatedFields);

        try {
            const response = await sendRequest({
                method: 'PUT',
                url: ENDPOINTS.UPDATE_ACCOUNT_SETTINGS,
                data: updatedFields,
                withCredentials: true
            });
            return response;
        } catch (error) {
            console.error('Error updating account settings: ', error);
            throw error;
        }
    };

    return {
        updateAccountSettings,
        loading,
        error
    };
};

export default useUpdateAccountSettings;