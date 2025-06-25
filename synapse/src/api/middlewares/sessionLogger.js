// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

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