// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import useCreateUser from '../../../src/api/hooks/useCreateUser.js'
import useGenerateCryptoKeys from "../../../src/api/hooks/useGenerateCryptoKeys.js";

const RegisterForm = () => {

    const [keysGenerated, setKeysGenerated] = useState(false);
    const [handle, setHandle] = useState('');
    const [display_name, setDisplay_name] = useState('');
    const [error, setError] = useState('');
    const [cryptoKeys, setCryptoKeys] = useState({});
    const navigate = useNavigate();
    const { createUser } = useCreateUser();

    const { generateCryptoKeys } = useGenerateCryptoKeys();

    const handleSubmit = async (event) => {
        event.preventDefault();

        try {
            const newCryptoKeys = await generateCryptoKeys();
            console.log('Generated Keys:', newCryptoKeys); // Debugging
            setCryptoKeys(newCryptoKeys);
            setKeysGenerated(true);
            const publicKey = newCryptoKeys.publicKey;
            console.log('newCryptoKeys.publicKey: ', publicKey);

            const userData = {publicKey, handle, display_name };
            const response = await createUser(userData);
            if (response.status === 200) {
                // Redirect to login page after successful registration
                console.log("Public key: ", publicKey)
            }
        } catch (error) {
            console.error(error);
            setError("Failed to create user");
        }


    }

    return (
        <div className="register__main-content flex flex-col items-center  justify-center p-8 rounded-xl ">
            {keysGenerated && (
                <label className="register__keys">
                    <p>public key: {cryptoKeys.publicKey}</p>
                    <p>private key: {cryptoKeys.privateKey}</p>
                </label>
            )}
            <form className='register__form' onSubmit={handleSubmit}>
                <label>
                    Handle:
                    <input
                        className={`w-full p-2 rounded-md border border-border mb-8 mt-2`}
                        type="text"
                        value={handle}
                        onChange={(e) => setHandle(e.target.value)} />
                </label>
                <br />
                <label>
                    Display Name:
                    <input
                        className={`w-full p-2 rounded-md border border-border mb-8 mt-2`}
                        type="text"
                        value={display_name}
                        onChange={(e) => setDisplay_name(e.target.value)} />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button
                    className={`w-full p-2 rounded-md bg-brand cursor-pointer hover:bg-primary`}
                    type="submit"
                >
                    Register
                </button>
            </form>
        </div>
    );
};

export default RegisterForm;