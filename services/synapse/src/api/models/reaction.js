// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

export const createReaction = async (id, publicKey, resourceId, resourceType, reactionType) => {
    return new Promise((resolve, reject) => {
        const sql = `
        INSERT INTO Reactions (reaction_id, public_key, resource_id, resource_type, reaction_type)
        VALUES (?, ?, ?, ?, ?)
    `;

        meNexus.query(sql, [id, publicKey, resourceId, resourceType, reactionType], (error, result) => {
            if (error) {
                console.error(error)
                return reject(new Error('Error creating Reaction'));
            }
            resolve(result.insertId);
        });
    });
}

export const deleteReaction = async (publicKey, resourceId, resourceType, reactionType) => {
    return new Promise((resolve, reject) => {
        const sql = `
            DELETE FROM Reactions
            WHERE public_key = ?
            AND resource_id = ?
            AND resource_type = ?
            AND reaction_type = ?
        `;

        meNexus.query(sql, [publicKey, resourceId, resourceType, reactionType], (error, result) => {
            if (error) {
                console.error(error)
                return reject(new Error('Error deleting Reaction'));
            }
            resolve(result);
        });
    });
}

export const getReactions = async (resourceId) => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT *
            FROM Reactions
            WHERE Reactions.resource_id = ?
        `;

        meNexus.query(sql, [resourceId], (error, result) => {
            if (error) {
                console.error(error)
                return reject(new Error('Error getting Reaction'));
            }
            resolve(result);
        });
    })
}

export default {
    createReaction,
    deleteReaction,
    getReactions
}