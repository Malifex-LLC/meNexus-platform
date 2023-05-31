import React, { useState } from 'react';
import axios from 'axios';
import {useNavigate} from "react-router-dom";

const RegisterWindow = () => {
    const [handle, setHandle] = useState('');
    const [username, setUsername] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();


    function  handleSubmit(event) {
        //Prevents page from reloading after submit button is pressed preventing code further execution
        event.preventDefault();
        //Checking to make sure all fields have been filled out
        if (!handle || !username || !email || !password) {
            setError('All fields are required');
            return;
        }
        let credentials = {handle, username, email, password};
        axios.post('/register', JSON.stringify(credentials), {
            headers: {
                'Content-Type': 'application/json'
            }
        }).then(res => {
            //Redirecting users to homepage after successful login
                navigate('/home');
            }).catch(error => {
                console.log(error);
            });
    }

    return (
        <div className="RegisterWindow">
            <form onSubmit={handleSubmit}>
                <label>
                    Handle:
                    <input
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)}
                    />
                </label>
                <br/>
                <label>
                    Username:
                    <input
                        type="text"
                        value={username}
                        onChange={(e) => setUsername(e.target.value)}
                    />
                </label>
                <br/>
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
                <button type="submit">Register</button>
            </form>
        </div>
    );
};

export default RegisterWindow;