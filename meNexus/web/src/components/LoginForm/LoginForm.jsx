import './LoginForm.css'
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import useGetSessionUser from '../../api/hooks/useGetSessionUser.js'
import useGetCryptoChallenge from '../../api/hooks/useGetCryptoChallenge.js'
import useVerifyCryptoSignature from '../../api/hooks/useVerifyCryptoSignature.js'
import * as secp from '@noble/secp256k1';

const LoginForm = () => {

    const [privateKey, setPrivateKey] = useState('');
    const [error, setError] = useState('');
    const navigate = useNavigate();

    const { getSessionUser, loading: sessionUserLoading, error: sessionUserError } = useGetSessionUser();
    const { getCryptoChallenge  } = useGetCryptoChallenge();
    const { verifyCryptoSignature } = useVerifyCryptoSignature();

    const handleSubmit = async (event) => {
        event.preventDefault();

        if (!privateKey) {
            setError('Private key is required');
            return;
        }

        // Attempt to log in
        try {
            // Step 1: Fetch challenge from api
            const challengeResponse = await getCryptoChallenge();
            if (challengeResponse.status !== 200) {
                throw new Error('Failed to fetch challenge');
            }

            const { challenge } = challengeResponse.data;

            // Step 2: Sign the challenge using the private key
            const signChallenge = async (privateKey, challenge) => {
                return await secp.signAsync(challenge, privateKey)
            };

            const publicKey = secp.getPublicKey(privateKey);
            const signature = await signChallenge(privateKey, challenge);
            const publicKeyString = secp.etc.bytesToHex(publicKey);
            const signatureString = signature.toCompactHex();

            // Debug logs
            console.log("publicKey: ", publicKey)
            console.log("signature: ", signature)
            console.log("publicKeyString: ", publicKeyString)
            console.log("signatureString: ", signatureString)
            console.log("challenge: ", challenge)

            // Step 3: Send signature to the api
            const verifyResponse = await verifyCryptoSignature(signatureString, challenge, publicKeyString);
            if (verifyResponse.status !== 200) {
                throw new Error('Signature verification failed');
            }

            // Step 4: Fetch user session
            const sessionResponse = await getSessionUser();
            if (sessionResponse.status === 200 && sessionResponse.data.handle) {
                navigate(`/home/${sessionResponse.data.handle}`);
            } else {
                throw new Error('Failed to retrieve session data');
            }

        } catch (error) {
            console.error("Error during login or session retrieval:", error);
            setError('Login failed. Please try again.');
        }
    };

    return (
        <div className="login__main-content">
            <form className='login__form' onSubmit={handleSubmit}>
                <label>
                    Private Key:
                    <textarea
                        value={privateKey}
                        onChange={(e) => setPrivateKey(e.target.value)}
                        placeholder="Paste your private key here"
                    />
                </label>
                <br />
                {error && <p>{error}</p>}
                <button type="submit">Login</button>
            </form>
        </div>
    );
};

export default LoginForm;