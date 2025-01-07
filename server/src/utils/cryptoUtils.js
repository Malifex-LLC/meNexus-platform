import * as secp from '@noble/secp256k1';

export async function verifySignature(signature, challenge, publicKey) {
    //const publicKeyHex = secp.etc.bytesToHex(publicKey);
    console.log('verifySignature called for:', "signature: ", signature, "challenge: ",  challenge, "publicKey: ", publicKey);
    try {
        return secp.verify(signature, challenge, publicKey);

    } catch (error) {
        console.log(error);
    }
}