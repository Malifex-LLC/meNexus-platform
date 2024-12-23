// Imports
const express = require('express')
const session = require('express-session');
const bodyParser = require('body-parser');
const cors = require('cors');
const morgan = require('morgan');
const mysql = require('mysql2');
require('dotenv').config();
const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;
const bcrypt = require('bcrypt');
const multer = require('multer');
const path = require('path');
const ws = require('ws')

// Instantiate Express app
const app = express()

// Configure Express app for CORS
app.use(cors({
    origin: 'http://localhost:5173', // client's origin
    credentials: true, // Allow credentials (cookies) to be sent
}));

// Assigning port number for the express server
const port = process.env.EXPRESS_PORT;

// Express server listening on port number specified
const httpServer = app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
});

// Instantiate WebSocket Server
const wss = new ws.Server({ noServer: true })

// Store connected clients
const clients = new Map();

// Configure WebSocket Server for CORS
wss.on('headers', (headers, req) => {
    headers.push('Access-Control-Allow-Origin: http://localhost:5173');
    headers.push('Access-Control-Allow-Credentials: true');
});


wss.on('connection', (ws, request) => {
    const urlParams = new URLSearchParams(request.url.split('?')[1]);
    const user_id = urlParams.get('user_id'); // Extract user_id from the query string
    ws.isAlive = true;

    if (!user_id) {
        console.error("WebSocket connection attempted without user_id.");
        ws.close();
        return;
    }

    console.log(`WebSocket connection established for user_id: ${user_id}`);
    clients.set(user_id, ws);

    ws.on('pong', () => {
        ws.isAlive = true; // Reset isAlive when a pong is received
    });

    ws.on('message', (message) => {
        console.log(`Received message from user ${user_id}: ${message}`);
    });

    ws.on('close', () => {
        console.log(`WebSocket connection closed for user_id: ${user_id}`);
        clients.delete(user_id);
    });
});

httpServer.on('upgrade', (req, socket, head) => {
    const url = new URL(req.url, `http://${req.headers.host}`);
    const user_id = url.searchParams.get('user_id'); // Extract user_id from the query params

    if (!user_id) {
        console.error("No user_id provided in WebSocket connection");
        socket.write('HTTP/1.1 400 Bad Request\r\n\r\n');
        socket.destroy();
        return;
    }

    wss.handleUpgrade(req, socket, head, (ws) => {
        ws.user_id = user_id; // Attach user_id to the WebSocket instance
        wss.emit('connection', ws, req);
    });
});

// Periodically ping clients to check activity
setInterval(() => {
    wss.clients.forEach((ws) => {
        if (!ws.isAlive) {
            console.log(`Terminating inactive connection for user: ${ws.user_id}`);
            return ws.terminate();
        }

        ws.isAlive = false; // Reset and send ping
        ws.ping();
    });
}, 10000); // Run every 10 seconds


app.use(morgan('combined'));

app.use(express.json());

// Initialize session middleware
app.use(session({
    // TODO replace with a secure, random session secret
    secret: 'your-secret-key', // Replace with a secure key
    resave: false,
    saveUninitialized: false,
    cookie: {
        httpOnly: true,
        secure: false, // Set to true only if using HTTPS
        sameSite: 'Lax', // Allow cross-origin requests
    },
}));

// Initialize passport middleware
app.use(passport.initialize());
app.use(passport.session());

app.use((req, res, next) => {
    console.log('Middleware Debugging');
    console.log('Cookies:', req.headers.cookie);
    console.log('Session ID:', req.sessionID);
    console.log('Session Data:', req.session);
    console.log('Session Store:', req.sessionStore);
    req.sessionStore.all((err, sessions) => {
        console.log('All Sessions:', sessions);
    });
    next();
});

// Middleware function to check session data
const checkSessionData = (req, res, next) => {
    console.log('Session Data:');
    console.log(req.session);
    next(); // Move to the next middleware or route handler
};

