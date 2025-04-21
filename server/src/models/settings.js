const mysql = require("../config/db");

exports.uploadProfilePicture = (s3Url, user_id) => {
    return new Promise((resolve, reject) => {

        // Update the database with the new profile picture path
        const sql = `
            UPDATE Profiles 
            SET profile_picture = ? 
            WHERE user_id = ?
        `;

        mysql.query(sql, [s3Url, user_id], (err, result) => {
            if (err) {
                console.error('Error updating profile picture:', err.message);
                return reject(err);
            }

            console.log('Database Update Result:', result);
            return resolve({ message: 'Profile picture uploaded successfully', profile_picture: s3Url });
        });
    });
}