import crypto from 'crypto';
import User from "../models/user.js" ;
import { storePublicKeyInDB, getUserIdByPublicKeyInDB, getAllPublicKeysInDB } from '#src/orbitdb/userPublicKeys.js'
import { verifySignature, generateCryptoKeysUtil } from '#utils/cryptoUtils.js'

// Account registration logic
export const createUser = async (req, res) => {
    try {
        const { publicKey, handle, display_name } = req.body;
        console.log("Received data:", { publicKey, handle, display_name });

        // Validate required fields
        if (!publicKey || !handle || !display_name) {
            console.log("Missing required fields.");
            return res.status(400).json({ error: 'All fields are required' });
        }

        // Check if the handle is already used by another user
        const existingUserByHandle = await User.getUserByHandle(handle);
        console.log("Existing user by handle:", existingUserByHandle);

        if (existingUserByHandle) {
            console.log("Handle is already taken.");
            return res.status(400).json({ error: 'Handle is already taken' });
        }

        // Call the createUser function from the User model
        const newUserId = await User.createUser(publicKey, handle, display_name);
        console.log("New user created with ID:", newUserId);

        // Return a success response
        return res.status(200).json({ message: 'User created successfully', user_id: newUserId });
    } catch (error) {
        console.error("Error in /createUser:", error);
        return res.status(500).json({ error: 'Failed to create user'});
    }
}

// Generates cryptographic public/private key pairs
// Not the preferred method as its generated on api instead of web
// security risk for privateKey...used mainly to convert meNexus-legacy accounts to PKI
export const generateCryptoKeys = async (req, res) => {
    const newCryptoKeys = await generateCryptoKeysUtil()
    return res.status(200).json(newCryptoKeys);
}

// Stores provided publicKey and associates provided userId
// used mainly to convert meNexus-legacy accounts to PKI
export const storePublicKey = async (req, res) => {
    const {userId, publicKey} = req.query;
    if (!userId || !publicKey) {
        return res.status(400).json({ error: 'No userId or publicKey provided' });
    }

    try {
        await storePublicKeyInDB(userId, publicKey);
        return res.status(200).json({ message: `userId: ${userId} and publicKey: ${publicKey} stored successfully` });
    } catch (error) {
        console.error("Error in /storePublicKey:", error);
    }
}

// Retrieves a user_id associated with provided public key
export const getUserIdByPublicKey = async (req, res) => {
    console.log("getUserIdByPublicKey called for: ", req.query.publicKey);
    const publicKey = req.query.publicKey;

    try {
        const userId = await getUserIdByPublicKeyInDB(publicKey);
        if (userId) {
            console.log(`userId: ${userId}`);
            res.status(200).json(userId)
        } else {
            console.log(`No userId found for publicKey ${publicKey}`);
            res.status(404).json({ error: 'No userId found for publicKey: ', publicKey });
        }
    } catch (error) {
        console.error(`Error fetching userId for publicKey ${publicKey}:`, error);
        res.status(500).json({ error: 'Failed to fetch userId' });
    }
}

export const getAllPublicKeys = async (req, res) => {
    console.log("getAllPublicKeys called");
    try {
        const publicKeys = await getAllPublicKeysInDB();
        console.log("Public keys: ", publicKeys);
        return res.status(200).json(publicKeys);
    } catch (error) {
        console.error("Error in /getAllPublicKeys called", error);
    }
}

// Provide a crypto challenge for user to sign
export const getCryptoChallenge = (req, res) => {
    const challenge = crypto.randomBytes(32).toString('hex'); // Generate challenge

    // Store the challenge in the session for later verification
    req.session.challenge = challenge;
    res.status(200).json({ challenge });
};

// Verify the challenge to signature to authenticate via private key
export const verifyCryptoSignature = async (req, res) => {
    const {signature}  = req.body;
    const {challenge} = req.body;
    const {publicKey} = req.body;

    if (!challenge) {
        return res.status(400).json({ error: 'No challenge found. Start login process again.' });
    }

    const isValid = await verifySignature(signature, challenge, publicKey);
    console.log("verifySignature isValid:", isValid);
    try {
        if (isValid) {
            console.log("Signature is valid");

            const user = await User.getUserByPublicKey(publicKey);
            console.log("user: ",  user);

            // Attach session data
            req.session.user = {
                publicKey: user.publicKey,
                handle: user.handle,
                display_name: user.displayName,
            };

            console.log('Session Data:', req.session.user);
            res.status(200).json({message: 'publicKey validated and session user data set'});

        }
    } catch (error) {
        console.error("Error in /verifyCryptoSignature:", error);
    }
};

// Logout logic
export const logout = (req, res) => {
    req.logout((err) => {
        if (err) {
            console.error('Error during logout:', err);
            return res.status(500).json({ error: 'Logout failed' });
        }

        req.session.destroy((destroyErr) => {
            if (destroyErr) {
                console.error('Error destroying session:', destroyErr);
                return res.status(500).json({ error: 'Failed to destroy session' });
            }

            // Clear the session cookie
            res.clearCookie('connect.sid', { path: '/' });
            console.log('User successfully logged out and session cleared.');
            return res.status(200).json({ message: 'Logged out successfully' });
        });
    });
}

export const updateAccountSettings = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }
    // TODO updateAccountSettings needs to be updated to support updating public/private keys instead of email/password
}

export default {
    createUser,
    generateCryptoKeys,
    storePublicKey,
    getUserIdByPublicKey,
    getAllPublicKeys,
    getCryptoChallenge,
    verifyCryptoSignature,
    logout,
    updateAccountSettings
}