// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import crypto from 'crypto';
import User from "../models/user.js" ;
import { storePublicKeyInDB, getUserIdByPublicKeyInDB, getAllPublicKeysInDB } from '#src/orbitdb/userPublicKeys.js'
import { verifySignature, generateCryptoKeysUtil } from '#utils/cryptoUtils.js'
import { loadConfig, saveConfig } from '#utils/configUtils.js';
import path from 'path';
import { fileURLToPath } from 'url';
import { mintAccessToken, mintRefreshToken, verifyRefreshToken } from '#utils/jwtUtils.js';
import { SignJWT, jwtVerify } from 'jose';


// Get __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const CONFIG_FILE = path.resolve(__dirname, '../../config/synapse-config.json');

// Account registration logic
// TODO Move createUser to userController? and call createUser by authController?
// TODO Verify that handle is unique across the network
export const createUser = async (req, res) => {
    try {
        const { publicKey, handle, display_name } = req.body;
        console.log("Received data:", { publicKey, handle, display_name });

        // Validate required fields
        if (!publicKey || !handle || !display_name) {
            console.log("Missing required fields.");
            return res.status(400).json({ error: 'All fields are required' });
        }

        // Get local Synapse publicKey
        const metadata = await loadConfig(CONFIG_FILE);
        const synapsePublicKey = metadata.identity.publicKey;

        // Call the createUser function from the User model
        const newUserId = await User.createUser(publicKey, handle, display_name, synapsePublicKey);
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

            const scopes = [
                'synapses:read', 'synapses:write',
                'users:read', 'users:write',
                'profiles:read','profiles:write',
                'follow:read','follow:write',
                'posts:read','posts:write',
                'comments:read','comments:write',
                'chats:read', 'chats:write',
                'reactions:read', 'reactions:write',
            ];

            const accessToken  = await mintAccessToken({ userPk: publicKey, scopes });
            const refreshToken = await mintRefreshToken({ userPk: publicKey, scopes });

            // Set httpOnly refresh cookie so the browser will send it on /api/auth/refresh
            res.cookie('refreshToken', refreshToken, {
                httpOnly: true,
                sameSite: 'lax',   // 'strict' is fine if all same-site; use 'none' + secure:true for cross-site
                secure: false,     // true in production behind HTTPS
                path: '/api/auth/refresh',
                maxAge: 7 * 24 * 60 * 60 * 1000,
            });

            return res.status(200).json({
                accessToken,
                tokenType: 'Bearer',
                expiresIn: 600,
            });

        }
    } catch (error) {
        console.error("Error in /verifyCryptoSignature:", error);
    }
};

export const refresh = async (req, res) => {
    try {
        const refreshToken = req.cookies?.refreshToken;
        if (!refreshToken) {
            return res.status(401).json({ error: 'Missing refresh token' });
        }

        // Verify refresh token
        let payload;
        try {
            payload = await verifyRefreshToken(refreshToken);
        } catch (err) {
            return res.status(401).json({ error: 'Invalid or expired refresh token' });
        }

        // Mint new access token
        const accessToken = await mintAccessToken({
            userPk: payload.pubkey,
            scopes: payload.scopes || [],
        });

        // Rotate refresh token (recommended)
        const newRefreshToken = await mintRefreshToken({
            userPk: payload.pubkey,
            scopes: payload.scopes || [],
        });

        res.cookie('refreshToken', newRefreshToken, {
            httpOnly: true,
            sameSite: 'lax',
            secure: false,                      // true in prod HTTPS
            path: '/api/auth/refresh',
            maxAge: 7 * 24 * 60 * 60 * 1000,
        });

        return res.json({ accessToken });
    } catch (err) {
        console.error('Refresh error:', err);
        return res.status(500).json({ error: 'Failed to refresh session' });
    }
};

export const logout = (req, res) => {
    console.log('logout called')
    res.clearCookie('refreshToken', {
        httpOnly: true,
        sameSite: 'lax',
        secure: false,
        path: '/api/auth/refresh',
    });
    return res.status(200).json({ message: 'Logged out' });
};


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
    refresh,
    logout,
    updateAccountSettings
}