const session = require('express-session');
require('dotenv').config();

const sessionMiddleware = session({
    secret: process.env.SESSION_SECRET,
    resave: false,
    saveUninitialized: false,
    cookie: {
        httpOnly: true,
        secure: process.env.NODE_ENV === 'production', // Use secure cookies in prod
        sameSite: 'Lax', // Safe for most use cases
        maxAge: 1000 * 60 * 60 * 24 * 7 // 1 week session duration
    }
});

module.exports = sessionMiddleware;
