// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import dotenv from 'dotenv';
dotenv.config();

import express from 'express';
import { createExpressApp }     from './config/express.js';
import { createWebSocketServer } from './config/websocket.js';
import sessionMiddleware         from './middlewares/session.js';
// import sessionLogger           from './middlewares/sessionLogger.js'

import { initializeOrbitDB, closeOrbitDB } from '#config/orbitdb-service.js';
import { getGlobalUsersDB } from '#src/orbitdb/globalUsers.js'

// Route modules
import activityRoutes     from "#api/routes/activityRoutes.js";
import authRoutes         from './routes/authRoutes.js';
import userRoutes         from './routes/userRoutes.js';
import followerRoutes     from './routes/followerRoutes.js';
import postRoutes         from './routes/postRoutes.js';
import commentRoutes      from './routes/commentRoutes.js';
import chatRoutes         from "#api/routes/chatRoutes.js";
import reactionRoutes     from './routes/reactionRoutes.js';
import conversationRoutes from './routes/conversationRoutes.js';
import messageRoutes      from './routes/messageRoutes.js';
import notificationRoutes from './routes/notificationRoutes.js';
import searchRoutes       from './routes/searchRoutes.js';
import settingsRoutes     from './routes/settingsRoutes.js';
import synapseRoutes      from './routes/synapseRoutes.js';
import remoteRoutes       from './routes/remoteRoutes.js';

import path from 'path';
import { fileURLToPath } from 'url';

/* ────────────────────────────────────────────────
   1.  The starter function
   ──────────────────────────────────────────────── */
export async function startApi ({ port = process.env.EXPRESS_PORT } = {}) {

    /* ---- Express & HTTP -------------------------------------------------- */
    const app = createExpressApp();
    app.use(sessionMiddleware);
    // app.use(sessionLogger);

    // Static uploads


    const __filename = fileURLToPath(import.meta.url);
    const __dirname = path.dirname(__filename);

// This points to meNexus/synapse/uploads
    const uploadsPath = path.join(__dirname, '../../uploads');
    console.log('Express serving uploads from: ', uploadsPath);

    app.use('/uploads', express.static(uploadsPath));

    /* ---- REST routes ----------------------------------------------------- */
    app.use('/api/activity',     activityRoutes);
    app.use('/api/auth',         authRoutes);
    app.use('/api/user',         userRoutes);
    app.use('/api/follow',       followerRoutes);
    app.use('/api/post',         postRoutes);
    app.use('/api/comment',      commentRoutes);
    app.use('/api/chat',         chatRoutes);
    app.use('/api/reaction',     reactionRoutes);
    app.use('/api/conversation', conversationRoutes);
    app.use('/api/message',      messageRoutes);
    app.use('/api/notification', notificationRoutes);
    app.use('/api/search',       searchRoutes);
    app.use('/api/settings',     settingsRoutes);
    app.use('/api/synapse',      synapseRoutes);
    app.use('/remote',           remoteRoutes);

    /* ---- Start listening ------------------------------------------------- */
    const httpServer = await new Promise(resolve => {
        const server = app.listen(port, () => {
            console.log(`API listening on port ${port}`);
            resolve(server);               // resolve after it’s ready
        });
    });

    /* ---- WebSockets ------------------------------------------------------ */
    const wss = createWebSocketServer(httpServer);

    /* ---- OrbitDB --------------------------------------------------------- */
    await initializeOrbitDB();
    console.log('OrbitDB service running');
    console.log('Getting globalUsersDB');
    //await getPublicKeysDB();
    await getGlobalUsersDB();

    process.once('SIGINT', async () => {
        await closeOrbitDB();
        httpServer.close(() => process.exit());
    });

    /* ---- Return whatever callers need ----------------------------------- */
    return { app, httpServer, wss };
}

/* ────────────────────────────────────────────────
   2.  Allow “node server.js” for local dev
   ──────────────────────────────────────────────── */
if (import.meta.url === `file://${process.argv[1]}`) {
    startApi().catch(err => {
        console.error('Failed to start API:', err);
        process.exit(1);
    });
}
