const meNexus = require('../../config/mysql.js');

// Fetch authentication details by email
exports.getAuthByEmail = (email) => {
    return new Promise((resolve, reject) => {
        const query = `
            SELECT * 
            FROM Authentication 
            WHERE email = ?
        `;

        meNexus.query(query, [email], (err, results) => {
            if (err) {
                console.error('Database error in getAuthByEmail:', err);
                return reject(err);
            }

            resolve(results[0]); // Return the first result (or undefined if none found)
        });
    });
};

// Fetch user by email (Authentication table)
exports.getUserByEmail = async (email) => {
    return new Promise((resolve, reject) => {
        const query = `
            SELECT u.*, a.email, a.auth_provider
            FROM Users u
            INNER JOIN Authentication a ON u.user_id = a.user_id
            WHERE a.email = ?
        `;

        meNexus.query(query, [email], (err, results) => {
            if (err) {
                console.error('Database error in getUserByEmail:', err);
                return reject(err);
            }

            resolve(results[0]);
        });
    });
};

exports.updateAccountSettings = (authFields, authValues) => {
    return new Promise((resolve, reject) => {
        const authSql = `
            UPDATE Authentication 
            SET ${authFields.join(', ')} 
            WHERE user_id = ?;
        `;

        meNexus.query(authSql, authValues, (err, results) => {
            if (err) {
                return reject(err);
            }

            resolve(results);
        })
    })
}