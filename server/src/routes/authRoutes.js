const express = require('express');
const router = express.Router();
const authController = require('../controllers/authController')

// Define authRoutes and link them to corresponding controller functions

router.post('/createUser', authController.createUser)
router.post('/login', authController.login);
router.post('/logout', authController.logout);
router.put('/updateAccountSettings', authController.updateAccountSettings)

// Export the router so it can be used in app.js
module.exports = router;