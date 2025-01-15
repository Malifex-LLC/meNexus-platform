// Import User model
import User from '../models/user.js'

export const getSessionUser = async (req, res) => {

    console.log('getSessionUser called');
    console.log('Session ID:', req.sessionID);
    console.log('Session Data:', req.session);

    if (req.session && req.session.user) {
        const { user_id, handle, display_name } = req.session.user;

        // Ensure the session data contains all necessary fields
        if (!user_id || !handle || !display_name) {
            console.error('Incomplete session data');
            return res.status(400).json({ error: 'Incomplete session data' });
        }

        // Send the user data stored in the session
        console.log('User found in session:', req.session.user);
        return res.json({
            user_id,
            handle,
            display_name
        });
    } else {
        console.log('User not authenticated');
        return res.status(401).json({ error: 'User not authenticated' });
    }
}

export const getUserById = async (req, res) => {
    console.log('/getUser called');
    const { user_id } = req.query;

    if(!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const user = await User.getUserById(user_id);
    return res.status(200).json(user);
}

export const getProfile = async (req, res) => {
    console.log('getProfile called');
    const handle = req.params.handle;

    const profile = await User.getProfile(handle);
    res.status(200).json(profile)
}

export const updateProfileSettings = async (req, res) => {
    const { handle } = req.params; // Get the current user handle from the request
    const updatedFields = req.body; // Get the fields to update from the request body

    console.log('updateProfileSettings called for handle: ', handle);

    if (!handle || Object.keys(updatedFields).length === 0) {
        return res.status(400).json({ error: "Invalid request data" });
    }

    // Separate fields for Users and Profiles tables
    const userFields = [];
    const userValues = [];
    const profileFields = [];
    const profileValues = [];

    // Classify updates into Users and Profiles fields
    for (const [key, value] of Object.entries(updatedFields)) {
        if (key === 'handle' || key === 'display_name') {
            userFields.push(`${key} = ?`);
            userValues.push(value);
        } else if (
            key === 'profile_name' ||
            key === 'profile_bio' ||
            key === 'profile_location'
        ) {
            profileFields.push(`${key} = ?`);
            profileValues.push(value);
        } else {
            console.warn(`Unknown field: ${key} - Ignoring`);
        }
    }

    // Push handle for SQL queries
    userValues.push(handle);
    profileValues.push(handle);

    // Tracks updates to handle, used for subsequent queries to Profiles table if needed
    let newHandle = handle;

    try {
        if (userFields.length > 0) {
            const results = await User.updateUser(userFields, userValues);

            if (results.affectedRows === 0) {
                return res.status(404).json({error: 'User not found'});
            }
            // Update newHandle if handle was updated
            if (updatedFields.handle && updatedFields.handle !== handle) {
                newHandle = updatedFields.handle;
                req.session.user.handle = newHandle; // Update session data
            }

            if (updatedFields.display_name) {
                req.session.user.display_name = updatedFields.display_name;
            }
        }

        if (profileFields.length > 0) {
            const results = await User.updateProfile(profileFields, profileValues);

            if (results.affectedRows === 0) {
                return res.status(404).json({error: 'Profile not found'});
            }
        }

        return res.status(200).json({message: "Profile updated successfully"});
    } catch (error) {
        console.error(error);
        return res.status(500).json({error: 'Error updating Profile settings'});
    }

}

export default {
    getSessionUser,
    getUserById,
    getProfile,
    updateProfileSettings,
}