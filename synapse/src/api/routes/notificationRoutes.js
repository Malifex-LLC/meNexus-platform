import express from 'express';
const router = express.Router();
import notificationController from '../controllers/notificationController.js'

// Define notificationRoutes and link them to corresponding controller functions

router.post('/createNotification', notificationController.createNotification);
router.get('/getNotifications', notificationController.getNotifications)
router.put('/setNotificationAsRead', notificationController.setNotificationAsRead)

// Export the router so it can be used in server.js
export default router;