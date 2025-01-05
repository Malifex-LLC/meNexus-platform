import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './RegisterForm.css'
import useCreateUser from '../../api/hooks/useCreateUser.js'

const RegisterForm = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [handle, setHandle] = useState('');
    const [display_name, setDisplay_name] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();
    const { createUser } = useCreateUser();

    const handleSubmit = async (event) => {
        event.preventDefault();
        // Checking to make sure all fields have been filled out
        if (!email || !password || !handle || !display_name) {
            setError('All fields are required');
            return;
        }

        try {
            const userData = { email, password, handle, display_name };
            const response = await createUser(userData);
            if (response.status === 200) {
                // Redirect to login page after successful registration
                navigate('/login');
            } else {
                setError('Failed to create user');
            }
        } catch (error) {
            console.error(error);
            setError(error);
        }
    }

    return (
        <div className="register__main-content">
            <form className='register__form' onSubmit={handleSubmit}>
                <label>
                    Email:
                    <input
                        type="email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)} />
                </label>
                <br />
                <label>
                    Password:
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)} />
                </label>
                <br />
                <label>
                    Handle:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)} />
                </label>
                <br />
                <label>
                    Display Name:
                    <input
                        type="text"
                        value={display_name}
                        onChange={(e) => setDisplay_name(e.target.value)} />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button type="submit">Register</button>
            </form>
        </div>
    );
};

export default RegisterForm;