const mysql = require('mysql2');
require('dotenv').config();

// Create a connection pool
const pool = mysql.createPool({
    socketPath  : process.env.DB_SOCKETPATH,
    host        : process.env.DB_HOST,
    port        : process.env.DB_PORT,
    user        : process.env.DB_USER,
    password    : process.env.DB_PASSWORD,
    database    : process.env.DB_DATABASE,
    waitForConnections: true,
    connectionLimit: 10, // adjust based on concurrency needs, not sure currently
    queueLimit: 0
});

//Connect to mySQL
pool.getConnection((err, connection) => {
    if(err) {
        console.log(err);
    } else {
        console.log("Connected to mySQL!");
        connection.release();
    }
})

// Export the connection to use it in other modules
module.exports = pool;