// Use the checkSessionData middleware
app.use(checkSessionData);

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
        console.log(err);
    } else {
        console.log("Connected to myNexus!");
    }
});

// Configure Multer storage
const storage = multer.diskStorage({
    destination: (req, file, cb) => {
        // Determine the upload folder based on the uploadType
        const uploadType = req.body.uploadType || req.query.uploadType;

        let folderPath;
        if (uploadType === 'profilePicture') {
            folderPath = path.join(__dirname, 'uploads/profile_pictures');
        } else if (uploadType === 'postMedia') {
            folderPath = path.join(__dirname, 'uploads/post_media');
        } else {
            folderPath = path.join(__dirname, 'uploads/others'); // Fallback folder
        }

        cb(null, folderPath); // Set the folder path
    },
    filename: (req, file, cb) => {
        // Create a unique filename with a timestamp
        cb(null, `${Date.now()}-${file.originalname}`);
    },
});

// Initialize Multer with the storage configuration
const upload = multer({
    storage: storage,
    limits: { fileSize: 5 * 1024 * 1024 }, // Limit file size to 5MB
    fileFilter: (req, file, cb) => {
        // Allow only certain file types
        const fileTypes = /jpeg|jpg|png/;
        const extName = fileTypes.test(file.originalname.toLowerCase());
        const mimeType = fileTypes.test(file.mimetype);

        if (extName && mimeType) {
            return cb(null, true);
        } else {
            cb(new Error('Only images (JPEG, JPG, PNG) are allowed!'));
        }
    },
});

// Serve static files from /uploads directory
app.use('/uploads', express.static('uploads'));


///////////////////////////////////////////Passportjs///////////////////////////////////////////

// Define a local authentication strategy
const User = require('./models/user');
passport.use(new LocalStrategy(
    {
        usernameField: 'email',
        passwordField: 'password',
    },
    async (email, password, done) => {
        try {
            // Fetch user and authentication details by email
            const auth = await User.getAuthByEmail(email);

            if (!auth) {
                return done(null, false, { message: 'Incorrect email or password' });
            }

            // Verify the password
            const isMatch = await bcrypt.compare(password, auth.hashed_password);
            if (!isMatch) {
                return done(null, false, { message: 'Incorrect email or password' });
            }

            // Fetch user details from Users table
            const user = await User.getUserById(auth.user_id);
            if (!user) {
                return done(null, false, { message: 'User not found' });
            }

            // Merge user and authentication data
            const completeUser = {
                user_id: user.user_id,
                handle: user.handle,
                display_name: user.display_name,
                email: auth.email,
            };

            return done(null, completeUser);
        } catch (error) {
            return done(error);
        }
    }
));

// Serialize user into session
passport.serializeUser((user, done) => {
    done(null, user.user_id); // Use user_id as the identifier
});

// Deserialize user from session
passport.deserializeUser(async (user_id, done) => {
    try {
        // Fetch user details
        const user = await User.getUserById(user_id);
        if (user) {
            done(null, user);
        } else {
            done(null, false);
        }
    } catch (error) {
        done(error, null);
    }
});


///////////////////////////////////////////API endpoints///////////////////////////////////////////

// API endpoint to ensure server is running
app.get('/ping', (req, res) => {
    console.log(req);
    res.send('Server is live.');
})

