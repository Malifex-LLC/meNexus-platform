// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Settings model
import Settings from '../models/settings.js';

export const uploadProfilePicture = async (req, res) => {
    console.log('uploadType: ', req.body.uploadType)
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    // Validate session user
    const { publicKey } = req.session.user // Extract user ID from the session

    // Validate uploaded file
    if (!req.file) {
        return res.status(400).json({ error: 'No file uploaded' });
    }

    const profilePicturePath = `/uploads/profile_pictures/${req.file.filename}`;
    console.log('Profile Picture Path:', profilePicturePath);

    const result = await Settings.uploadProfilePicture(profilePicturePath, publicKey);
    return res.status(200).json(result);
}

export default {
    uploadProfilePicture
};