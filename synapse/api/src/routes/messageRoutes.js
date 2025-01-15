import express from 'express';
const router = express.Router();
import messageController from '../controllers/messageController.js';

// Define messageRoutes and link them to corresponding controller functions

router.get('/getMessages', messageController.getMessages);
router.post('/createMessage', messageController.createMessage);
router.put('/setMessagesAsRead', messageController.setMessagesAsRead);

// Export the router so it can be used in server.js
export default router;