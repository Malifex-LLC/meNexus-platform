import express from 'express';
const router = express.Router();
import userController from '../controllers/userController.js';

// Define userRoutes and link them to corresponding controller functions

router.get('/getAllUsers', userController.getAllUsers);
router.get('/getSessionUser', userController.getSessionUser);
router.get('/getUserById/:user_id', userController.getUserById)
router.get('/getProfile/:handle', userController.getProfile);
router.put('/updateProfileSettings/:handle', userController.updateProfileSettings);

// Export the router so it can be used in server.js
export default router;