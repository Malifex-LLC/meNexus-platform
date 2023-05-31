//Import Express
const express = require('express')
const session = require('express-session');
const cors = require('cors');
const mysql = require('mysql');
require('dotenv').config();
const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;

//Instantiate Express app called "server"
const server = express()
server.use(cors());

// Use Express session middleware
server.use(session({
    secret: 'secret',
    resave: false,
    saveUninitialized: false
}));

// Use Passport middleware
server.use(passport.initialize());
server.use(passport.session());

// Define a local authentication strategy
// TODO Update authentication strategy so that registration and login is functional
passport.use(new LocalStrategy({
        usernameField: 'email',
        passwordField: 'password'
    },
    (email, password, done) => {
        let sql = `SELECT * FROM users WHERE email = '${email}'`;
        social_database.query(sql, (err, results) => {
            if (err) {
                return done(err);
            }
            if (!results.length) {
                return done(null, false, { message: 'Incorrect email.' });
            }
            let user = results[0];
            if (user.password !== password) {
                return done(null, false, { message: 'Incorrect password.' });
            }
            return done(null, user);
        });
    }
));

// Assigning port number for the express server
const port = 3001

//Create mySQL Connection
let social_database = mysql.createConnection({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE
});

//Connect to mySQL
social_database.connect((err) => {
    if(err) {
        console.log(err);
    } else {
        console.log("Connected to myNexus!");
    }
});

//API endpoints
server.get('/', (req, res) => {
  res.send('enkiLabs Software')
})

server.get('/ping', (req, res) => {
    console.log(req);
    res.send('Server is live.');
})

server.get("/api", (req, res) => {
    res.json({
        "message" : "Hello from enkiLabs!"
    });    
  });

//Server call that queries social_database for all user accounts
server.get('/getUsers', (req, res) => {
    let sql = 'SELECT * FROM users';
    let query = social_database.query(sql, (err, results) => {
        if (err) {
            console.log(err);
        } else {
            console.log(results);
            res.send(results);
        }
    })
});

//Server call that queries social_database for all userposts
server.get('/getUserPosts', (req, res) => {
    let sql = 'SELECT * FROM userposts';
    let query = social_database.query(sql, (err, results) => {
        if (err) {
            console.log(err);
        } else {
            res.send(results);
        }
    })
});




//Express server listening on port number specified
server.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
})