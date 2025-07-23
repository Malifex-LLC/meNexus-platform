// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

export const createPost = (publicKey, activeBoard, content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO Posts (public_key, board, content)
            VALUES (?, ?, ?)
        `;

        meNexus.query(sql, [publicKey, activeBoard, content], (err, result) => {
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

export const getBoardPosts = async (board) => {
    return new Promise((resolve, reject) => {
        const query = `
            SELECT *
            FROM Posts
            WHERE Posts.board = ?
            ORDER BY Posts.created_at DESC
        `;
        meNexus.query(query, board, async (err, results) => {
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
export const getPosts = async (publicKey) => {
    try {
        const user = await getUserByPublicKeyFromDB(publicKey);

        if (!user) {
            throw new Error(`User not found in globalUsers: ${publicKey}`);
        }

        // Combine own publicKey + following list
        const publicKeysToQuery = [publicKey, ...(user.following || [])];

        // If there's no one to query, return an empty array early
        if (publicKeysToQuery.length === 0) {
            return [];
        }

        // Create a dynamic placeholders string like (?, ?, ?)
        const placeholders = publicKeysToQuery.map(() => '?').join(', ');

        const sql = `
            SELECT *
            FROM Posts
            WHERE public_key IN (${placeholders})
            ORDER BY created_at DESC
        `;

        return await new Promise((resolve, reject) => {
            meNexus.query(sql, publicKeysToQuery, async (err, results) => {
                if (err) {
                    console.error('Error fetching posts:', err);
                    return reject(new Error(err));
                }

                try {
                    const enrichedPosts = await Promise.all(
                        results.map(async (post) => {
                            const postUser = await getUserByPublicKeyFromDB(post.public_key);
                            return {
                                ...post,
                                handle: postUser?.handle || 'Unknown',
                                displayName: postUser?.displayName || 'Unknown'
                            };
                        })
                    );
                    resolve(enrichedPosts);
                } catch (error) {
                    console.error('Error enriching posts:', error);
                    reject(error);
                }
            });
        });

    } catch (error) {
        console.error('Error in getPosts:', error);
        throw error;
    }
};


export const getUserPosts = (publicKey) => {
    return new Promise((resolve, reject) => {
        // SQL is performing an inner join on Posting and Users tables where post.user_id == users.user_id
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

export const uploadPostMedia = async ({ postId, publicKey, mediaUrl, filename, mimetype }) => {

    return new Promise((resolve, reject) => {
        const sql = `
            UPDATE Posts
            SET media_url = ?
            WHERE post_id = ?
        `;

        meNexus.query(sql, [mediaUrl, postId], (err, result) => {
            if (err) {
                console.error(err)
                return reject(new Error('Database error'));
            }
            resolve(result);
        });
    });
};


export default {
    createPost,
    updatePost,
    deletePost,
    getPost,
    getAllPosts,
    getBoardPosts,
    getPosts,
    getUserPosts,
    uploadPostMedia
}