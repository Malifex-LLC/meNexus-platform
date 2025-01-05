// Import Settings model
const Settings = require('../models/Settings');

exports.uploadProfilePicture = async (req, res) => {
    console.log('uploadType: ', req.body.uploadType)
    if (!req.session || !req.session.user) {
        console.log("User not authenticated or session missing");
        return res.status(401).json({ error: "User not authenticated" });
    }

    // Validate session user
    const { user_id } = req.session.user // Extract user ID from the session

    // Validate uploaded file
    if (!req.file) {
        return res.status(400).json({ error: 'No file uploaded' });
    }

    const profilePicturePath = `/uploads/profile_pictures/${req.file.filename}`;
    console.log('Profile Picture Path:', profilePicturePath);

    const result = await Settings.uploadProfilePicture(profilePicturePath, user_id);
    return res.status(200).json(result);
}