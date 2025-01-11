const express = require('express');
const router = express.Router();
const notificationController = require('../controllers/notificationController');

// Define notificationRoutes and link them to corresponding controller functions

router.post('/createNotification', notificationController.createNotification);
router.get('/getNotifications', notificationController.getNotifications)
router.put('/setNotificationAsRead', notificationController.setNotificationAsRead)

// Export the router so it can be used in server.js
module.exports = router;