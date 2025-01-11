const multer =  require("multer");
const path = require('path')

// Configure Multer storage
const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        // Determine the upload folder based on the uploadType
        const uploadType = req.body.uploadType || req.query.uploadType;

        let folderPath;
        if (uploadType === 'profilePicture') {
            folderPath = path.join(__dirname, '../uploads/profile_pictures');
        } else if (uploadType === 'postMedia') {
            folderPath = path.join(__dirname, '../uploads/post_media');
        } else {
            folderPath = path.join(__dirname, '../uploads/others'); // Fallback folder
        }

        cb(null, folderPath); // Set the folder path
    },
    filename: (req, file, cb) => {
        // Create a unique filename with a timestamp
        cb(null, `${Date.now()}-${file.originalname}`);
    },
});

// Initialize Multer with the storage configuration
const upload = multer({
    storage: storage,
    limits: { fileSize: 5 * 1024 * 1024 }, // Limit file size to 5MB
    fileFilter: (req, file, cb) => {
        // Allow only certain file types
        const fileTypes = /jpeg|jpg|png/;
        const extName = fileTypes.test(file.originalname.toLowerCase());
        const mimeType = fileTypes.test(file.mimetype);

        if (extName && mimeType) {
            return cb(null, true);
        } else {
            cb(new Error('Only images (JPEG, JPG, PNG) are allowed!'));
        }
    },
});

module.exports = upload;