// Import Settings model
const Settings = require('../models/settings');
const { uploadToS3 } = require('../utils/uploadToS3');
const fs = require("fs");

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

    const localFilePath = req.file.path;
    const s3Key = `profile_pictures/${Date.now()}_${req.file.originalname}`;
    console.log('Profile Picture localFilePath: ', localFilePath);
    console.log('Profile Picture s3Key: ', s3Key);

    try {
        const s3Url = await uploadToS3(localFilePath, s3Key);
        fs.unlinkSync(localFilePath); // delete local file

        const result = await Settings.uploadProfilePicture(s3Url, user_id);
        return res.status(200).json({ profile_picture: s3Url, message: result.message });
    } catch (err) {
        console.error('Upload error:', err);
        return res.status(500).json({ error: 'Failed to upload image' });
    }
}