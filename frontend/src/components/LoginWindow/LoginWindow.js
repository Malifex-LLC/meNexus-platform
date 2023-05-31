import React, { useState } from 'react';
import axios from 'axios';
import {useNavigate} from "react-router-dom";

const LoginWindow = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();


    function handleSubmit(event) {
        event.preventDefault();
        //Checking to make sure all fields have been filled out
        if (!email || !password) {
            setError('All fields are required');
            return;
        }

        const credentials = { email, password };
        axios.post('/login', JSON.stringify(credentials), {
            headers: {
                'Content-Type': 'application/json'
            }
        })
            .then(res => {
                if (res.data.success) {
                    // Store user data in local storage or in context, etc.
                    navigate('/home');
                } else {
                    setError('Email or password is incorrect.');
                }
            })
            .catch(error => {
                console.log(error);
            });
    }

    return (
        <div className="LoginWindow">
            <form onSubmit={handleSubmit}>
                <label>
                    Email:
                    <input
                        type="email"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                    />
                </label>
                <br />
                <label>
                    Password:
                    <input
                        type="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button type="submit">Login</button>
            </form>
        </div>
    );
};

export default LoginWindow;