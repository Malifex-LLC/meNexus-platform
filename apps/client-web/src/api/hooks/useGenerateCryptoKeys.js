// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import * as secp from '@noble/secp256k1';

const useGenerateCryptoKeys = () => {

    const generateCryptoKeys = async () => {
        try {
            // Generate private key
            const privateKey = secp.utils.randomPrivateKey();

            // Generate public key (compressed format by default)
            const publicKey = secp.getPublicKey(privateKey, true);

            // Convert Uint8Array keys to hex strings
            const publicKeyHex = secp.etc.bytesToHex(publicKey);
            const privateKeyHex = secp.etc.bytesToHex(privateKey);

            // Return private key (user must securely store it)
            return {
                publicKey: publicKeyHex,
                privateKey: privateKeyHex,
            };
        } catch (error) {
            console.error('Error generating crypto keys:', error);
            throw error;
        }
    };

    return {
        generateCryptoKeys,

    };
}

export default useGenerateCryptoKeys;