import useAxios from './useAxios.js'
import { ENDPOINTS } from '../config.js';

const useLogin = () => {
    console.log('useLogin called');
    const { data, loading, error, sendRequest } = useAxios();

    const login = async (email, password) => {
        try {
            const url = ENDPOINTS.LOGIN;
            const response = await sendRequest( {
                method: 'POST',
                url: url,
                data: { email, password },
                withCredentials: true,
            });

            return response;
        } catch (err) {
            console.error('Error in login:', err);
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