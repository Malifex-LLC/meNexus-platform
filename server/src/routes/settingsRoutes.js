const express = require('express');
const router = express.Router();
const settingsController = require('../controllers/settingsController');

// Import Multer upload config
const upload = require('../config/multer');

// Define settingsRoutes and link them to corresponding controller functions

router.post('/uploadProfilePicture', upload.single('profile_picture'), settingsController.uploadProfilePicture);

// Export the router so it can be used in app.js
module.exports = router;