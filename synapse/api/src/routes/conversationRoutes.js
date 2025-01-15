import express from 'express';
const router = express.Router();
import conversationController from '../controllers/conversationController.js';

// Define conversationRoutes and link them to corresponding controller functions

router.get('/getConversations', conversationController.getConversations);
router.post('/createConversation', conversationController.createConversation);
router.put('/updateConversationParticipants', conversationController.updateConversationParticipants)

// Export the router so it can be used in server.js
export default router;