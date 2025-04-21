const mysql = require("../config/db");

exports.createPost = (content, handle) => {
    return new Promise((resolve, reject) => {
        // Fetch user_id from handle
        const userSql = `
            SELECT user_id 
            FROM Users 
            WHERE handle = ?
        `;

        mysql.query(userSql, [handle], (err, results) => {
            if (err) return reject(err);
            if (results.length === 0) return reject(new Error('User not found'));

            const userId = results[0].user_id;

            // Insert the post
            const postSql = `
                INSERT INTO Posts (content, user_id) 
                VALUES (?, ?)
            `;

            mysql.query(postSql, [content, userId], (err, result) => {
                if (err) {
                    return reject(err);
                }

                resolve(result.insertId); // Return the new post ID
            });
        })
    })
}

exports.updatePost = (postId, updatedContent) => {
    return new Promise((resolve, reject) => {
        // Update the post in database
        const updateSql = `
            UPDATE Posts 
            SET content = ? 
            WHERE post_id = ?
        `;

        mysql.query(updateSql, [updatedContent, postId], (updateErr, updateResult) => {
            if (updateErr) {
                console.error(updateErr);
                return reject(new Error('Database error')); // Reject with an error
            }

            resolve(updateResult);
        });
    })
}

exports.deletePost = (postId) => {
    return new Promise((resolve, reject) => {
        // Delete the post from the database
        const deleteSql = `
            DELETE FROM Posts 
            WHERE post_id = ?
        `;

        mysql.query(deleteSql, [postId], (deleteErr, deleteResult) => {
            if (deleteErr) {
                console.error(deleteErr);
                return reject(new Error('Database error')); // Reject with an error
            }

            resolve(deleteResult);
        });
    })
}

exports.getPost = (postId) => {
    console.log('getPost() called with postId:', postId);
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT Posts.*, Users.display_name, Users.handle
            FROM Posts
            INNER JOIN Users ON Posts.user_id = Users.user_id
            WHERE Posts.post_id = ?
        `;


        mysql.query(sql, [postId], (err, result) => {
            if (err) {
                console.error('Error fetching post:', err);
                return reject(new Error(err));
            }
            console.log('getPost() result:', result);
            resolve(result[0]);
        })
    })
}

exports.getPosts = (user_id) => {
    return new Promise((resolve, reject) => {
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

        mysql.query(sql, [user_id, user_id], (err, results) => {
            if (err) {
                console.error('Error fetching posts:', err);
                return reject(new Error(err)); // Reject with an error
            }

            resolve(results); // Return posts in descending order of creation time
        });

    })
}

exports.getUserPosts = (handle) => {
    return new Promise((resolve, reject) => {
        // SQL is performing an inner join on Posts and Users tables where post.user_id == users.user_id
        const sql = `
            SELECT Posts.*, Users.display_name, Users.handle
            FROM Posts
            INNER JOIN Users 
            ON Posts.user_id = Users.user_id
            WHERE Users.handle = ?
    `;

        mysql.query(sql, [handle], (err, results) => {
            if (err) {
                console.log('Error getting user posts:', err);
                return reject(new Error(err)); // Reject with an error
            }

            resolve(results);
        });
    })
}