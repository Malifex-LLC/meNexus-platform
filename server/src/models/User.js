const mysql = require('mysql2');
const bcrypt = require('bcrypt');

let meNexus = mysql.createConnection({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE
});

meNexus.connect();

const createUser = async (email, password, handle, username) => {
    console.log('createUser called for handle: ', handle);
    try {
        // Hash the password
        const hashedPassword = await bcrypt.hash(password, 10);

        // Insert the new user into the database
        const query = 'INSERT INTO Users (email, password, handle, username) VALUES (?, ?, ?, ?)';
        const params = [email, hashedPassword, handle, username];

        const result = await executeQuery(query, params);
        const userID = result.insertId;
        console.log('User created with ID:', userID);

        // Create a default profile for the new user
        await createProfile(userID, handle);

        return result.insertId;
    } catch (error) {
        console.error("Error creating user:", error.message);
        throw new Error('Failed to create user');
    }
};

const createProfile = async (userID, handle) => {
    console.log('createProfile called for handle: ', handle);
    try {
        const query = 'INSERT INTO Profiles (user_id, name, bio, location, created_at) VALUES (?, ?, ?, ?, NOW())';
        const defaultName = handle;
        const defaultBio = 'Update your bio!';
        const defaultLocation = 'Update your location';

        const result = await executeQuery(query, [userID, defaultName, defaultBio, defaultLocation]);
        console.log('Default profile created with ID: ', result.insertId);
        return result.insertId;
    } catch (err) {
        console.error("Error creating default profile: ", err.message);
        throw new Error('Failed to create default profile');
    }
};

const getUserByEmail = async (email) => {
    console.log("getUserByEmail called for email:", email);
    try {
        const query = 'SELECT * FROM Users WHERE email = ?';
        const result = await executeQuery(query, [email]);

        return result.length > 0 ? result[0] : null; // Return user or null
    } catch (error) {
        console.error("Error in getUserByEmail:", error.message);
        throw new Error('Failed to fetch user by email');
    }
};

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

const getUserById = async (id) => {
    console.log("getUserByID called for ID:", id);
    try {
        const query = 'SELECT * FROM Users WHERE user_id = ?';
        const result = await executeQuery(query, [id]);
        if (result.length === 0) {
            return null; // User not found
        }
        return result[0]; // Return the first user object from the result
    } catch (error) {
        throw new Error(error);
    }
};

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
    getUserById
};