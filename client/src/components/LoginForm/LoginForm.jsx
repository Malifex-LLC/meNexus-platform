import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import useLogin from "../../api/hooks/useLogin.js";

const LoginForm = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { login, loading: loginLoading, error: loginError } = useLogin();

    const handleSubmit = async (event) => {
        event.preventDefault();

        // Validate fields
        if (!email || !password) {
            setError('All fields are required');
            return;
        }

        try {
            // Attempt to login
            const loginResponse = await login(email, password);
            console.log('Login Response:', loginResponse);
            if (loginResponse.status === 200) {
                console.log("Login successful. Fetching session data...");

                // Fetch current user session from the server
                console.log('Making request to getCurrentUser');
                const sessionResponse = await getSessionUser();
                console.log('Session Response:', sessionResponse);

                if (sessionResponse.status === 200 && sessionResponse.data.handle) {
                    // Navigate to the user's home page based on session data
                    navigate(`/home/${sessionResponse.data.handle}`);
                } else {
                    setError('Failed to retrieve session data.');
                }
            } else {
                setError('Incorrect email or password');
            }
        } catch (error) {
            console.error("Error during login or session retrieval:", error);
            setError('Login failed. Please try again.');
        }
    };

    return (
        <div className="login__main-content flex flex-col items-center justify-center p-8  rounded-2xl">
            <form className='login__form w-full flex flex-col' onSubmit={handleSubmit}>
                <label>
                    Email:
                    <input
                        className={`w-full p-2 rounded-md border border-border`}
                        type="email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)} />
                </label>
                <br />
                <label>
                    Password:
                    <input
                        className={`w-full p-2 rounded-md border border-border`}
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)} />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button
                    className={`w-full p-2 rounded-md bg-brand`}

                    type="submit"
                >
                    Login
                </button>
            </form>
        </div>
    );
};

export default LoginForm;