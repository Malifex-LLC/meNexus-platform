const mysql = require('mysql2');
const dotenv =require('dotenv');
dotenv.config({path: '../config/.env'});

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
module.exports = meNexus;