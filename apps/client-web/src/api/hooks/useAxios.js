// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from 'react';
import axios from 'axios';

const useAxios = () => {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    // Function to make API requests
    const sendRequest = async ({ method, url, data: bodyData, params = {}, withCredentials = false }) => {
        setLoading(true); // Start loading state
        setError(null); // Reset error state
        try {
            const response = await axios({
                method, // GET/POST method specifier
                url, // API endpoint
                data: bodyData, // Payload for POST/PUT requests
                params, // Query parameters for GET requests
                withCredentials: withCredentials,
            });

            setData(response); // Update fetched data
            return response; // Return the response
        } catch (err) {
            setError(err.message || 'An error occurred');
            throw err; // Rethrow the error for further handling
        } finally {
            setLoading(false); // End loading state
        }
    };

    return {
        sendRequest,
        data,
        loading,
        error
    };
};

export default useAxios;