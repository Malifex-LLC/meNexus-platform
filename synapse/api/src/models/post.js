import meNexus from "../../config/mysql.js";
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

export const createPost = (publicKey, content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO Posts (public_key, content)
            VALUES (?, ?)
        `;

        meNexus.query(sql, [publicKey, content], (err, result) => {
            if (err) {
                console.error(err)
                return reject(new Error('Database error'));
            }
            resolve(result.insertId); // Return the new post ID
        });
    });
}

export const updatePost = (postId, updatedContent) => {
    return new Promise((resolve, reject) => {
        // Update the post in orbitdb
        const updateSql = `
            UPDATE Posts 
            SET content = ? 
            WHERE post_id = ?
        `;

        meNexus.query(updateSql, [updatedContent, postId], (updateErr, updateResult) => {
            if (updateErr) {
                console.error(updateErr);
                return reject(new Error('Database error')); // Reject with an error
            }

            resolve(updateResult);
        });
    })
}

export const deletePost = (postId) => {
    return new Promise((resolve, reject) => {
        // Delete the post from the orbitdb
        const deleteSql = `
            DELETE FROM Posts 
            WHERE post_id = ?
        `;
        meNexus.query(deleteSql, [postId], (deleteErr, deleteResult) => {
            if (deleteErr) {
                console.error(deleteErr);
                return reject(new Error('Database error')); // Reject with an error
            }
            resolve(deleteResult);
        });
    })
}

export const getPost = (postId) => {
    console.log('getPost() called with postId:', postId);
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT *
            FROM Posts
            WHERE Posts.post_id = ?
        `;
        meNexus.query(sql, [postId], async (err, result) => {
            if (err) {
                console.error('Error fetching post:', err);
                return reject(new Error(err));
            }
            try {
                const enrichedPost = await Promise.all(result.map(async (post) => {
                    const user = await getUserByPublicKeyFromDB(post.public_key);
                    return {
                        ...post,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown'
                    };
                }));
                resolve(enrichedPost[0]);
            } catch (error) {
                console.error('Error enriching posts:', error);
                reject(error);
            }
        })
    })
}

export const getAllPosts = async (req, res) => {
    return new Promise((resolve, reject) => {
        const query = `
            SELECT *
            FROM Posts
            ORDER BY Posts.created_at DESC
        `;
        meNexus.query(query, async (err, results) => {
            if (err) {
                console.error('Database error in getAllPosts:', err);
                return reject(err)
            }
            try {
                const enrichedPosts = await Promise.all(results.map(async (post) => {
                    const user = await getUserByPublicKeyFromDB(post.public_key);
                    return {
                        ...post,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown'
                    };
                }));
                resolve(enrichedPosts);
            } catch (error) {
                console.error('Error enriching posts:', error);
                reject(error);
            }
        });
    });
};

// TODO getUserByPublicKeyFromDB(post.public_key) in a loop is a performance bottleneck
export const getPosts = (publicKey) => {
    return new Promise(async (resolve, reject) => {
        const sql = `
            SELECT Posts.*
            FROM Posts
            WHERE Posts.public_key = ?
               OR Posts.public_key IN (SELECT followed_public_key
                                       FROM Followers
                                       WHERE follower_public_key = ?)
            ORDER BY Posts.created_at DESC
        `;
        meNexus.query(sql, [publicKey, publicKey], async (err, results) => {
            if (err) {
                console.error('Error fetching posts:', err);
                return reject(new Error(err)); // Reject with an error
            }
            try {
                const enrichedPosts = await Promise.all(results.map(async (post) => {
                    const user = await getUserByPublicKeyFromDB(post.public_key);
                    return {
                        ...post,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown'
                    };
                }));
                resolve(enrichedPosts);
            } catch (error) {
                console.error('Error enriching posts:', error);
                reject(error);
            }
        });

    })
}

export const getUserPosts = (publicKey) => {
    return new Promise((resolve, reject) => {
        // SQL is performing an inner join on Posts and Users tables where post.user_id == users.user_id
        const sql = `
            SELECT *
            FROM Posts
            WHERE Posts.public_key = ?
    `;
        meNexus.query(sql, publicKey, async (err, results) => {
            if (err) {
                console.log('Error getting user posts:', err);
                return reject(new Error(err)); // Reject with an error
            }
            try {
                const enrichedPosts = await Promise.all(results.map(async (post) => {
                    const user = await getUserByPublicKeyFromDB(post.public_key);
                    return {
                        ...post,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown'
                    };
                }));
                resolve(enrichedPosts);
            } catch (error) {
                console.error('Error enriching posts:', error);
                reject(error);
            }
        });
    })
}

export default {
    createPost,
    updatePost,
    deletePost,
    getPost,
    getAllPosts,
    getPosts,
    getUserPosts,
}