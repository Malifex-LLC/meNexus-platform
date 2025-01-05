const express = require('express');
const router = express.Router();
const searchController = require('../controllers/searchController');

// Define the searchRoutes and link it to the controller function

router.get('/search', searchController.search);

// Export the router so it can be used in app.js
module.exports = router;