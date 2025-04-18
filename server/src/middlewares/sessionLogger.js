const fs = require('fs');
const path = require('path');

const logsDir = path.join(__dirname, '../logs');

// Create the logs directory if it doesn't exist
if (!fs.existsSync(logsDir)) {
    fs.mkdirSync(logsDir, { recursive: true });
}

// Simple log file setup — could later switch to Winston or another rotating logger
const logPath = path.join(logsDir, 'session.log');
const logStream = fs.createWriteStream(logPath, { flags: 'a' }); // append mode

const sessionLogger = (req, res, next) => {
    const now = new Date().toISOString();
    const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
    const userAgent = req.headers['user-agent'];
    const sessionID = req.sessionID || 'N/A';
    const user = req.session?.user || {};

    const logEntry = `[${now}] Session ID: ${sessionID} | User ID: ${user.user_id || 'N/A'} | Handle: ${user.handle || 'N/A'} | IP: ${ip} | Agent: ${userAgent}\n`;

    logStream.write(logEntry);

    next();
};

module.exports = sessionLogger;
