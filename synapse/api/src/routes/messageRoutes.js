const express = require('express');
const router = express.Router();
const messageController = require('../controllers/messageController');

// Define messageRoutes and link them to corresponding controller functions

router.get('/getMessages', messageController.getMessages);
router.post('/createMessage', messageController.createMessage);
router.put('/setMessagesAsRead', messageController.setMessagesAsRead);

// Export the router so it can be used in server.js
module.exports = router;