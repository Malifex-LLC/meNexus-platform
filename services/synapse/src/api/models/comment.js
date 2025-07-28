// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

export const createComment = (resourceType, resourceId, content, publicKey) => {
    console.log('createComment called from models:', resourceType, resourceId, content, publicKey);
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO PostComments (resource_type, resource_id, content, public_key) VALUES (?, ?, ?, ?)
        `;

        meNexus.query(sql, [resourceType, resourceId, content, publicKey], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        });
    });
};

export const updateComment = (commentId, updated_content) => {
    return new Promise((resolve, reject) => {
        const sql = `
            UPDATE PostComments 
            SET content = ? 
            WHERE comment_id = ?
        `;

        meNexus.query(sql, [updated_content, commentId], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        });
    });
};

export const deleteComment = (commentId) => {
    return new Promise((resolve, reject) => {
        const deleteSql = `
            DELETE FROM PostComments 
            WHERE commentId = ?
        `;

        meNexus.query(deleteSql, [commentId], (error, result) => {
            if (error) {
                console.error(error);
                return reject(error);
            }

            resolve(result);
        })
    })
}

export const getComments = (resource_type, resource_id) => {
    return new Promise((resolve, reject) => {
        let sql = "";
        const params = [`%${resource_id}%`];

        switch (resource_type) {
            case "POST":
                sql = `
                    SELECT
                        PostComments.comment_id,
                        PostComments.public_key AS comment_public_key,
                        PostComments.resource_id,
                        PostComments.resource_type,
                        PostComments.content AS comment_content,
                        PostComments.created_at AS comment_created_at,
                        PostComments.updated_at AS comment_updated_at,
                        Posts.post_id,
                        Posts.content AS post_content,
                        Posts.public_key AS post_public_key,
                        Posts.media_url,
                        Posts.created_at AS post_created_at
                    FROM PostComments
                             INNER JOIN Posts ON PostComments.resource_id = Posts.post_id
                    WHERE Posts.post_id = ?
                `;

                meNexus.query(sql, [resource_id], async (error, results) => {
                    if (error) {
                        console.error(error);
                        return reject(error);
                    }
                    try {
                        const enrichedPosts = await Promise.all(results.map(async (comment) => {
                            const user = await getUserByPublicKeyFromDB(comment.comment_public_key);
                            return {
                                ...comment,
                                handle: user?.handle || 'Unknown',
                                displayName: user?.displayName || 'Unknown'
                            };
                        }));
                        resolve(enrichedPosts);
                    } catch (error) {
                        console.error('Error enriching posts:', error);
                        reject(error);
                    }
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

export default {
    createComment,
    updateComment,
    deleteComment,
    getComments,
}