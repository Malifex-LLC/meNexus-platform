const express = require('express');
const router = express.Router();
const authController = require('../controllers/authController')

// Define authRoutes and link them to corresponding controller functions

router.post('/createUser', authController.createUser);
router.get('/getUserIdByPublicKey', authController.getUserIdByPublicKey);
router.get('/getAllPublicKeys', authController.getAllPublicKeys);
router.get('/getCryptoChallenge', authController.getCryptoChallenge);
router.post('/verifyCryptoSignature', authController.verifyCryptoSignature);
router.post('/login', authController.login);
router.post('/logout', authController.logout);
router.put('/updateAccountSettings', authController.updateAccountSettings);

// Export the router so it can be used in app.js
module.exports = router;