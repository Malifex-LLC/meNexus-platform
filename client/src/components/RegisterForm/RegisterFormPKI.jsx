import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './RegisterForm.css'
import useCreateUser from '../../api/hooks/useCreateUser.js'
import useGenerateCryptoKeys from "../../api/hooks/useGenerateCryptoKeys.js";

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
        } catch (error) {
            console.error(error);
            setError("Failed to generate CryptoKeys");
        }


    }

    return (
        <div className="register__main-content">
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