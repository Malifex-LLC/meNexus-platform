import { useState } from 'react';
import axios from 'axios';
import { useNavigate } from 'react-router-dom';

const LoginForm = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();

    const handleSubmit = async (event) => {
        event.preventDefault();

        // Validate fields
        if (!email || !password) {
            setError('All fields are required');
            return;
        }

        try {
            // Attempt to login
            const loginResponse = await axios.post('http://localhost:3001/login', { email, password }, {withCredentials: true});
            console.log('Login Response:', loginResponse);
            if (loginResponse.status === 200) {
                console.log("Login successful. Fetching session data...");

                // Fetch current user session from the server
                console.log('Making request to getCurrentUser');
                const sessionResponse = await axios.get('http://localhost:3001/getCurrentUser', { withCredentials: true });
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
        <div className="LoginWindow">
            <form onSubmit={handleSubmit}>
                <label>
                    Email:
                    <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} />
                </label>
                <br />
                <label>
                    Password:
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button type="submit">Login</button>
            </form>
        </div>
    );
};

export default LoginForm;
