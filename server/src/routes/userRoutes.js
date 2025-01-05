const express = require('express');
const router = express.Router();
const userController = require('../controllers/userController');

// Define userRoutes and link them to corresponding controller functions

router.get('/getSessionUser', userController.getSessionUser);
router.get('/getUserById/:user_id', userController.getUserById)
router.get('/getProfile/:handle', userController.getProfile);
router.put('/updateProfileSettings/:handle', userController.updateProfileSettings);

// Export the router so it can be used in app.js
module.exports = router;