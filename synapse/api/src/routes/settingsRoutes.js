import express from 'express';
const router = express.Router();
import settingsController from '../controllers/settingsController.js'

// Import Multer upload config
import {upload}  from '../../config/multer.js'

// Define settingsRoutes and link them to corresponding controller functions

router.post('/uploadProfilePicture', upload.single('profile_picture'), settingsController.uploadProfilePicture);

// Export the router so it can be used in server.js
export default router