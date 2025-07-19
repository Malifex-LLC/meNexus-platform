// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

// Import Express library
import express  from 'express'

// Import CORS library
import cors from 'cors'

import dotenv from 'dotenv';
dotenv.config();

// Configure and instantiate Express app
export function createExpressApp () {
    const app = express();

    // Configure CORS
    app.use(cors({
        origin: process.env.CORS_ORIGIN, // web's origin
        credentials: true, // Allow credentials (cookies) to be sent
    }));

    app.use(express.json());

    return app;
};

