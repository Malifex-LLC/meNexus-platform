// Import orbitdb connection
import meNexus from '../../config/mysql.js';

// Import orbitDB userPublicKeys orbitdb wrapper function
import { storePublicKeyInDB } from '#src/orbitdb/userPublicKeys.js';

// TODO not all functions are written in the return new Promise() format

// Function to create a new user
export const createUser = async (publicKey, handle, displayName) => {
    console.log('createUser called for handle: ', handle);
    console.log('createUser called with publicKey: ', publicKey);

    try {
        // Insert the new user into the Users table
        const userQuery = `
            INSERT INTO Users (handle, display_name, created_at) 
            VALUES (?, ?, NOW())
        `;

        const userParams = [handle, displayName];
        const userResult = await executeQuery(userQuery, userParams);

        const userID = userResult.insertId;
        console.log('User created with ID:', userID);

        await storePublicKeyInDB(userID, publicKey);

        // Create a default profile for the new user
        await createProfile(userID, handle);

        return userID;
    } catch (error) {
        console.error("Error creating user:", error.message);
        throw new Error('Failed to create user');
    }
};

// Function to create a default profile for a user
const createProfile = async (userID, handle) => {
    console.log('createProfile called for user ID:', userID);
    try {
        const profileQuery = `
            INSERT INTO Profiles (user_id, profile_name, profile_bio, profile_location, profile_picture, profile_banner, custom_css, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, NOW())
        `;

        const defaultProfileName = handle;
        const defaultProfileBio = 'Update your bio!';
        const defaultProfileLocation = 'Update your location';
        const defaultProfilePicture = '/uploads/profile_pictures/default_avatar.jpeg';
        const defaultProfileBanner = '/assets/default_profile_banner.jpg';
        const defaultCSS = '';

        const profileParams = [
            userID,
            defaultProfileName,
            defaultProfileBio,
            defaultProfileLocation,
            defaultProfilePicture,
            defaultProfileBanner,
            defaultCSS
        ];

        const result = await executeQuery(profileQuery, profileParams);
        console.log('Default profile created with ID:', result.insertId);

        return result.insertId;
    } catch (err) {
        console.error("Error creating default profile: ", err.message);
        throw new Error('Failed to create default profile');
    }
};

export const getAllUsers = () => {
    return new Promise((resolve, reject) => {
        const query = 'SELECT * FROM Users';
        meNexus.query(query, (err, results) => {
            if (err) {
                console.error('Database error in getAllUsers: ', err);
                return reject(err);
            }
            resolve(results);
        });
    });
};

// Fetch user details by user_id
export const getUserById = (userId) => {
    return new Promise((resolve, reject) => {
        const query = 'SELECT * FROM Users WHERE user_id = ?';
        meNexus.query(query, [userId], (err, results) => {
            if (err) {
                console.error('Database error in getUserById:', err);
                return reject(err);
            }
            resolve(results[0]); // Return the first result (or undefined if none found)
        });
    });
};

// Fetch user by handle
export const getUserByHandle = async (handle) => {
    console.log("getUserByHandle called for handle:", handle);
    try {
        const query = 'SELECT * FROM Users WHERE handle = ?';
        const result = await executeQuery(query, [handle]);
        return result.length > 0 ? result[0] : null; // Return user or null
    } catch (error) {
        console.error("Error in getUserByHandle:", error.message);
        throw new Error('Failed to fetch user by handle');
    }
};

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
    getAllUsers,
    getUserById,
    getUserByHandle,
    getProfile,
    updateUser,
    updateProfile
}