// API endpoint to create a user
app.post('/createUser', async (req, res) => {
    console.log('/createUser FIRED');
    try {
        const { email, password, handle, display_name } = req.body;
        console.log("Received data:", { email, password, handle, display_name });

        // Validate required fields
        if (!email || !password || !handle || !display_name) {
            console.log("Missing required fields.");
            return res.status(400).json({ error: 'All fields are required' });
        }

        // Check if the email is already used by another user
        const existingUserByEmail = await User.getUserByEmail(email);
        console.log("Existing user by email:", existingUserByEmail);

        if (existingUserByEmail) {
            console.log("Email is already taken.");
            return res.status(400).json({ error: 'Email is already taken' });
        }

        // Check if the handle is already used by another user
        const existingUserByHandle = await User.getUserByHandle(handle);
        console.log("Existing user by handle:", existingUserByHandle);

        if (existingUserByHandle) {
            console.log("Handle is already taken.");
            return res.status(400).json({ error: 'Handle is already taken' });
        }

        // Call the createUser function from the User model
        const newUserId = await User.createUser(email, password, handle, display_name);
        console.log("New user created with ID:", newUserId);

        // Return a success response
        return res.status(201).json({ message: 'User created successfully', user_id: newUserId });
    } catch (error) {
        console.error("Error in /createUser:", error.message);
        return res.status(500).json({ error: 'Failed to create user' });
    }
});

// API endpoint to update password
app.put('/updatePassword', async (req, res) => {
    const { user_id, newPassword } = req.body;
    if (!user_id || !newPassword) {
        return res.status(400).json({ error: "User ID and new password are required" });
    }

    try {
        // Hash the new password
        const hashedPassword = await bcrypt.hash(newPassword, 10);

        // Update the password in the Authentication table
        const sql = 'UPDATE Authentication SET hashed_password = ? WHERE user_id = ?';
        meNexus.query(sql, [hashedPassword, user_id], (err, result) => {
            if (err) {
                console.error("Error updating password:", err);
                return res.status(500).json({ error: "Error updating password" });
            }

            if (result.affectedRows === 0) {
                return res.status(404).json({ error: "User not found" });
            }

            console.log("Password updated successfully for user:", user_id);
            res.json({ message: "Password updated successfully" });
        });
    } catch (error) {
        console.error("Error updating password:", error.message);
        res.status(500).json({ error: "Error updating password" });
    }
});

// API endpoint to handle login request
app.post('/login', (req, res, next) => {
    console.log('/login called');
    passport.authenticate('local', (err, user, info) => {
        if (err) {
            // Handle error
            console.error('Error during login authentication:', err);
            return res.status(500).json({ error: 'An error occurred during login' });
        }
        if (!user) {
            // Authentication failed
            return res.status(401).json({ error: info ? info.message : 'Incorrect email or password' });
        }
        // Log in the user
        req.login(user, (err) => {
            if (err) {
                // Handle error
                console.error('Error during session login:', err);
                return res.status(500).json({ error: 'Failed to log in' });
            }
            // Authentication successful, attach session data
            req.session.user = {
                user_id: user.user_id,
                handle: user.handle,
                display_name: user.display_name,
            };
            console.log('Session Data:', req.session.user);

            return res.json({
                message: 'Login successful',
                handle: user.handle,
                display_name: user.display_name,
            });
        });
    })(req, res, next);
});

// API endpoint to handle logout request and destroy session
app.post('/logout', (req, res) => {
    console.log('/logout called');
    req.logout((err) => {
        if (err) {
            console.error('Error during logout:', err);
            return res.status(500).json({ error: 'Logout failed' });
        }

        req.session.destroy((destroyErr) => {
            if (destroyErr) {
                console.error('Error destroying session:', destroyErr);
                return res.status(500).json({ error: 'Failed to destroy session' });
            }

            // Clear the session cookie
            res.clearCookie('connect.sid', { path: '/' });
            console.log('User successfully logged out and session cleared.');
            return res.status(200).json({ message: 'Logged out successfully' });
        });
    });
});

// API endpoint that queries meNexus database for all user accounts
app.get('/getUsers', (req, res) => {
    let sql = 'SELECT * FROM Users';
    let query = meNexus.query(sql, (err, results) => {
        if (err) {
            console.log(err);
        } else {
            console.log(results);
            res.send(results);
        }
    })
});

