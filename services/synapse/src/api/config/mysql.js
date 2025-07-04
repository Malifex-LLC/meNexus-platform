// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import mysql from 'mysql2';
import dotenv from 'dotenv';
import path from 'path';
import { fileURLToPath } from 'url';

// Resolve __dirname for ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Explicitly point to your .env file
dotenv.config();

//Create mySQL Connection using data stored in .env file
let meNexus = mysql.createConnection({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE
});

//Connect to mySQL
meNexus.connect((err) => {
    if(err) {
        console.log('Failed to connect to mySQL database:', err);
    } else {
        console.log("Connected to mySQL database!");
    }
});

// Export the connection to use it in other modules
export default meNexus;