const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;
const bcrypt = require('bcrypt');
const Auth = require('../models/auth');

// Define a local authentication strategy
const User = require('../models/user');
passport.use(new LocalStrategy(
    {
        usernameField: 'email',
        passwordField: 'password',
    },
    async (email, password, done) => {
        try {
            // Fetch user and authentication details by email
            const auth = await Auth.getAuthByEmail(email);

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

module.exports = passport;