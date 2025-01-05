// Imports
const createExpressApp = require('./config/express');
const { createWebSocketServer, clients } = require('./config/websocket');
const express = require('express')
const sessionMiddleware = require('./middlewares/session')
const sessionLogger = require('./middlewares/sessionLogger');

// Create Express app
const app = createExpressApp();
app.use(express.json());

// Assigning port number for the express server
const port = process.env.EXPRESS_PORT;

// Express server listening on port number specified
const httpServer = app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
});

// Instantiate WebSocket Server
const wss = createWebSocketServer(httpServer);

// Export Express app, WebSocket server, and WebSocket clients for use externally
module.exports = {
    app,
    wss,
    clients,
};

// Initialize session middleware
app.use(sessionMiddleware);

// Use sessionLogger middleware
//app.use(sessionLogger);

// Initialize passport middleware
passport = require('../src/config/passport')
app.use(passport.initialize());
app.use(passport.session());

// Serve static files from /uploads directory
app.use('/uploads', express.static('uploads'));

///////////////////////////////////////////API Routes///////////////////////////////////////////

const authRoutes = require('../src/routes/authRoutes');
app.use('/api/auth', authRoutes);

const userRoutes = require('../src/routes/userRoutes');
app.use('/api/user', userRoutes);

const followerRoutes = require('./routes/followerRoutes');
app.use('/api/follow', followerRoutes);

const postRoutes = require('../src/routes/postRoutes');
app.use('/api/post', postRoutes );

const commentRoutes = require('../src/routes/commentRoutes');
app.use('/api/comment', commentRoutes);

const conversationRoutes = require('../src/routes/conversationRoutes');
app.use('/api/conversation', conversationRoutes);

const messageRoutes = require('../src/routes/messageRoutes');
app.use('/api/message', messageRoutes);

const notificationRoutes = require('../src/routes/notificationRoutes');
app.use('/api/notification', notificationRoutes);

const searchRoutes = require('../src/routes/searchRoutes');
app.use('/api/search', searchRoutes);

const settingsRoutes = require('../src/routes/settingsRoutes');
app.use('/api/settings', settingsRoutes);