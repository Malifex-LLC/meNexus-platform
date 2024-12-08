import { useState } from 'react';
import axios from 'axios';

const useAxios = () => {
    const [data, setData] = useState(null); // Fetched data or response
    const [loading, setLoading] = useState(false); // Loading state
    const [error, setError] = useState(null); // Error state

    // Function to make API requests
    const sendRequest = async ({ method, url, data: bodyData, params = {} }) => {
        setLoading(true);
        setError(null); // Reset error state
        try {
            const response = await axios({
                method,
                url,
                data: bodyData, // Payload for POST/PUT requests
                params, // Query parameters for GET requests
            });
            setData(response.data); // Update fetched data
            return response.data; // Optionally return the response
        } catch (err) {
            setError(err.message || 'An error occurred');
            throw err; // Rethrow the error for further handling
        } finally {
            setLoading(false); // End loading state
        }
    };

    return { data, loading, error, sendRequest };
};

export default useAxios;
