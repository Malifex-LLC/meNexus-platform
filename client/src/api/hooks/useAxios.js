import { useState } from 'react';
import axios from 'axios';


const useAxios = () => {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    // Function to make API requests
    const sendRequest = async ({ method, url, data: bodyData, params = {} }) => {
        setLoading(true); // Start loading state
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
