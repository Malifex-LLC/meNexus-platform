// Import the Passport.js config
const passport = require('../config/passport');

// Import bcrypt
bcrypt = require('bcrypt');

// Import the Auth model
const Auth = require('../models/auth')

// Import the User model
const User = require("../models/user");

// Account registration logic
exports.createUser = async (req, res) => {
    try {
        const { email, password, handle, display_name } = req.body;
        console.log("Received data:", { email, password, handle, display_name });

        // Validate required fields
        if (!email || !password || !handle || !display_name) {
            console.log("Missing required fields.");
            return res.status(400).json({ error: 'All fields are required' });
        }

        // Check if the email is already used by another user
        const existingUserByEmail = await Auth.getUserByEmail(email);
        console.log("Existing user by email:", existingUserByEmail);

        if (existingUserByEmail) {
            console.log("Email is already taken.");
            return res.status(400).json({ error: 'Email is already taken' });
        }

        // Check if the handle is already used by another user
        const existingUserByHandle = await User.getUserByHandle(handle);
        console.log("Existing user by handle:", existingUserByHandle);

        if (existingUserByHandle) {
            console.log("Handle is already taken.");
            return res.status(400).json({ error: 'Handle is already taken' });
        }

        // Call the createUser function from the User model
        const newUserId = await User.createUser(email, password, handle, display_name);
        console.log("New user created with ID:", newUserId);

        // Return a success response
        return res.status(200).json({ message: 'User created successfully', user_id: newUserId });
    } catch (error) {
        console.error("Error in /createUser:", error);
        return res.status(500).json({ error: 'Failed to create user'});
    }
}

// Login logic
exports.login = (req, res, next) => {
    console.log('/login called');
    passport.authenticate('local', (err, user, info) => {
        if (err) {
            // Handle error
            console.error('Error during login authentication:', err);
            return res.status(500).json({ error: 'An error occurred during login' });
        }
        if (!user) {
            // Authentication failed
            return res.status(401).json({ error: info ? info.message : 'Incorrect email or password' });
        }
        // Log in the user
        req.login(user, (err) => {
            if (err) {
                // Handle error
                console.error('Error during session login:', err);
                return res.status(500).json({ error: 'Failed to log in' });
            }

            // Attach session data
            req.session.user = {
                user_id: user.user_id,
                handle: user.handle,
                display_name: user.display_name,
            };

            console.log('Session Data:', req.session.user);

            return res.status(200).json({
                message: 'Login successful',
                handle: user.handle,
                display_name: user.display_name,
            });
        });
    })(req, res, next); // Call passport.authenticate middleware
};

// Logout logic
exports.logout = (req, res) => {
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

exports.updateAccountSettings = async (req, res) => {
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    const { user_id } = req.session.user;
    const updatedFields = req.body;
    console.log('updateAccountSettings called for user_id: ', user_id);

    if (!user_id || Object.keys(updatedFields).length === 0) {
        return res.status(400).json({ error: "Invalid request data" });
    }

    // Separate fields from values
    const authFields = [];
    const authValues = [];

    for ( [key, value] of Object.entries(updatedFields)) {
        if (key === 'password') {
            key = 'hashed_password';
            hashed_password = await bcrypt.hash(value, 10);
            authFields.push(`${key} = ?`);
            authValues.push(hashed_password);
        } else if (key === 'email') {
            authFields.push(`${key} = ?`);
            authValues.push(value);
        } else {
            console.warn(`Unknown field: ${key} - Ignoring`);
        }

    }
    authValues.push(user_id) // Add current user_id

    if (authFields.length > 0) {
        try {
            const results = await Auth.updateAccountSettings(authFields, authValues);

            if (results.affectedRows === 0) {
                return res.status(404).json({ error: 'Authentication not found' });
            }

            return res.status(200).json({ message: 'Account settings updated successfully', results });
        } catch (error) {
            console.error('Error updating account settings:', error);
            return res.status(500).json({ error: 'Failed to update account settings' });
        }
    } else {
        return res.status(400).json({ error: 'No valid fields to update' });
    }
}