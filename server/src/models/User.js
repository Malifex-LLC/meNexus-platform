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
    try {
        const hashedPassword = await bcrypt.hash(password, 10);

        const query = 'INSERT INTO Users (email, password, handle, username) VALUES (?, ?, ?, ?)';
        const params = [email, hashedPassword, handle, username];

        const result = await executeQuery(query, params);

        console.log('User created with ID:', result.insertId);
        return result.insertId;
    } catch (error) {
        console.error("Error creating user:", error.message);
        throw new Error('Failed to create user');
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
    getUserByEmail,
    getUserByHandle,
    getUserById
};