// API endpoint to fetch a user's profile by handle
app.get('/getProfile/:handle', (req, res) => {
    const handle = req.params.handle;

    const sql = `
        SELECT
            Profiles.user_id,
            Profiles.profile_name,
            Profiles.profile_bio,
            Profiles.profile_location,
            Profiles.profile_picture,
            Profiles.profile_banner,
            Profiles.custom_css,
            Users.display_name,
            Users.handle
        FROM Profiles
                 INNER JOIN Users ON Profiles.user_id = Users.user_id
        WHERE Users.handle = ?;
    `;

    meNexus.query(sql, [handle], (err, results) => {
        if (err) {
            console.error('Error fetching user profile:', err);
            res.status(500).json({ error: 'Error fetching user profile' });
        } else if (results.length === 0) {
            res.status(404).json({ error: 'Profile not found' });
        } else {
            res.json(results[0]); // Send the first (and only) result
        }
    });
});

// API endpoint to fetch the current user from the session
app.get('/getCurrentUser', (req, res) => {
    console.log('getCurrentUser called');
    console.log('Session ID:', req.sessionID);
    console.log('Session Data:', req.session);

    if (req.session && req.session.user) {
        const { user_id, handle, display_name } = req.session.user;

        // Ensure the session data contains all necessary fields
        if (!user_id || !handle || !display_name) {
            console.error('Incomplete session data');
            return res.status(400).json({ error: 'Incomplete session data' });
        }

        // Send the user data stored in the session
        console.log('User found in session:', req.session.user);
        return res.json({
            user_id,
            handle,
            display_name
        });
    } else {
        console.log('User not authenticated');
        return res.status(401).json({ error: 'User not authenticated' });
    }
});

// Endpoint to upload profile picture
app.post('/settings/uploadProfilePicture', upload.single('profile_picture'), async (req, res) => {
    const uploadType = req.body.uploadType;
    console.log('uploadType: ', req.body.uploadType)
    try {
        // Validate session user
        const { user_id } = req.session.user // Extract user ID from the session
        if (!user_id) {
            return res.status(401).json({ error: 'User not authenticated' });
        }

        // Validate uploaded file
        if (!req.file) {
            return res.status(400).json({ error: 'No file uploaded' });
        }

        const profilePicturePath = `/uploads/profile_pictures/${req.file.filename}`;
        console.log('Profile Picture Path:', profilePicturePath);

        // Update the database with the new profile picture path
        const sql = 'UPDATE Profiles SET profile_picture = ? WHERE user_id = ?';
        const query = meNexus.query(sql, [profilePicturePath, user_id], (err, result) => {
            if (err) {
                console.error('Error updating profile picture:', err.message);
                return res.status(500).json({ error: 'Failed to update profile picture' });
            }

            console.log('Database Update Result:', result);
            res.json({ message: 'Profile picture uploaded successfully', profile_picture: profilePicturePath });
        });
    } catch (error) {
        console.error('Error in profile picture upload:', error.message);
        res.status(500).json({ error: 'Internal server error' });
    }
});

// API endpoint that queries database for all posts from a specified handle
app.get('/getUserPosts/:handle', (req, res) => {
    const handle = req.params.handle || req.query.handle;
    console.log(`GetPostsFired for ${handle}`)
    // SQL is performing an inner join on Posts and Users tables where post.user_id == users.user_id
    let sql = `
        SELECT Posts.*, Users.display_name, Users.handle
        FROM Posts
        INNER JOIN Users ON Posts.user_id = Users.user_id
        WHERE Users.handle = ?
    `;
    let query = meNexus.query(sql, [handle], (err, results) => {
        if (err) {
            console.log(err);
            res.status(500).send('Error fetching user posts');
        } else {
            res.send(results);
        }
    });
});

