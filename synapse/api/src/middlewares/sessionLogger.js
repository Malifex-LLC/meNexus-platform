const sessionLogger = (req, res, next) => {
    if (req.session) {
        console.log('Session Logger:');
        console.log('Session ID:', req.sessionID);
        console.log('Session Data:', req.session);
    } else {
        console.warn('Session Logger: No session data found.');
    }
    next();
};

export default sessionLogger;