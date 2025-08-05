// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from '../config/mysql.js';
import { getUserByPublicKeyFromDB } from "#src/orbitdb/globalUsers.js";

export const getSynapseMembers = async () => {
    return new Promise((resolve, reject) => {
        const sql = `
            SELECT * 
            FROM Members
        `;
        meNexus.query(sql, async (err, results) => {
            if (err) {
                console.error('Database error in getSynapseMembers', err);
                return reject(err);
            }
            try {
                const enrichedMembers = await Promise.all(results.map(async (member) => {
                    const user = await getUserByPublicKeyFromDB(member.public_key);
                    return {
                        ...member,
                        handle: user?.handle || 'Unknown',
                        displayName: user?.displayName || 'Unknown',
                    }
                }));
                resolve(enrichedMembers);
            } catch (error) {
                console.error('Error enriching members', error);
                reject(error);
            }
        });
    });
}

export const addSynapseMember = async (publicKey) => {
    return new Promise((resolve, reject) => {
        const sql = `
            INSERT INTO Members (public_key)
            VALUES (?)
        `
        meNexus.query(sql, publicKey, (err, result) => {
            if (err) {
                console.error('Database error in addSynapseMember', err);
                return reject(err);
            }
            resolve(result);
        })
    })
}

export const removeSynapseMember = async (publicKey) => {
    return new Promise((resolve, reject) => {
        const sql = `
            DELETE FROM Members 
            WHERE public_key = ? 
        `
        meNexus.query(sql, publicKey, (err, result) => {
            if (err) {
                console.error('Database error in removeSynapseMember', err);
                return reject(err);
            }
            resolve(result);
        })
    })
}

export default {
    getSynapseMembers,
    addSynapseMember,
    removeSynapseMember
}