// API endpoint to aggregate posts
app.get('/getPosts', (req, res) => {
    const { user_id } = req.session.user; // Get the current user's ID

    if (!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const sql = `
        SELECT Posts.*, Users.display_name, Users.handle
        FROM Posts
        INNER JOIN Users ON Posts.user_id = Users.user_id
        WHERE Posts.user_id = ?
        OR Posts.user_id IN (
            SELECT followed_id
            FROM Followers
            WHERE follower_id = ?
        )
        ORDER BY Posts.created_at DESC
    `;

    meNexus.query(sql, [user_id, user_id], (err, results) => {
        if (err) {
            console.error('Error fetching posts:', err);
            return res.status(500).json({ error: 'Internal server error' });
        }

        res.json(results); // Return posts in descending order of creation time
    });
});

// API to get a single post by post_id
app.get('/getPost', (req, res) => {
    const { user_id } = req.session.user; // Get the current user's ID
    const { post_id } = req.query; // Get the post ID from the route

    if (!user_id) {
        return res.status(401).json({ error: 'User not authenticated' });
    }

    const sql = `
        SELECT Posts.*, Users.display_name, Users.handle
        FROM Posts
        INNER JOIN Users ON Posts.user_id = Users.user_id
        WHERE Posts.post_id = ?
        AND (
            Posts.user_id = ?
            OR Posts.user_id IN (
                SELECT followed_id
                FROM Followers
                WHERE follower_id = ?
            )
        )
        LIMIT 1
    `;

    meNexus.query(sql, [post_id, user_id, user_id], (err, results) => {
        if (err) {
            console.error("Error fetching post details:", err);
            return res.status(500).json({ error: "Internal server error" });
        }
        if (results.length === 0) {
            return res.status(404).json({ error: "Post not found or access denied" });
        }
        res.json(results[0]); // Return the single post with user details
    });
});


// API endpoint for submitting a post
app.post("/createPost", (req, res) => {
    const { content, handle } = req.body;
    // Fetch the user_id based on the handle
    const userSql = "SELECT user_id FROM Users WHERE handle = ?";
    meNexus.query(userSql, [handle], (userErr, userResult) => {
        if (userErr) {
            console.error(userErr);
            return res.status(500).json({ error: "Failed to fetch user data." });
        }
        if (userResult.length === 0) {
            return res.status(404).json({ error: "User not found." });
        }
        const user_id = userResult[0].user_id;
        // Insert the post into the database
        const postSql = "INSERT INTO Posts (content, user_id) VALUES (?, ?)";
        meNexus.query(postSql, [content, user_id], (postErr, postResult) => {
            if (postErr) {
                console.error(postErr);
                return res.status(500).json({ error: "Failed to submit the post." });
            }
            // Return a success response
            return res.json({ message: "Post submitted successfully." });
        });
    });
});

// API endpoint to update a post
app.put("/updatePost/:postId", (req, res) => {
    const postId = req.params.postId;
    const updatedContent = req.body.content;
    // Update the post in database
    const updateSql = "UPDATE Posts SET content = ? WHERE post_id = ?";
    meNexus.query(updateSql, [updatedContent, postId], (updateErr, updateResult) => {
        if (updateErr) {
            console.error(updateErr);
            return res.status(500).json({ error: "Failed to update the post." });
        }
        // Check if any rows were affected
        if (updateResult.affectedRows === 0) {
            return res.status(404).json({ error: "Post not found." });
        }
        // Return a success response
        return res.json({ message: "Post updated successfully." });
    });
});

// Api endpoint for deleting a post given a specified postId
app.delete("/deletePost/:post_id", (req, res) => {
    const postId = req.params.post_id;
    // Delete the post from the database
    const deleteSql = "DELETE FROM Posts WHERE post_id = ?";
    meNexus.query(deleteSql, [postId], (deleteErr, deleteResult) => {
        if (deleteErr) {
            console.error(deleteErr);
            return res.status(500).json({ error: "Failed to delete the post." });
        }
        // Check if any rows were affected
        if (deleteResult.affectedRows === 0) {
            return res.status(404).json({ error: "Post not found." });
        }
        // Return a success response
        return res.json({ message: "Post deleted successfully." });
    });
});

// API endpoint for creating a new comment
app.post("/createComment", (req, res) => {
    const { user_id } = req.session.user
    const { resource_type, resource_id, content } = req.body;

    const sql = `
        INSERT INTO PostComments (user_id, resource_type, resource_id, content) VALUES (?, ?, ?, ?)
    `;
    meNexus.query(sql, [user_id, resource_type, resource_id, content], (commentErr, commentResult) => {
        if (commentErr) {
            console.error(commentErr);
            return res.status(500).json({ error: "Failed to create a comment." });
        }
        return res.json({ message: "Comment submitted successfully." });
    })
});

// API endpoint for updating a comment
app.put("/updateComment/:comment_id", (req, res) => {
    const comment_id = req.params.comment_id;
    const updatedContent = req.body.content;
    // Update the Comment in database
    const sql = 'UPDATE PostComments SET content = ? WHERE comment_id = ?';
    meNexus.query(sql, [updatedContent, comment_id], (commentErr, commentResult) => {
        if (commentErr) {
            console.error(commentErr);
            return res.status(500).json({ error: "Failed to update the comment." });
        }
        // Check if any rows were affected
        if (commentResult.affectedRows === 0) {
            return res.status(404).json({ error: "Comment not found." });
        }
        return res.json({ message: "Comment updated successfully." });
    });
});

// API endpoint for deleting a comment
app.delete("/deleteComment/:comment_id", (req, res) => {
    const comment_id = req.params.comment_id;
    // Delete the comment from the database
    const deleteSql = "DELETE FROM PostComments WHERE comment_id = ?";
    meNexus.query(deleteSql, [comment_id], (deleteErr, deleteResult) => {
        if (deleteErr) {
            console.error(deleteErr);
            return res.status(500).json({ error: "Failed to delete the comment." });
        }
        // Check if any rows were affected
        if (deleteResult.affectedRows === 0) {
            return res.status(404).json({ error: "Comment not found." });
        }
        // Return a success response
        return res.json({ message: "Comment submitted successfully." });
    })
});

// API endpoint for getting comments
app.get("/getComments", (req, res) => {
    const {resource_type, resource_id } = req.query;

    console.log('/getComments called for query: ', req.query);

    if (!resource_id || resource_id.trim() === "") {
        return res.status(400).json({ error: "Invalid getComments query." });
    }

    let sql = "";
    const params = [`%${resource_id}%`];

    switch (resource_type) {
        case "POST":
            sql = `
                SELECT
                    PostComments.comment_id,
                    PostComments.user_id AS comment_user_id,
                    PostComments.resource_id,
                    PostComments.resource_type,
                    PostComments.content AS comment_content,
                    PostComments.created_at AS comment_created_at,
                    PostComments.updated_at AS comment_updated_at,
                    Posts.post_id,
                    Posts.content AS post_content,
                    Posts.user_id AS post_user_id,
                    Posts.media_url,
                    Posts.created_at AS post_created_at,
                    Users.display_name,
                    Users.handle
                FROM PostComments
                         INNER JOIN Posts ON PostComments.resource_id = Posts.post_id
                         INNER JOIN Users ON PostComments.user_id = Users.user_id
                WHERE Posts.post_id = ?
            `;
            meNexus.query(sql, [resource_id], (err, results) => {
                if (err) {
                    console.error("Error getting post comments", err);
                    return res.status(500).json({ error: 'Internal server error' });
                }
                const comments = results;
                console.log('getComments for post results:', results);
                res.json(comments);
            })
            break;

        default:
                sql = `
                SELECT * FROM PostComments WHERE post_id = ?;
                `;
            break;


    }
});

// API endpoint for following a user
app.post('/followUser', async (req, res) => {
    const { user_id } = req.session.user; // Get the follower's user ID from the session
    const { followed_id } = req.body;
    console.log("handle Follow user: ", followed_id, " for user: ", user_id);


    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const sql = 'INSERT INTO Followers (follower_id, followed_id) VALUES (?, ?)';
    meNexus.query(sql, [user_id, followed_id], (err, result) => {
        if (err) {
            console.error('Error adding follow:', err.message);
            return res.status(500).json({ error: 'Failed to follow user' });
        }
        res.json({ message: 'Follow successful' });
    });
});

//API endpoint for unfollowing a user
app.delete('/unfollowUser', async (req, res) => {
    const { user_id } = req.session.user; // Get the follower's user ID from the session
    const { followed_id } = req.body;

    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    const sql = 'DELETE FROM Followers WHERE follower_id = ? AND followed_id = ?';
    meNexus.query(sql, [user_id, followed_id], (err, result) => {
        if (err) {
            console.error('Error removing follow:', err.message);
            return res.status(500).json({ error: 'Failed to unfollow user' });
        }
        res.json({ message: 'Unfollow successful' });
    });
});

// API endpoint to check is a user is being followed
app.get('/followCheck', (req, res) => {
    const { user_id } = req.session.user; // Get the current user's ID
    const { followed_id } = req.query; // ID of the user being checked
    console.log("server handling followCheck for followed_id: ", followed_id, "for user_id: ", user_id);

    if (!user_id || !followed_id) {
        return res.status(400).json({ error: 'Invalid data' });
    }

    const sql = `
        SELECT * FROM Followers WHERE follower_id = ? AND followed_id = ?
    `;

    meNexus.query(sql, [user_id, followed_id], (err, results) => {
        if (err) {
            console.error('Error checking follow status:', err);
            return res.status(500).json({ error: 'Internal server error' });
        }

        const isFollowing = results.length > 0; // If a record exists, the user is following
        console.log("isFollowing: ", isFollowing);
        res.json({ isFollowing });
    });
});

// API endpoint for search functionality
app.get('/search', async (req, res) => {
    const { query, type } = req.query;

    if (!query || query.trim() === "") {
        return res.status(400).json({ error: 'Search query is required' });
    }

    let sql = "";
    const params = [`%${query}%`];

    switch (type) {
        case "users":
            sql = `
                SELECT handle, display_name
                FROM Users
                WHERE handle LIKE ? OR display_name LIKE ?
            `;
            params.push(`%${query}%`);
            break;

        case "posts":
            sql = `
                SELECT Posts.content, Posts.post_id, Posts.user_id, Posts.created_at,
                       Users.handle, Users.display_name
                FROM Posts
                         INNER JOIN Users ON Posts.user_id = Users.user_id
                WHERE Posts.content LIKE ?
            `;
            break;

        default: // Handle both users and posts
            const userQuery = `
                SELECT
                    'user' AS type,
                    handle,
                    display_name,
                    user_id,
                    NULL AS content,
                    NULL AS post_id,
                    NULL AS created_at
                FROM Users
                WHERE handle LIKE ? OR display_name LIKE ?
            `;
            const postQuery = `
                SELECT
                    'post' AS type,
                    Users.handle,
                    Users.display_name,
                    Posts.user_id,
                    Posts.content,
                    Posts.post_id,
                    Posts.created_at
                FROM Posts
                         INNER JOIN Users ON Posts.user_id = Users.user_id
                WHERE Posts.content LIKE ?
            `;
            sql = `(${userQuery}) UNION ALL (${postQuery})`;
            params.push(`%${query}%`, `%${query}%`);
            break;
    }

    try {
        const results = await new Promise((resolve, reject) => {
            meNexus.query(sql, params, (err, results) => {
                if (err) return reject(err);
                resolve(results);
            });
        });

        res.json({ type, results });
    } catch (err) {
        console.error("Error executing search:", err);
        res.status(500).json({ error: "Internal server error" });
    }
});

// Helper to send notifications
const sendNotification = (userId, notification) => {
    console.log(`sendNotification called with userId: ${userId} (type: ${typeof userId})`);
    console.log("Current WebSocket clients:", Array.from(clients.keys()));

    const client = clients.get(String(userId)); // Ensure type consistency
    if (!client) {
        console.log(`No WebSocket client found for user_id: ${userId}`);
        return;
    }

    if (client.readyState !== WebSocket.OPEN) {
        console.log(`WebSocket client for user_id: ${userId} is not open. Current state: ${client.readyState}`);
        return;
    }

    console.log(`Sending notification to user_id: ${userId}`, notification);
    client.send(JSON.stringify(notification));
};



// API endpoint for creating notifications
app.post('/createNotification', async (req, res) => {
    const { user_id, actor_id, resource_type, resource_id, action } = req.body;
    console.log("createNotification req: ", req);

    if(!user_id || !actor_id || !resource_type || !resource_id || !action) {
        return res.status(400).json({ error: 'Invalid request data' });
    }

    // Fetch actor's handle from Users table
    const actorQuery = `
        SELECT handle FROM Users WHERE user_id = ?
    `;

    meNexus.query(actorQuery, [actor_id], (actorErr, actorResult) => {
        if (actorErr) {
            console.error("Error fetching actor handle:", actorErr);
            return res.status(500).json({error: "Failed to fetch actor details."});
        }

        if (actorResult.length === 0) {
            return res.status(404).json({error: "Actor not found."});
        }

        const actorHandle = actorResult[0].handle;
        // Generate notification summary based on resource_type
        let summary = "";
        switch (resource_type) {
            case "POST":
                if (action === "COMMENT") {
                    summary = `${actorHandle} commented on your post!`;
                } else if (action === "LIKE") {
                    summary = `${actorHandle} liked your post!`;
                }
                break;
            case "COMMENT":
                if (action === "LIKE") {
                    summary = `${actorHandle} liked your comment!`;
                }
                break;
            case "FOLLOW":
                summary = `${actorHandle} followed you!`;
                break;
            default:
                return res.status(400).json({ error: 'Unsupported resource_type or action' });

        }

        const sql = `
        INSERT INTO Notifications (user_id, actor_id, resource_type, resource_id, action, summary)
        VALUES (?, ?, ?, ?, ?, ?)
    `;
        meNexus.query(sql, [user_id, actor_id, resource_type, resource_id, action, summary], (err, result) => {
            if (err) {
                console.error("Error creating notification:", err);
                return res.status(500).json({ error: "Failed to create notification." });
            }

            // Broadcast via WebSocket
            console.log("Preparing to call sendNotification")
            console.log("Current WebSocket clients:", Array.from(clients.keys()));
            sendNotification(user_id, { summary, is_read: 0, created_at: new Date() });

            res.json({ message: "Notification created successfully." });
        });
    });
});

// API endpoint for updating notification
app.put('/updateNotification', async (req, res) => {
    const { notification_id } = req.body;

    if (!notification_id) {
        return res.status(400).json({ error: "Notification ID is required." });
    }

    let sql = `UPDATE Notifications SET is_read = 1 WHERE notification_id = ?`;
    meNexus.query(sql, [notification_id], (err, result) => {
        if (err) {
            console.error("Error updating notification:", err);
            return res.status(500).json({ error: "Failed to update notification." });
        }

        if (result.affectedRows === 0) {
            return res.status(404).json({ error: "Notification not found or already updated." });
        }

        return res.status(200).json({ message: "Notification updated successfully." });
    });
});

// API endpoint for getting notifications
app.get("/getNotifications", (req, res) => {
    const { user_id } = req.session.user;
    console.log("/getNotifications called for user_id: ", user_id);

    if (!user_id) {
        return res.status(400).json({ error: 'No authorized user' });
    }

    let sql = `
    SELECT * FROM Notifications WHERE user_id = ? AND is_read = false;
    `;
    meNexus.query(sql, [user_id], (err, results) => {
        if (err) {
            console.error("Error fetching notifications for user_id: ", user_id);
            res.status(500).json({ error: 'Error fetching notifications' });
        } else {
            res.json({ notifications: results });
        }
    })
});







