// Import User model
import User from '../models/user.js'
import {getGlobalUsersDB, getUserByPublicKeyFromDB} from "#src/orbitdb/globalUsers.js";


export const getAllUsers = async (req, res) => {
    const users = await User.getAllUsers();
    res.status(200).json(users);
}

export const getSessionUser = async (req, res) => {

    console.log('getSessionUser called');
    console.log('Session ID:', req.sessionID);
    console.log('Session Data:', req.session);

    if (req.session && req.session.user) {
        const { publicKey, handle, displayName } = req.session.user;

        // Ensure the session data contains all necessary fields
        if (!publicKey || !handle || !displayName) {
            console.error('Incomplete session data');
            return res.status(400).json({ error: 'Incomplete session data' });
        }

        // Send the user data stored in the session
        console.log('User found in session:', req.session.user);
        return res.json({
            publicKey,
            handle,
            displayName
        });
    } else {
        console.log('User not authenticated');
        return res.status(401).json({ error: 'User not authenticated' });
    }
}

export const getUserByPublicKey = async (req, res) => {
    const publicKey = req.query.publicKey;
    try {
        const user = await User.getUserByPublicKey(publicKey);
        res.status(200).json(user);
    } catch (error) {
        console.error('Error getting user by publicKey: ', error);
        return res.status(500).json({ error: 'Error getting user by publicKey: ', publicKey });
    }
}

export const getUserByHandle = async (req, res) => {
    const handle = req.query.handle;
    try {
        const user = await User.getUserByHandle(handle);
        res.status(200).json(user);
    } catch (error) {
        console.error('Error getting user by handle: ', error);
        return res.status(500).json({ error: 'Error getting user by handle: ', handle });
    }
}

export const getProfile = async (req, res) => {
    console.log('getProfile called');
    const handle = req.params.handle;
    try {
        const profile = await User.getProfile(handle);
        res.status(200).json(profile)
    } catch (error) {
        console.error(error);
        return res.status(500).json({ error: 'Error getting profile info' });
    }

}

export const updateProfileSettings = async (req, res) => {
    const { publicKey } = req.params;
    const updatedFields = req.body;
    console.log('updateProfileSettings called for publicKey: ', publicKey);
    console.log('updateProfileSettings called for fields: ', updatedFields);
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    console.log('updateProfileSettings called for updatedUser: ', updatedUser);

    if (updatedUser) {
        if (updatedFields.displayName) {
            updatedUser.displayName = updatedFields.displayName;
            await db.put(updatedUser);
            req.session.user.displayName = updatedFields.displayName;
        }
        if (updatedFields.handle) {
            updatedUser.handle = updatedFields.handle;
            await db.put(updatedUser);
            req.session.user.handle = updatedFields.handle;
        }
        if (updatedFields.profileName) {
            updatedUser.profileName = updatedFields.profileName;
            await db.put(updatedUser);
        }
        if (updatedFields.bio) {
            updatedUser.bio = updatedFields.bio;
            await db.put(updatedUser);
        }
        if (updatedFields.location) {
            updatedUser.location = updatedFields.location;
            await db.put(updatedUser);
        }
    }
    req.session.save();
    console.log('Updated session user data: ', req.session.user);
}

export default {
    getAllUsers,
    getSessionUser,
    getUserByPublicKey,
    getUserByHandle,
    getProfile,
    updateProfileSettings,
}