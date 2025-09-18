// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { useState } from 'react';
import {Link, useNavigate} from 'react-router-dom';
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
        <div className="flex flex-col items-center  justify-center rounded-xl bg-background/70 font-montserrat">
            {keysGenerated && (
                <div className={'grid grid-cols-2 bg-surface p-4 border border-border rounded-xl '}>
                    <div className={`col-span-2 text-sm border border-accent p-2 font-inter`}>Copy and save your Public/Private Keys securely. Do NOT lose your private key as it is your password and irreplaceable</div>
                    <div className={`flex flex-col mt-2 justify-center text-md xl:text-3xl text-brand font-semibold space-y-8`}>
                        <label>
                            Public Key:
                        </label>
                        <label >
                            Private Key:
                        </label>
                    </div>
                    <div className={`flex flex-col mt-2 space-y-8 justify-center`}>
                        <input
                            className={`text-xl text-foreground border border-border p-2 focus:outline-1 focus:outline-brand/60`}
                            type="text"
                            value={`${cryptoKeys.privateKey}`}
                        />
                        <input
                            className={`text-xl text-foreground border border-border p-2 focus:outline-1 focus:outline-brand/60`}
                            type="email"
                            value={`${cryptoKeys.publicKey}`}
                        />
                    </div>
                    <div className={`col-span-2 text-md text-center mt-4`}>
                        {error && <p>{error}</p>}
                        {!error && (<p>User created successfully! Go <Link to="/login" className={'text-brand'}>Login!</Link></p>)}
                    </div>
                </div>
            )}
            {!keysGenerated && (
                <div className={`p-8`}>
                    <form onSubmit={handleSubmit}>
                        <label>
                            Handle:
                            <input
                                className={`w-full p-2 rounded-md border border-border mb-8 mt-2 bg-surface/70 focus:outline-1 focus:outline-brand/60`}
                                type="text"
                                value={handle}
                                onChange={(e) => setHandle(e.target.value)} />
                        </label>
                        <br />
                        <label>
                            Display Name:
                            <input
                                className={`w-full p-2 rounded-md border border-border mb-8 mt-2 bg-surface/70 focus:outline-1 focus:outline-brand/60`}
                                type="text"
                                value={display_name}
                                onChange={(e) => setDisplay_name(e.target.value)} />
                        </label>
                        <br />
                        {error && <p>{error}</p>}
                        <button
                            className={`w-full p-2 rounded-md bg-brand cursor-pointer hover:bg-brand/70`}
                            type="submit"
                        >
                            Register
                        </button>
                    </form>
                </div>
            )}
        </div>
    );
};

export default RegisterForm;