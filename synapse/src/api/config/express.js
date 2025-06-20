// Import Express library
import express  from 'express'

// Import CORS library
import cors from 'cors'

// Configure and instantiate Express app
export function createExpressApp () {
    const app = express();

    // Configure CORS
    app.use(cors({
        origin: 'http://localhost:5173', // web's origin
        credentials: true, // Allow credentials (cookies) to be sent
    }));

    app.use(express.json());

    return app;
};

