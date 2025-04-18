// Import Express library
const express = require('express')

// Import CORS library
const cors = require('cors');

// Import express-session middleware
const session = require('../middlewares/session')

// Import sessionLogger middleware
const sessionLogger = require('../middlewares/sessionLogger');



// Configure and instantiate Express app
const createExpressApp = () => {
    const app = express();

    // Configure CORS
    app.use(cors({
        origin: process.env.CORS_ORIGIN, // client's origin  // TODO update for production
        credentials: true, // Allow credentials (cookies) to be sent
    }));

    // Configure express-session middleware
    app.use(session);

    // Configure sessionLogger middleware
    app.use(sessionLogger);

    return app;
};

module.exports = createExpressApp;