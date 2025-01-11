// Imports
import dotenv from 'dotenv';
dotenv.config({ path: '../config/.env' });
import { createExpressApp } from '../config/express.js';
import { createWebSocketServer, clients } from '../config/websocket.js'
import express from 'express'
import sessionMiddleware from './middlewares/session.js'
import sessionLogger from './middlewares/sessionLogger.js'

// Create Express server
const server = createExpressApp();


// Assigning port number for the express api
const port = process.env.EXPRESS_PORT;

// Express api listening on port number specified
const httpServer = server.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
});

// Instantiate WebSocket Server
const wss = createWebSocketServer(httpServer);

// Export Express server, WebSocket api, and WebSocket clients for use externally
export {
    server,
    wss,
    clients,
};

// Initialize session middleware
server.use(sessionMiddleware);

// Use sessionLogger middleware
//server.use(sessionLogger);

// Serve static files from /uploads directory
server.use('/uploads', express.static('../uploads'));

// Import orbitdb-service functions
import { initializeOrbitDB, getDatabase, closeDatabase, closeOrbitDB } from '../../config/orbitdb-service.js'

// Initialize OrbitDB when the api starts
(async () => {
    await initializeOrbitDB();
    console.log('OrbitDB service running');
})();

// Ensure OrbitDB stops gracefully when the api shuts down
process.on('SIGINT', async () => {
    await closeOrbitDB();
    process.exit();
});

///////////////////////////////////////////API Routes///////////////////////////////////////////

import authRoutes from './routes/authRoutes.js'
server.use('/api/auth', authRoutes);

import userRoutes from './routes/userRoutes.js'
server.use('/api/user', userRoutes);

import followerRoutes from './routes/followerRoutes.js'
server.use('/api/follow', followerRoutes);

import postRoutes from './routes/postRoutes.js'
server.use('/api/post', postRoutes );

import commentRoutes from './routes/commentRoutes.js'
server.use('/api/comment', commentRoutes);

import conversationRoutes from './routes/conversationRoutes.js'
server.use('/api/conversation', conversationRoutes);

import messageRoutes from './routes/messageRoutes.js'
server.use('/api/message', messageRoutes);

import notificationRoutes from './routes/notificationRoutes.js'
server.use('/api/notification', notificationRoutes);

import searchRoutes from './routes/searchRoutes.js'
server.use('/api/search', searchRoutes);

import settingsRoutes from './routes/settingsRoutes.js'
server.use('/api/settings', settingsRoutes);

import synapseRoutes from './routes/synapseRoutes.js'
server.use('/synapse', synapseRoutes);