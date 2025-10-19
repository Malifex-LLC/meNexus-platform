// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import orbitdb connection
import meNexus from '../config/mysql.js';
import { createGlobalUser, deleteGlobalUser, getUserByPublicKeyFromDB, getUserByHandleFromDB, getAllUsersFromDB } from "#src/orbitdb/globalUsers.js";

// Import orbitDB userPublicKeys orbitdb wrapper function
import { storePublicKeyInDB } from '#src/orbitdb/userPublicKeys.js';

export const createUser = async (publicKey, handle, displayName, synapsePublicKey) => {
    console.log("Create User called for publicKey:", publicKey);
    try {
        const user = await createGlobalUser(publicKey, handle, displayName, synapsePublicKey);
        return user;
    } catch (error) {
        console.error('Error creating user: ', error);
        throw new Error('Failed to create User called for publicKey');
    }
}

export const deleteUser = async (publicKey) => {
    console.log("Delete User called for publicKey:", publicKey);
    try {
        const user = await deleteGlobalUser(publicKey);
        return user;
    } catch (error) {
        console.error('Error deleting user: ', error);
        throw new Error('Failed to delete User called for publicKey');
    }
}

export const getAllUsers = async () => {
    try {
        const users = await getAllUsersFromDB();
        return users;
    } catch (error) {
        console.error('Failed to get all users:', error);
        throw new Error('Failed to get all users.');
    }
};

export const getUserByPublicKey = async (publicKey) => {
    try {
        const user = await getUserByPublicKeyFromDB(publicKey);
        return user;
    } catch (error) {
        console.error('Failed to get user by publicKey:', error);
        throw new Error('Failed to get user by publicKey');
    }
}

export const getUserByHandle = async (handle) => {
    try {
        const user = await getUserByHandleFromDB(handle);
        return user;
    } catch (error) {
        console.error('Failed to get user by handle:', error);
        throw new Error('Failed to get user by handle');
    }
}

// Generic query execution
const executeQuery = (query, params) => {
    return new Promise((resolve, reject) => {
        meNexus.query(query, params, (error, results) => {
            if (error) {
                console.error(`Database query failed: ${query}`, error.message);
                reject(error);
            } else {
                resolve(results);
            }
        });
    });
};

export const getProfile = (handle) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT
                Profiles.user_id,
                Profiles.profile_name,
                Profiles.profile_bio,
                Profiles.profile_location,
                Profiles.profile_picture,
                Profiles.profile_banner,
                Profiles.custom_css,
                Users.display_name,
                Users.handle
            FROM Profiles
            INNER JOIN Users ON Profiles.user_id = Users.user_id
            WHERE Users.handle = ?;
        `;

        meNexus.query(sql, [handle], (err, results) => {
            if (err) {
                console.error('Error fetching user profile:', err);
                return reject(new Error (err));
            } else if (results.length === 0) {
                return reject(null)
            } else {
                resolve(results[0]);
            }
        });
    });
};

export const updateUser = (userFields, userValues) => {
    return new Promise((resolve, reject) => {
        const userSql = `
            UPDATE Users 
            SET ${userFields.join(', ')} 
            WHERE handle = ?
        `;

        meNexus.query(userSql, userValues, (err, result) => {
            if (err) {
                return reject(err);
            }

            resolve(result);
        });
    });
};

export const updateProfile = (profileFields, profileValues) => {
    return new Promise((resolve, reject) => {
        const profileSql = `
            UPDATE Profiles 
            SET ${profileFields.join(', ')} 
            WHERE user_id = (SELECT user_id FROM Users WHERE handle = ?)
        `;

        meNexus.query(profileSql, profileValues, (err, result) => {
            if (err) {
                return reject(err);
            }

            resolve(result);
        });
    });
};

export default {
    createUser,
    deleteUser,
    getAllUsers,
    getUserByPublicKey,
    getUserByHandle,
    getProfile,
    updateUser,
    updateProfile
}