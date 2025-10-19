// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import express  from 'express'
import cors from 'cors'
import cookieParser from 'cookie-parser';
import dotenv from 'dotenv';
dotenv.config();

// Configure and instantiate Express app
export function createExpressApp () {
    const app = express();

    app.use(cookieParser());

    // Configure CORS
    app.use(cors({
        origin: process.env.CORS_ORIGIN, // web's origin
        credentials: true, // Allow credentials (cookies) to be sent
    }));

    app.use(express.json());

    return app;
};

