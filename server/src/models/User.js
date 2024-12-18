const mysql = require('mysql2');
const bcrypt = require('bcrypt');

// Database connection
let meNexus = mysql.createConnection({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE
});

meNexus.connect();

// Function to create a new user
const createUser = async (email, password, handle, displayName) => {
    console.log('createUser called for handle: ', handle);

    try {
        // Hash the password
        const hashedPassword = await bcrypt.hash(password, 10);

        // Insert the new user into the Users table
        const userQuery = 'INSERT INTO Users (handle, display_name, created_at) VALUES (?, ?, NOW())';
        const userParams = [handle, displayName];
        const userResult = await executeQuery(userQuery, userParams);

        const userID = userResult.insertId;
        console.log('User created with ID:', userID);

        // Insert the authentication data into the Authentication table
        const authQuery = 'INSERT INTO Authentication (user_id, email, hashed_password, auth_provider, created_at) VALUES (?, ?, ?, "local", NOW())';
        const authParams = [userID, email, hashedPassword];
        await executeQuery(authQuery, authParams);
        console.log('Authentication record created for user:', userID);

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
        const defaultProfilePicture = '/assets/default_profile_picture.jpg';
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

// Fetch user by email (Authentication table)
const getUserByEmail = async (email) => {
    console.log("getUserByEmail called for email:", email);
    try {
        const query = `
            SELECT u.*, a.email, a.auth_provider
            FROM Users u
            INNER JOIN Authentication a ON u.user_id = a.user_id
            WHERE a.email = ?
        `;
        const result = await executeQuery(query, [email]);

        return result.length > 0 ? result[0] : null; // Return user or null
    } catch (error) {
        console.error("Error in getUserByEmail:", error.message);
        throw new Error('Failed to fetch user by email');
    }
};

// Fetch user by handle
const getUserByHandle = async (handle) => {
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

// Fetch user by ID
const getUserById = async (userID) => {
    console.log("getUserByID called for ID:", userID);
    try {
        const query = 'SELECT * FROM Users WHERE user_id = ?';
        const result = await executeQuery(query, [userID]);
        return result.length > 0 ? result[0] : null; // Return user or null
    } catch (error) {
        console.error("Error in getUserById:", error.message);
        throw new Error('Failed to fetch user by ID');
    }
};

// Fetch Authentication by email
const getAuthByEmail = async (email) => {
    console.log("getAuthByEmail called for email:", email);
    try {
        const query = `
            SELECT * 
            FROM Authentication 
            WHERE email = ?
        `;
        const result = await executeQuery(query, [email]);
        return result.length > 0 ? result[0] : null;
    } catch (error) {
        console.error("Error in getAuthByEmail:", error.message);
        throw new Error('Failed to fetch authentication data by email');
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

module.exports = {
    createUser,
    createProfile,
    getUserByEmail,
    getUserByHandle,
    getUserById,
    getAuthByEmail
};
