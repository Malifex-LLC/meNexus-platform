import * as secp from '@noble/secp256k1';

// Verify the challenge to signature to authenticate via private key
export async function verifySignature(signature, challenge, publicKey) {
    //const publicKeyHex = secp.etc.bytesToHex(publicKey);
    console.log('verifySignature called for:', "signature: ", signature, "challenge: ",  challenge, "publicKey: ", publicKey);
    try {
        return secp.verify(signature, challenge, publicKey);

    } catch (error) {
        console.log(error);
    }
}

// Generates cryptographic public/private key pairs
// Not the preferred method as its generated on server instead of client
// security risk for privateKey...used mainly to convert meNexus-legacy accounts to PKI
export async function generateCryptoKeys() {
    console.log('generateCryptoKeys cryptoUtils called');
    try {
        // Generate private key
        const privateKey = secp.utils.randomPrivateKey();

        // Generate public key (compressed format by default)
        const publicKey = secp.getPublicKey(privateKey, true);

        // Convert Uint8Array keys to hex strings
        const publicKeyHex = secp.etc.bytesToHex(publicKey);
        const privateKeyHex = secp.etc.bytesToHex(privateKey);

        console.log('publicKeyHex: ', publicKeyHex, 'privateKeyHex: ', privateKeyHex);

        // Return private key (user must securely store it)
        return {
            publicKey: publicKeyHex,
            privateKey: privateKeyHex,
        };
    } catch (error) {
        console.error('Error generating crypto keys:', error);
        throw error;
    }
}