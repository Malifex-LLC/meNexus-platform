const meNexus = require("../../config/mysql.js");

exports.search = (query, type) => {
    return new Promise((resolve, reject) => {
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

        meNexus.query(sql, params, (err, results) => {
            if (err) {
                return reject(err);
            }

            resolve({type, results});
        });
    });
};