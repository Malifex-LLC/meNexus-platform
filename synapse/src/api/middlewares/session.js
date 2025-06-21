import session from 'express-session';
import dotenv from 'dotenv'
dotenv.config();

const sessionMiddleware = session({
    // TODO replace with a secure, random session secret
    secret: 'your-secret-key', // Replace with a secure key
    resave: false,
    saveUninitialized: false,
    cookie: {
        httpOnly: true,
        secure: false, // Set to true only if using HTTPS
        sameSite: 'Lax', // Allow cross-origin requests
    }
})

export default sessionMiddleware;