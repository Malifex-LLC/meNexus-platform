const mysql = require("../config/db");

exports.createComment = (user_id, resource_type, resource_id, content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO PostComments (user_id, resource_type, resource_id, content) VALUES (?, ?, ?, ?)
        `;

        mysql.query(sql, [user_id, resource_type, resource_id, content], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        });
    });
};

exports.updateComment = (comment_id, updated_content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            UPDATE PostComments 
            SET content = ? 
            WHERE comment_id = ?
        `;

        mysql.query(sql, [updated_content, comment_id], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        });
    });
};

exports.deleteComment = (comment_id) => {
    return new Promise((resolve, reject) => {
        const deleteSql = `
            DELETE FROM PostComments 
            WHERE comment_id = ?
        `;

        mysql.query(deleteSql, [comment_id], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        })
    })
}

exports.getComments = (resource_type, resource_id) => {
    return new Promise((resolve, reject) => {
        let sql = "";
        const params = [`%${resource_id}%`];

        switch (resource_type) {
            case "POST":
                sql = `
                    SELECT
                        PostComments.comment_id,
                        PostComments.user_id AS comment_user_id,
                        PostComments.resource_id,
                        PostComments.resource_type,
                        PostComments.content AS comment_content,
                        PostComments.created_at AS comment_created_at,
                        PostComments.updated_at AS comment_updated_at,
                        Posts.post_id,
                        Posts.content AS post_content,
                        Posts.user_id AS post_user_id,
                        Posts.media_url,
                        Posts.created_at AS post_created_at,
                        Users.display_name,
                        Users.handle
                    FROM PostComments
                             INNER JOIN Posts ON PostComments.resource_id = Posts.post_id
                             INNER JOIN Users ON PostComments.user_id = Users.user_id
                    WHERE Posts.post_id = ?
                `;

                mysql.query(sql, [resource_id], (error, results) => {
                    if (error) {
                        console.error(error);
                        return reject(error);
                    }

                    resolve(results);
                })
                break;

            default:
                sql = `
                    SELECT * 
                    FROM PostComments 
                    WHERE post_id = ?;
                `;

                break;
        }
    })
}