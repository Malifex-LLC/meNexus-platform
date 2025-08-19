// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";

export const createActivity = async ({
                                         id,
                                         type,
                                         actorPublicKey,
                                         objectType,
                                         objectId,
                                         targetType = null,
                                         targetId = null,
                                         contextType = null,
                                         contextId = null,
                                         meta = null,
                                         published,
                                     }) => {
    return new Promise((resolve, reject) => {
        const sql = `
      INSERT INTO Activities (
        id, type, actor_public_key, object_type, object_id, target_type, target_id, context_type, context_id, meta, published
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `;

        meNexus.query(
            sql,
            [
                id,
                type,
                actorPublicKey,
                objectType,
                objectId,
                targetType,
                targetId,
                contextType,
                contextId,
                meta,
                published,
            ],
            (err, result) => {
                if (err) {
                    console.error('Error inserting activity:', err);
                    return reject(err);
                }
                resolve(resolve({
                    id,
                    type,
                    actorPublicKey,
                    objectType,
                    objectId,
                    targetType,
                    targetId,
                    contextType,
                    contextId,
                    meta,
                    published
                })
            );
            }
        );
    });
};


export const createPostActivity = async (id, actorPublicKey, objectId, contextType, contextId, published) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO Activities (id, type, actor_public_key, object_type, object_id, context_type, context_id, published)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        `;

        meNexus.query(sql, [id, ACTIVITY_TYPES.POSTED, actorPublicKey, OBJECT_TYPES.POST, objectId, contextType, contextId, published], (err, result) => {
            if (err) {
                console.error(err)
                return reject(err);
            }
            resolve(result);
        });
    });
}

export const getAllActivities = async () => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT * FROM Activities
            ORDER BY published DESC
        `;
        meNexus.query(sql, async (err, results) => {
            if (err) {
                console.error(err);
                return reject(err);
            }
            resolve(results);
        })
    })
}

export default {
    createActivity,
    createPostActivity,
    getAllActivities,
}