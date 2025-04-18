// Imports
const createExpressApp = require('./config/express');
const { createWebSocketServer, clients } = require('./config/websocket');
const express = require('express')
const sessionMiddleware = require('./middlewares/session')
const sessionLogger = require('./middlewares/sessionLogger');

// Create Express app
const app = createExpressApp();
//app.use(express.json());

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
app.use('/api/auth', express.json(), authRoutes);

const userRoutes = require('../src/routes/userRoutes');
app.use('/api/user', express.json(), userRoutes);

const followerRoutes = require('./routes/followerRoutes');
app.use('/api/follow', express.json(), followerRoutes);

const postRoutes = require('../src/routes/postRoutes');
app.use('/api/post', express.json(), postRoutes );

const commentRoutes = require('../src/routes/commentRoutes');
app.use('/api/comment', express.json(), commentRoutes);

const conversationRoutes = require('../src/routes/conversationRoutes');
app.use('/api/conversation', express.json(), conversationRoutes);

const messageRoutes = require('../src/routes/messageRoutes');
app.use('/api/message', express.json(), messageRoutes);

const notificationRoutes = require('../src/routes/notificationRoutes');
app.use('/api/notification', express.json(), notificationRoutes);

const searchRoutes = require('../src/routes/searchRoutes');
app.use('/api/search', express.json(), searchRoutes);

const settingsRoutes = require('../src/routes/settingsRoutes');
app.use('/api/settings',  settingsRoutes);