// Import Express library
const express = require('express')

// Import CORS library
const cors = require('cors');

// Configure and instantiate Express app
const createExpressApp = () => {
    const app = express();

    // Configure CORS
    app.use(cors({
        origin: process.env.CORS_ORIGIN, // client's origin  // TODO update for production
        credentials: true, // Allow credentials (cookies) to be sent
    }));

    return app;
};

module.exports = createExpressApp;