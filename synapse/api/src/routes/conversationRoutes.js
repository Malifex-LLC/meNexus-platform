const express = require('express');
const router = express.Router();
const conversationController = require('../controllers/conversationController');

// Define conversationRoutes and link them to corresponding controller functions

router.get('/getConversations', conversationController.getConversations);
router.post('/createConversation', conversationController.createConversation);
router.put('/updateConversationParticipants', conversationController.updateConversationParticipants)

// Export the router so it can be used in server.js
module.exports = router;