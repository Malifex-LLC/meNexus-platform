const express = require('express');
const router = express.Router();
const synapseController = require('../controllers/synapseController.js');

// Define synapseRoutes and link them to corresponding controller functions

router.get('/getSynapseUserPosts', synapseController.getSynapseUserPosts);

// Export the router so it can be used in server.js
module.exports = router;