// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import User model
import User from '../models/user.js'
import {getGlobalUsersDB, getUserByPublicKeyFromDB} from "#src/orbitdb/globalUsers.js";


export const getAllUsers = async (req, res) => {
    const users = await User.getAllUsers();
    res.status(200).json(users);
}

export const getSessionUser = async (req, res) => {
    if (!req.user?.publicKey) return res.status(401).json({ error: 'Not authenticated' });

    const user = await User.getUserByPublicKey(req.user.publicKey);
    if (!user) return res.status(404).json({ error: 'User not found' });

    return res.json({user});
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
    if (!req.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const publicKey = req.user?.publicKey;
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
        }
        if (updatedFields.handle) {
            updatedUser.handle = updatedFields.handle;
            await db.put(updatedUser);
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

    res.status(200).json({message: 'Updated profile setting successfully'});
}

export default {
    getAllUsers,
    getSessionUser,
    getUserByPublicKey,
    getUserByHandle,
    getProfile,
    updateProfileSettings,
}