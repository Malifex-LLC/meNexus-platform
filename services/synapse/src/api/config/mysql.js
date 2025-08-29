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

// Create mySQL Pool using data stored in .env file
let meNexus = mysql.createPool({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE,
    waitForConnections: true,
    connectionLimit: 10, // adjust based on concurrency needs, not sure currently
    queueLimit: 0,
    enableKeepAlive: true,
    keepAliveInitialDelay: 0
});

//Connect to mySQL
meNexus.getConnection((err, connection) => {
    if(err) {
        console.log('Failed to connect to mySQL database:', err);
    } else {
        console.log("Connected to mySQL database!");
        connection.release();
    }
});

// Heartbeat to prevent connection timeout
setInterval(() => {
    meNexus.query('SELECT 1', (err) => {
        if (err) console.error('DB heartbeat failed:', err.message || err);
    });
}, 5 * 60 * 1000);

// Export the connection to use it in other modules
export default meNexus;